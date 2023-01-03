pub mod wave;

use crate::error::ResultCode;
use wave::WaveInfo;

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum OutputMode {
    Mono = ctru_sys::NDSP_OUTPUT_MONO,
    Stereo = ctru_sys::NDSP_OUTPUT_STEREO,
    Surround = ctru_sys::NDSP_OUTPUT_SURROUND,
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

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum InterpolationType {
    Polyphase = ctru_sys::NDSP_INTERP_POLYPHASE,
    Linear = ctru_sys::NDSP_INTERP_LINEAR,
    None = ctru_sys::NDSP_INTERP_NONE,
}

pub struct Channel {
    id: i32,
}

#[non_exhaustive]
pub struct Ndsp(());

impl Ndsp {
    pub fn init() -> crate::Result<Self> {
        ResultCode(unsafe { ctru_sys::ndspInit() })?;

        Ok(Self(()))
    }

    /// Return the representation of the specified channel.
    ///
    /// # Errors
    ///
    /// An error will be returned if the channel id is not between 0 and 23.
    pub fn channel(&self, id: u8) -> crate::Result<Channel> {
        if id > 23 {
            return Err(crate::Error::InvalidChannel(id.into()));
        }

        Ok(Channel { id: id.into() })
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
        unsafe { ctru_sys::ndspChnSetMix(self.id, mix.as_ptr().cast_mut()) }
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
    /// # Beware
    ///
    /// `libctru` expects the user to manually keep the info data (in this case [WaveInfo]) alive during playback.
    /// Furthermore, there are no checks to see if the used memory is still valid. All responsibility of handling this data is left to the user.
    pub fn queue_wave(&self, buffer: &mut WaveInfo) {
        unsafe { ctru_sys::ndspChnWaveBufAdd(self.id, &mut buffer.raw_data) };
    }

    // FILTERS

    // TODO: Add Mono filters (and maybe setup the filter functions in a better way)

    pub fn iir_biquad_set_enabled(&self, enable: bool) {
        unsafe { ctru_sys::ndspChnIirBiquadSetEnable(self.id, enable) };
    }

    pub fn iir_biquad_set_params_high_pass_filter(&self, cut_off_freq: f32, quality: f32) {
        unsafe {
            ctru_sys::ndspChnIirBiquadSetParamsHighPassFilter(self.id, cut_off_freq, quality)
        };
    }

    pub fn iir_biquad_set_params_low_pass_filter(&self, cut_off_freq: f32, quality: f32) {
        unsafe { ctru_sys::ndspChnIirBiquadSetParamsLowPassFilter(self.id, cut_off_freq, quality) };
    }

    pub fn iir_biquad_set_params_notch_filter(&self, notch_freq: f32, quality: f32) {
        unsafe { ctru_sys::ndspChnIirBiquadSetParamsNotchFilter(self.id, notch_freq, quality) };
    }

    pub fn iir_biquad_set_params_band_pass_filter(&self, mid_freq: f32, quality: f32) {
        unsafe { ctru_sys::ndspChnIirBiquadSetParamsBandPassFilter(self.id, mid_freq, quality) };
    }

    pub fn iir_biquad_set_params_peaking_equalizer(
        &self,
        central_freq: f32,
        quality: f32,
        gain: f32,
    ) {
        unsafe {
            ctru_sys::ndspChnIirBiquadSetParamsPeakingEqualizer(
                self.id,
                central_freq,
                quality,
                gain,
            )
        };
    }
}

impl Drop for Ndsp {
    fn drop(&mut self) {
        for i in 0..24 {
            self.channel(i).unwrap().clear_queue();
        }

        unsafe {
            ctru_sys::ndspExit();
        }
    }
}
