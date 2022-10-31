use crate::error::ResultCode;
use crate::services::ServiceReference;

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
#[repr(u16)]
pub enum AudioFormat {
    PCM8Mono = ctru_sys::NDSP_FORMAT_MONO_PCM8,
    PCM16Mono = ctru_sys::NDSP_FORMAT_MONO_PCM16,
    ADPCMMono = ctru_sys::NDSP_FORMAT_MONO_ADPCM,
    PCM8Stereo = ctru_sys::NDSP_FORMAT_STEREO_PCM8,
    PCM16Stereo = ctru_sys::NDSP_FORMAT_STEREO_PCM16,
    FrontBypass = ctru_sys::NDSP_FRONT_BYPASS,
    SurroundPreprocessed = ctru_sys::NDSP_3D_SURROUND_PREPROCESSED,
}

pub struct Wave {
    /// Buffer data with specified type
    buffer: Vector<i8>,
    // adpcm_data: AdpcmData, TODO: Requires research on how this WaveBuffer type is handled
    /// Whether to loop the track or not
    looping: bool,
}

#[non_exhaustive]
pub struct Ndsp {
    // To ensure safe tracking of the different channels, there must be only ONE
    // existing instance of the service at a time.
    _service_handler: ServiceReference,
    channel_mask: u32,
}

#[non_exhaustive]
pub struct Channel {
    id: u8, // From 0 to 23.
}

static NDSP_ACTIVE: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

impl Ndsp {
    pub fn init() -> crate::Result<Self> {
        let _service_handler = ServiceReference::new(
            &NDSP_ACTIVE,
            false,
            || unsafe {
                ctru_sys::ndspInit();

                Ok(())
            },
            || unsafe { ctru_sys::ndspExit() },
        )?;

        Ok(Self {
            _service_handler,
            channel_mask: 0,
        })
    }

    /// Return the representation of the specified channel.
    ///
    /// # Errors
    ///
    /// An error will be returned if the channel id is greater than 23.
    pub fn channel(&self, id: u8) -> crate::Result<Channel> {
        if channel > 23 {
            return Err(crate::Error::InvalidChannel);
        }

        Ok(Channel(id as i32))
    }

    /// Set the audio output mode. Defaults to `OutputMode::Stereo`.
    pub fn set_output_mode(&mut self, mode: OutputMode) {
        unsafe { ndspSetOutputMode(mode) };
    }
}

impl Channel {
    /// Reset the channel
    pub fn reset(&mut self) {
        unsafe { ctru_sys::ndspChnReset(self.id) };
    }

    /// Initialize the channel's parameters
    pub fn init_parameters(&mut self) {
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
    pub fn set_paused(&mut self, state: bool) {
        unsafe { ctru_sys::ndspChnSetPaused(self.id, state) };
    }

    /// Set the channel's output format.
    /// Change this setting based on the used sample's format.
    pub fn set_format(&mut self, format: AudioFormat) {
        unsafe { ctru_sys::ndspChnSetFormat(self.id, format) };
    }

    /// Set the channel's interpolation mode.
    pub fn set_interpolation(&mut self, interp_type: InterpolationType) {
        unsafe { ctru_sys::ndspChnSetInterp(self.id, interp_type) };
    }

    /// Set the channel's volume mix.
    /// Docs about the buffer usage: https://libctru.devkitpro.org/channel_8h.html#a30eb26f1972cc3ec28370263796c0444
    pub fn set_mix(&mut self, mix: [f32; 12]) {
        unsafe { ctru_sys::ndspChnSetMix(self.id, mix) }
    }

    /// Set the channel's rate of sampling.
    pub fn set_sample_rate(&mut self, rate: f32) {
        unsafe { ctru_sys::ndspChnSetRate(self.id, rate) };
    }

    // TODO: find a way to wrap `ndspChnSetAdpcmCoefs`

    /// Clear the wave buffer's queue and stop playback.
    pub fn clear_wave_buffer(&mut self) {
        unsafe { ctru_sys::ndspChnWaveBufClear(self.id) };
    }

    /// Add a wave buffer to the channel's queue.
    /// Note: if there are no other buffers in queue, playback for this buffer will start.
    pub fn add_wave_buffer(&mut self, buffer: buf) {
        unsafe { ctru_sys::ndspChnWaveBufAdd(self.id, buffer) };
    }
}

impl Drop for Ndsp {
    fn drop(&mut self) {
        unsafe {
            ctru_sys::ndspExit();
        }
    }
}

impl From<Wave> for ctru_sys::ndspWaveBuf {
    fn from(wave: Wave) -> ctru_sys::ndspWaveBuf {
        let WaveBuffer(vector) = wave.buffer;
        let length = wave.buffer.len();
        let buffer_data = ctru_sys::tag_ndspWaveBuf__bindgen_ty_1 {
            data_vaddr: vector.as_ptr(),
        };

        ctru_sys::ndspWaveBuf {
            __bindgen_anon_1: buffer_data,
            looping: wave.looping,
            next: 0 as *const _,
            sequence_id: 0,
            nsamples: lenght,
            adpcm_data: 0 as *const _,
            offset: 0,
            status: 0,
        }
    }
}
