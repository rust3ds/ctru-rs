#![feature(allocator_api)]
use ctru::linear::LinearAllocator;

use ctru::prelude::*;
use ctru::services::ndsp::{
    wave::{WaveInfo, WaveStatus},
    AudioFormat, Channel, InterpolationType, Ndsp, OutputMode,
};

use lewton::inside_ogg::OggStreamReader;
use lewton::samples::InterleavedSamples;

use std::fs::File;

const CHANNEL_COUNT: usize = 2;
// It should never get this big, but extra space is useful since we can't know the size beforehand
// Read
const AUDIO_WAVE_SIZE: usize = 4000 * CHANNEL_COUNT;

fn setup_next_wave(
    stream_reader: &mut OggStreamReader<File>,
    channel: &Channel,
    wave_info: &mut WaveInfo,
) {
    // Interleaved Dual Channel (Stereo PCM16)
    match stream_reader
        .read_dec_packet_generic::<InterleavedSamples<i16>>()
        .unwrap()
    {
        Some(pck) => {
            // A good way to handle the data would be to allocate it on LINEAR memory directly within `lewton`,
            // but since that API isn't exposed (for its instability) a memcopy is needed either way.

            let mut samples = pck.samples;
            let raw_samples = bytemuck::cast_slice_mut::<_, u8>(samples.as_mut_slice());

            // We need only the first part of the slice to clone the data over
            let buf = wave_info.get_buffer_mut();
            let (buf, _) = buf.split_at_mut(raw_samples.len());

            buf.copy_from_slice(raw_samples);

            // We change the sample_count of the WaveInfo so NDSP won't read the unused bytes
            wave_info
                .set_sample_count((samples.len() / pck.channel_count) as u32)
                .unwrap();

            channel.queue_wave(wave_info).unwrap();
        }
        None => return, // do nothing when the audio ends
    }
}

fn main() {
    ctru::init();
    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let _console = Console::init(gfx.top_screen.borrow_mut());

    cfg_if::cfg_if! {
        if #[cfg(not(all(feature = "romfs", romfs_exists)))] {
            panic!("No RomFS was found, are you sure you included it?")
        }
    }

    let _romfs = ctru::romfs::RomFS::init().unwrap();
    let f = File::open("romfs:/music.ogg").unwrap();
    let mut srr = OggStreamReader::new(f).unwrap();

    println!("\nPress START to exit");
    println!("Sample rate: {}", srr.ident_hdr.audio_sample_rate);

    // If the audio is Mono or it has more channels than 2, it's incompatibile with this current setup
    if srr.ident_hdr.audio_channels != 2 {
        panic!(
            "Audio file improperly modified: Number of channels incompatible with Stereo output"
        );
    }

    // We setup the alternating buffers
    let audio_data1 = Box::new_in([0u8; AUDIO_WAVE_SIZE], LinearAllocator);
    let audio_data2 = audio_data1.clone();
    let mut wave_info1 = WaveInfo::new(audio_data1, AudioFormat::PCM16Stereo, false);
    let mut wave_info2 = WaveInfo::new(audio_data2, AudioFormat::PCM16Stereo, false);

    // NDSP setup
    let mut ndsp = Ndsp::init().expect("Couldn't obtain NDSP controller");
    ndsp.set_output_mode(OutputMode::Stereo);

    let channel_zero = ndsp.channel(0).unwrap();
    channel_zero.set_interpolation(InterpolationType::Linear);
    channel_zero.set_sample_rate(srr.ident_hdr.audio_sample_rate as f32);
    channel_zero.set_format(AudioFormat::PCM16Stereo);

    setup_next_wave(&mut srr, &channel_zero, &mut wave_info1);
    setup_next_wave(&mut srr, &channel_zero, &mut wave_info2);

    // Audio volume mix
    let mut mix: [f32; 12] = [0f32; 12];
    mix[0] = 0.8;
    mix[1] = 0.8;
    channel_zero.set_mix(&mix);

    let mut altern = true; // true is wave_info1, false is wave_info2

    // Main loop
    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }

        let current: &mut WaveInfo = if altern {
            &mut wave_info1
        } else {
            &mut wave_info2
        };

        match current.get_status() {
            WaveStatus::Free | WaveStatus::Done => {
                setup_next_wave(&mut srr, &channel_zero, current);

                altern = !altern;
            }
            _ => (),
        }

        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        // We don't vsync here because that would slow down the sample's decoding.
        // Audio code is supposed to run on a different thread in normal applications.
    }
}
