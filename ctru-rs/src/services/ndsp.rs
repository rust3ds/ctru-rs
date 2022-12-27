use crate::error::ResultCode;
use crate::linear::LinearAllocator;

use arr_macro::arr;

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum OutputMode {
    Mono = ctru_sys::NDSP_OUTPUT_MONO,
    Stereo = ctru_sys::NDSP_OUTPUT_STEREO,
    Surround = ctru_sys::NDSP_OUTPUT_SURROUND,
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum InterpolationType {
    Polyphase = ctru_sys::NDSP_INTERP_POLYPHASE,
    Linear = ctru_sys::NDSP_INTERP_LINEAR,
    None = ctru_sys::NDSP_INTERP_NONE,
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum AudioFormat {
    PCM8Mono = ctru_sys::NDSP_FORMAT_MONO_PCM8,
    PCM16Mono = ctru_sys::NDSP_FORMAT_MONO_PCM16,
    ADPCMMono = ctru_sys::NDSP_FORMAT_MONO_ADPCM,
    PCM8Stereo = ctru_sys::NDSP_FORMAT_STEREO_PCM8,
    PCM16Stereo = ctru_sys::NDSP_FORMAT_STEREO_PCM16,
    FrontBypass = ctru_sys::NDSP_FRONT_BYPASS,
    SurroundPreprocessed = ctru_sys::NDSP_3D_SURROUND_PREPROCESSED,
}

/// Base struct to represent audio wave data. This requires audio format information.
pub struct WaveBuffer {
    /// Buffer data. This data must be allocated on the LINEAR memory.
    data: Box<[u8], LinearAllocator>,
    audio_format: AudioFormat,
    length: usize,
    // adpcm_data: AdpcmData, TODO: Requires research on how this format is handled.
}

pub struct WaveInfo {
    /// Data block of the audio wave (plus its format information).
    buffer: WaveBuffer,
    // Holding the data with the raw format is necessary since `libctru` will access it.
    raw_data: ctru_sys::ndspWaveBuf,
}

#[non_exhaustive]
pub struct Ndsp(());

impl Ndsp {
    pub fn init() -> crate::Result<Self> {
        ResultCode(unsafe { ctru_sys::ndspInit() })?;

        let mut i = 0;

        Ok(Self(()))
    }

    /// Return the representation of the specified channel.
    ///
    /// # Errors
    ///
    /// An error will be returned if the channel id is not between 0 and 23.
    pub fn channel(&self, id: usize) -> crate::Result<Channel> {
        if id > 23 {
            return Err(crate::Error::InvalidChannel);
        }

        Ok(self.channels[id])
    }

    /// Set the audio output mode. Defaults to `OutputMode::Stereo`.
    pub fn set_output_mode(&mut self, mode: OutputMode) {
        unsafe { ctru_sys::ndspSetOutputMode(mode as u32) };
    }
}

// All channel operations are thread-safe thanks to `libctru`'s use of thread locks.
// As such, there is no need to hold channels to ensure correct mutability.
// With this prospective in mind, this struct looks more like a dummy than an actually functional block.
impl Channel {
    /// Reset the channel
    pub fn reset(&self) {
        unsafe { ctru_sys::ndspChnReset(self.id) };
    }

    /// Initialize the channel's parameters
    pub fn init_parameters(&self) {
        unsafe { ctru_sys::ndspChnInitParams(self.id) };
    }

    /// Returns whether the channel is playing any audio.
    pub fn is_playing(&self) -> bool {
        unsafe { ctru_sys::ndspChnIsPlaying(self.id) }
    }

    /// Returns whether the channel's playback is currently paused.
    pub fn is_paused(&self) -> bool {
        unsafe { ctru_sys::ndspChnIsPaused(self.id) }
    }

    /// Returns the channel's current sample's position.
    pub fn get_sample_position(&self) -> u32 {
        unsafe { ctru_sys::ndspChnGetSamplePos(self.id) }
    }

    /// Returns the channel's current wave sequence's id.
    pub fn get_wave_sequence_id(&self) -> u16 {
        unsafe { ctru_sys::ndspChnGetWaveBufSeq(self.id) }
    }

    /// Pause or un-pause the channel's playback.
    pub fn set_paused(&self, state: bool) {
        unsafe { ctru_sys::ndspChnSetPaused(self.id, state) };
    }

    /// Set the channel's output format.
    /// Change this setting based on the used sample's format.
    pub fn set_format(&self, format: AudioFormat) {
        unsafe { ctru_sys::ndspChnSetFormat(self.id, format as u16) };
    }

    /// Set the channel's interpolation mode.
    pub fn set_interpolation(&self, interp_type: InterpolationType) {
        unsafe { ctru_sys::ndspChnSetInterp(self.id, interp_type as u32) };
    }

    /// Set the channel's volume mix.
    /// Docs about the buffer usage: https://libctru.devkitpro.org/channel_8h.html#a30eb26f1972cc3ec28370263796c0444
    pub fn set_mix(&self, mix: &[f32; 12]) {
        unsafe { ctru_sys::ndspChnSetMix(self.id, mix.as_mut_ptr()) }
    }

    /// Set the channel's rate of sampling.
    pub fn set_sample_rate(&self, rate: f32) {
        unsafe { ctru_sys::ndspChnSetRate(self.id, rate) };
    }

    // TODO: find a way to wrap `ndspChnSetAdpcmCoefs`

    /// Clear the wave buffer queue and stop playback.
    pub fn clear_queue(&self) {
        unsafe { ctru_sys::ndspChnWaveBufClear(self.id) };
    }

    /// Add a wave buffer to the channel's queue.
    /// Note: if there are no other buffers in queue, playback for this buffer will start.
    ///
    /// # Unsafe
    ///
    /// This function is unsafe due to how the buffer is handled internally.
    /// `libctru` expects the user to manually keep the info data (in this case [WaveInfo]) alive during playback.
    /// Furthermore, there are no checks to see if the used memory is still valid. All responsibility of handling this data is left to the user.

    // INTERNAL NOTE: After extensive research to make a Rust checker for these cases,
    // I came to the conclusion that leaving the responsibility to the user is (as of now) the only "good" way to handle this.
    // Sadly `libctru` lacks the infrastructure to make runtime checks on the queued objects, like callbacks and iterators.
    // Also, in most cases the memory is still accessible even after a `free`, so it may not be a problem to the average user.
    // This is still a big "hole" in the Rust wrapper. Upstream changes to `libctru` would be my go-to way to solve this issue.
    pub unsafe fn queue_wave(&self, buffer: WaveInfo) {
        unsafe { ctru_sys::ndspChnWaveBufAdd(self.id, &mut buffer.raw_data) };
    }
}

impl AudioFormat {
    pub fn size(self) -> u8 {
        match self {
            AudioFormat::PCM16Mono | AudioFormat::PCM16Stereo => 2,
            AudioFormat::SurroundPreprocessed => {
                panic!("Can't find size for Sourround Preprocessed audio: format is under research")
            }
            _ => 1,
        }
    }
}

impl WaveBuffer {
    pub fn new(data: Box<[u8], LinearAllocator>, audio_format: AudioFormat) -> Self {
        WaveBuffer {
            data,
            audio_format,
            length: data.len() / format.size(),
        }
    }

    pub fn format(&self) -> AudioFormat {
        self.audio_format
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl WaveInfo {
    pub fn new(buffer: WaveBuffer, looping: bool) -> Self {
        let raw_data = ctru_sys::ndspWaveBuf {
            __bindgen_anon_1: buffer.data.as_ptr(), // Buffer data virtual address
            nsamples: buffer.length,
            adpcm_data: std::ptr::null(),
            offset: 0,
            looping,
            // The ones after this point aren't supposed to be setup by the user
            status: 0,
            sequence_id: 0,
            next: std::ptr::null(),
        };

        Self { buffer, raw_data }
    }
}

impl Drop for Ndsp {
    fn drop(&mut self) {
        unsafe {
            ctru_sys::ndspExit();
        }
    }
}
