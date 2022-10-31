use ctru::prelude::*;
use ctru::services::ndsp::{Ndsp, OutputMode, InterpolationType};

const SAMPLERATE: u32 = 22050;
const SAMPLESPERBUF: u32 = SAMPLERATE / 30; // 735
const BYTESPERSAMPLE: u32 = 4;

fn array_size(array: &[u8]) -> usize {
	array.len()
} // (sizeof(array)/sizeof(array[0]))

// audioBuffer is stereo PCM16
void fill_buffer(void* audioBuffer, size_t offset, size_t size, int frequency) {
	u32* dest = (u32*) audioBuffer;

	for (int i = 0; i < size; i++) {
		// This is a simple sine wave, with a frequency of `frequency` Hz, and an amplitude 30% of maximum.
		s16 sample = 0.3 * 0x7FFF * sin(frequency * (2 * M_PI) * (offset + i) / SAMPLERATE);

		// Stereo samples are interleaved: left and right channels.
		dest[i] = (sample << 16) | (sample & 0xffff);
	}

	DSP_FlushDataCache(audioBuffer, size);
}

fn main() {
	ctru::init();
    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    let _console = Console::init(gfx.top_screen.borrow_mut());

	println!("libctru filtered streamed audio\n");

	let audioBuffer = [0u32; (SAMPLESPERBUF * BYTESPERSAMPLE * 2)];

	let fillBlock = false;

	let ndsp = Ndsp::init().expect("Couldn't obtain NDSP controller");
	ndsp.set_output_mode(OutputMode::Stereo);

	let channel_zero = ndsp.channel(0);
	channel_zero.set_interpolation(InterpolationType::Linear);
	channel_zero.set_sample_rate(SAMPLERATE);
	channel_zero.set_format(NDSP_FORMAT_STEREO_PCM16);

	// Output at 100% on the first pair of left and right channels.

	let mix = [0f32; 12];
	memset(mix, 0, sizeof(mix));
	mix[0] = 1.0;
	mix[1] = 1.0;
	ndspChnSetMix(0, mix);

	// Note Frequencies

	int notefreq[] = {
		220,
		440, 880, 1760, 3520, 7040,
		14080,
		7040, 3520, 1760, 880, 440
	};

	int note = 4;

	// Filters

	const char* filter_names[] = {
		"None",
		"Low-Pass",
		"High-Pass",
		"Band-Pass",
		"Notch",
		"Peaking"
	};

	int filter = 0;

	// We set up two wave buffers and alternate between the two,
	// effectively streaming an infinitely long sine wave.

	ndspWaveBuf waveBuf[2];
	memset(waveBuf,0,sizeof(waveBuf));
	waveBuf[0].data_vaddr = &audioBuffer[0];
	waveBuf[0].nsamples = SAMPLESPERBUF;
	waveBuf[1].data_vaddr = &audioBuffer[SAMPLESPERBUF];
	waveBuf[1].nsamples = SAMPLESPERBUF;

	size_t stream_offset = 0;

	fill_buffer(audioBuffer,stream_offset, SAMPLESPERBUF * 2, notefreq[note]);

	stream_offset += SAMPLESPERBUF;

	ndspChnWaveBufAdd(0, &waveBuf[0]);
	ndspChnWaveBufAdd(0, &waveBuf[1]);

	println!("Press up/down to change tone frequency\n");
	println!("Press left/right to change filter\n");
	println!("\x1b[6;1Hnote = {} Hz        ", notefreq[note]);
	println!("\x1b[7;1Hfilter = {}         ", filter_names[filter]);

	while(aptMainLoop()) {

		gfxSwapBuffers();
		gfxFlushBuffers();
		gspWaitForVBlank();

		hidScanInput();
		u32 kDown = hidKeysDown();

		if (kDown & KEY_START)
			break; // break in order to return to hbmenu

		if (kDown & KEY_DOWN) {
			note--;
			if (note < 0) {
				note = ARRAY_SIZE(notefreq) - 1;
			}
			println!("\x1b[6;1Hnote = {} Hz        ", notefreq[note]);
		} else if (kDown & KEY_UP) {
			note++;
			if (note >= ARRAY_SIZE(notefreq)) {
				note = 0;
			}
			println!("\x1b[6;1Hnote = {} Hz        ", notefreq[note]);
		}

		bool update_params = false;
		if (kDown & KEY_LEFT) {
			filter--;
			if (filter < 0) {
				filter = ARRAY_SIZE(filter_names) - 1;
			}
			update_params = true;
		} else if (kDown & KEY_RIGHT) {
			filter++;
			if (filter >= ARRAY_SIZE(filter_names)) {
				filter = 0;
			}
			update_params = true;
		}

		if (update_params) {
			println!("\x1b[7;1Hfilter = {}         ", filter_names[filter]);
			switch (filter) {
			default:
				ndspChnIirBiquadSetEnable(0, false);
				break;
			case 1:
				ndspChnIirBiquadSetParamsLowPassFilter(0, 1760.f, 0.707f);
				break;
			case 2:
				ndspChnIirBiquadSetParamsHighPassFilter(0, 1760.f, 0.707f);
				break;
			case 3:
				ndspChnIirBiquadSetParamsBandPassFilter(0, 1760.f, 0.707f);
				break;
			case 4:
				ndspChnIirBiquadSetParamsNotchFilter(0, 1760.f, 0.707f);
				break;
			case 5:
				ndspChnIirBiquadSetParamsPeakingEqualizer(0, 1760.f, 0.707f, 3.0f);
				break;
			}
		}

		if (waveBuf[fillBlock].status == NDSP_WBUF_DONE) {
			fill_buffer(waveBuf[fillBlock].data_pcm16, stream_offset, waveBuf[fillBlock].nsamples, notefreq[note]);
			ndspChnWaveBufAdd(0, &waveBuf[fillBlock]);
			stream_offset += waveBuf[fillBlock].nsamples;
			fillBlock = !fillBlock;
		}
	}

	ndspExit();

	linearFree(audioBuffer);

	gfxExit();
	return 0;
}
