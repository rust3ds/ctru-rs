#![feature(allocator_api)]

use std::f32::consts::PI;

use ctru::linear::LinearAllocator;
use ctru::prelude::*;
use ctru::services::ndsp::{
    wave::{WaveBuffer, WaveInfo},
    AudioFormat, InterpolationType, Ndsp, OutputMode,
};

const SAMPLE_RATE: u32 = 22050;
const SAMPLES_PER_BUF: u32 = SAMPLE_RATE / 30; // 735
const BYTES_PER_SAMPLE: u32 = 4;
const AUDIO_WAVE_LENGTH: usize = (SAMPLES_PER_BUF * BYTES_PER_SAMPLE * 2) as usize;

// Note Frequencies
const NOTEFREQ: [u32; 7] = [220, 440, 880, 1760, 3520, 7040, 14080];

// audioBuffer is stereo PCM16
fn fill_buffer(audioData: &mut [u8], frequency: u32) {
	let formatted_data: Vec<i16> = audioData.chunks_exact(2).map(|s| i16::from_le_bytes(s.try_into().unwrap())).collect();

    for i in 0..audioData.len() {
        // This is a simple sine wave, with a frequency of `frequency` Hz, and an amplitude 30% of maximum.
        let sample: i16 = (0.3 * i16::MAX as f32 * (frequency as f32 * (2f32 * PI) * (i / SAMPLE_RATE as usize) as f32).sin()) as i16;

        // Stereo samples are interleaved: left and right channels.
        formatted_data[i] = (sample << 16) | (sample & 0xffff);
    }
}

fn main() {
    ctru::init();
    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let _console = Console::init(gfx.top_screen.borrow_mut());

    println!("libctru filtered streamed audio\n");

    let audioBuffer = Box::new_in(
        [0u8; AUDIO_WAVE_LENGTH],
        LinearAllocator,
    );
    fill_buffer(&mut audioBuffer, NOTEFREQ[4]);

    let audioBuffer1 =
        WaveBuffer::new(audioBuffer, AudioFormat::PCM16Stereo).expect("Couldn't sync DSP cache");
    let audioBuffer2 = audioBuffer1.clone();

    let fillBlock = false;

    let ndsp = Ndsp::init().expect("Couldn't obtain NDSP controller");

    // This line isn't needed since the default NDSP configuration already sets the output mode to `Stereo`
    ndsp.set_output_mode(OutputMode::Stereo);

    let channel_zero = ndsp.channel(0).unwrap();
    channel_zero.set_interpolation(InterpolationType::Linear);
    channel_zero.set_sample_rate(SAMPLE_RATE as f32);
    channel_zero.set_format(AudioFormat::PCM16Stereo);

    // Output at 100% on the first pair of left and right channels.

    let mix = [0f32; 12];
    mix[0] = 1.0;
    mix[1] = 1.0;
    channel_zero.set_mix(&mix);

    let note: usize = 4;

    // Filters

    let filter_names = [
        "None",
        "Low-Pass",
        "High-Pass",
        "Band-Pass",
        "Notch",
        "Peaking",
    ];

    let filter = 0;

    // We set up two wave buffers and alternate between the two,
    // effectively streaming an infinitely long sine wave.

    let mut buf1 = WaveInfo::new(&mut audioBuffer1, false);
    let mut buf2 = WaveInfo::new(&mut audioBuffer2, false);

    unsafe {
        channel_zero.queue_wave(&mut buf1);
        channel_zero.queue_wave(&mut buf2);
    };

    println!("Press up/down to change tone frequency\n");
    println!("Press left/right to change filter\n");
    println!("\x1b[6;1Hnote = {} Hz        ", NOTEFREQ[note]);
    println!("\x1b[7;1Hfilter = {}         ", filter_names[filter]);

    while apt.main_loop() {
        hid.scan_input();
        let keys_down = hid.keys_down();

        if keys_down.contains(KeyPad::KEY_START) {
            break;
        } // break in order to return to hbmenu

        if keys_down.contains(KeyPad::KEY_DOWN) {
            note = note.saturating_sub(1);
            if note < 0 {
                note = NOTEFREQ.len() - 1;
            }
            println!("\x1b[6;1Hnote = {} Hz        ", NOTEFREQ[note]);
        } else if keys_down.contains(KeyPad::KEY_UP) {
            note += 1;
            if note >= NOTEFREQ.len() {
                note = 0;
            }
            println!("\x1b[6;1Hnote = {} Hz        ", NOTEFREQ[note]);
        }

        // Check for upper limit
        note = std::cmp::max(note, NOTEFREQ.len() - 1);

        let update_params = false;
        if keys_down.contains(KeyPad::KEY_LEFT) {
            filter = filter.saturating_sub(1);
            if filter < 0 {
                filter = filter_names.len() - 1;
            }
            update_params = true;
        } else if keys_down.contains(KeyPad::KEY_LEFT) {
            filter += 1;
            if filter >= filter_names.len() {
                filter = 0;
            }
            update_params = true;
        }

        if update_params {
            println!("\x1b[7;1Hfilter = {}         ", filter_names[filter]);
            match filter {
                1 => channel_zero.iir_biquad_set_params_low_pass_filter(1760., 0.707),
                2 => channel_zero.iir_biquad_set_params_high_pass_filter(1760., 0.707),
                3 => channel_zero.iir_biquad_set_params_band_pass_filter(1760., 0.707),
                4 => channel_zero.iir_biquad_set_params_notch_filter(1760., 0.707),
                5 => channel_zero.iir_biquad_set_params_peaking_equalizer(1760., 0.707, 3.),
                _ => channel_zero.iir_biquad_set_enabled(false),
            }
        }

        if waveBuf[fillBlock].status == NDSP_WBUF_DONE {
            if fillBlock {
                fill_buffer(buf1.get_mut_wavebuffer().get_mut_data(), NOTEFREQ[note]);
                channel_zero.queue_wave(&mut buf1);
            } else {
                fill_buffer(buf2.get_mut_wavebuffer().get_mut_data(), NOTEFREQ[note]);
                channel_zero.queue_wave(&mut buf2);
            }
            fillBlock = !fillBlock;
        }

        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}
