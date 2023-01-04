#![feature(allocator_api)]

use std::f32::consts::PI;

use ctru::linear::LinearAllocator;
use ctru::prelude::*;
use ctru::services::ndsp::{
    wave::{WaveBuffer, WaveInfo, WaveStatus},
    AudioFormat, InterpolationType, Ndsp, OutputMode,
};

const SAMPLE_RATE: u32 = 22050;
const SAMPLES_PER_BUF: u32 = SAMPLE_RATE / 30; // 735
const BYTES_PER_SAMPLE: u32 = 4;
const AUDIO_WAVE_LENGTH: usize = (SAMPLES_PER_BUF * BYTES_PER_SAMPLE * 2) as usize;

// Note Frequencies
const NOTEFREQ: [f32; 7] = [220., 440., 880., 1760., 3520., 7040., 14080.];

// audioBuffer is stereo PCM16
fn fill_buffer(audio_data: &mut [u8], frequency: f32) {
    let formatted_data = audio_data.chunks_exact_mut(2);

    let mut i = 0.;
    for chunk in formatted_data {
        // This is a simple sine wave, with a frequency of `frequency` Hz, and an amplitude 30% of maximum.
        let sample: f32 = (frequency * (i / SAMPLE_RATE as f32) * 2. * PI).sin();
        let amplitude = 0.3 * i16::MAX as f32;

        // This operation is safe, since we are writing to a slice of exactly 16 bits
        let chunk_ptr: &mut i16 = unsafe { std::mem::transmute(chunk.as_mut_ptr()) };
        // Stereo samples are interleaved: left and right channels.
        *chunk_ptr = (sample * amplitude) as i16;

        i += 1.;
    }
}

fn main() {
    ctru::init();
    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let _console = Console::init(gfx.top_screen.borrow_mut());

    println!("libctru filtered streamed audio\n");

    let mut ndsp = Ndsp::init().expect("Couldn't obtain NDSP controller");

    // This line isn't needed since the default NDSP configuration already sets the output mode to `Stereo`
    ndsp.set_output_mode(OutputMode::Stereo);

    let channel_zero = ndsp.channel(0).unwrap();
    channel_zero.set_interpolation(InterpolationType::Linear);
    channel_zero.set_sample_rate(SAMPLE_RATE as f32);
    channel_zero.set_format(AudioFormat::PCM16Stereo);

    // Output at 100% on the first pair of left and right channels.

    let mut mix: [f32; 12] = [0f32; 12];
    mix[0] = 1.0;
    mix[1] = 1.0;
    channel_zero.set_mix(&mix);

    let mut note: usize = 4;

    // Filters

    let filter_names = [
        "None",
        "Low-Pass",
        "High-Pass",
        "Band-Pass",
        "Notch",
        "Peaking",
    ];

    let mut filter = 0;

    // We set up two wave buffers and alternate between the two,
    // effectively streaming an infinitely long sine wave.

	let mut audio_data1 = Box::new_in([0u8; AUDIO_WAVE_LENGTH], LinearAllocator);
    fill_buffer(&mut audio_data1[..], NOTEFREQ[4]);
	let mut audio_data2 = audio_data1.clone();

    let mut audio_buffer1 = WaveBuffer::new(audio_data1, AudioFormat::PCM16Stereo).expect("Couldn't sync DSP cache");
    let mut audio_buffer2 = WaveBuffer::new(audio_data2, AudioFormat::PCM16Stereo).expect("Couldn't sync DSP cache");

    let mut wave_info1 = WaveInfo::new(&mut audio_buffer1, false);
    let mut wave_info2 = WaveInfo::new(&mut audio_buffer2, false);

    channel_zero.queue_wave(&mut wave_info1);
    channel_zero.queue_wave(&mut wave_info2);

    println!("Press up/down to change tone frequency\n");
    println!("Press left/right to change filter\n");
    println!("\x1b[6;1Hnote = {} Hz        ", NOTEFREQ[note]);
    println!("\x1b[7;1Hfilter = {}         ", filter_names[filter]);

    let mut altern = true; // true is wave_info1, false is wave_info2

    while apt.main_loop() {
        hid.scan_input();
        let keys_down = hid.keys_down();

        if keys_down.contains(KeyPad::KEY_START) {
            break;
        } // break in order to return to hbmenu

        if keys_down.intersects(KeyPad::KEY_DOWN) {
            note = note.saturating_sub(1);
        } else if keys_down.intersects(KeyPad::KEY_UP) {
            note += 1;
        }

        let mut update_params = false;
        if keys_down.intersects(KeyPad::KEY_LEFT) {
            let wraps;
            (filter, wraps) = filter.overflowing_sub(1);

            if wraps {
                filter = filter_names.len() - 1;
            }

            update_params = true;
        } else if keys_down.intersects(KeyPad::KEY_RIGHT) {
            filter += 1;
            if filter >= filter_names.len() {
                filter = 0;
            }
            update_params = true;
        }

        // Check for upper limit
        note = std::cmp::min(note, NOTEFREQ.len() - 1);

        println!("\x1b[6;1Hnote = {} Hz        ", NOTEFREQ[note]);
        println!("\x1b[7;1Hfilter = {}         ", filter_names[filter]);

        if update_params {
            match filter {
                1 => channel_zero.iir_biquad_set_params_low_pass_filter(1760., 0.707),
                2 => channel_zero.iir_biquad_set_params_high_pass_filter(1760., 0.707),
                3 => channel_zero.iir_biquad_set_params_band_pass_filter(1760., 0.707),
                4 => channel_zero.iir_biquad_set_params_notch_filter(1760., 0.707),
                5 => channel_zero.iir_biquad_set_params_peaking_equalizer(1760., 0.707, 3.),
                _ => channel_zero.iir_biquad_set_enabled(false),
            }
        }

        let current: &mut WaveInfo;

        if altern {
            current = &mut wave_info1;
        } else {
            current = &mut wave_info2;
        }

        let status = current.get_status();
        if let WaveStatus::Done = status {
            fill_buffer(current.get_mut_wavebuffer().get_mut_data(), NOTEFREQ[note]);
            channel_zero.queue_wave(current);

            altern = !altern;
        }

        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}
