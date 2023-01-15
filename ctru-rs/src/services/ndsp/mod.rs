pub mod wave;
use wave::{WaveInfo, WaveStatus};

use crate::error::ResultCode;
use crate::services::ServiceReference;

use std::cell::{RefCell, RefMut};
use std::error;
use std::fmt;
use std::sync::Mutex;

const NUMBER_OF_CHANNELS: u8 = 24;

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
    PCM8Stereo = ctru_sys::NDSP_FORMAT_STEREO_PCM8,
    PCM16Stereo = ctru_sys::NDSP_FORMAT_STEREO_PCM16,
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum InterpolationType {
    Polyphase = ctru_sys::NDSP_INTERP_POLYPHASE,
    Linear = ctru_sys::NDSP_INTERP_LINEAR,
    None = ctru_sys::NDSP_INTERP_NONE,
}

#[derive(Copy, Clone, Debug)]
pub enum NdspError {
    InvalidChannel(u8),      // channel id
    ChannelAlreadyInUse(u8), // channel id
    WaveAlreadyQueued(u8),   // channel id
}

pub struct Channel<'ndsp> {
    id: u8,
    _rf: RefMut<'ndsp, ()>, // we don't need to hold any data
}

static NDSP_ACTIVE: Mutex<usize> = Mutex::new(0);

pub struct Ndsp {
    _service_handler: ServiceReference,
    channel_flags: [RefCell<()>; 24],
}

impl Ndsp {
    pub fn init() -> crate::Result<Self> {
        let _service_handler = ServiceReference::new(
            &NDSP_ACTIVE,
            false,
            || {
                ResultCode(unsafe { ctru_sys::ndspInit() })?;

                Ok(())
            },
            || unsafe {
                ctru_sys::ndspExit();
            },
        )?;

        Ok(Self {
            _service_handler,
            channel_flags: Default::default(),
        })
    }

    /// Return the representation of the specified channel.
    ///
    /// # Errors
    ///
    /// An error will be returned if the channel id is not between 0 and 23.
    pub fn channel(&self, id: u8) -> std::result::Result<Channel, NdspError> {
        let in_bounds = self.channel_flags.get(id as usize);

        match in_bounds {
            Some(ref_cell) => {
                let flag = ref_cell.try_borrow_mut();
                match flag {
                    Ok(_rf) => Ok(Channel { id, _rf }),
                    Err(_) => Err(NdspError::ChannelAlreadyInUse(id)),
                }
            }
            None => Err(NdspError::InvalidChannel(id)),
        }
    }

    /// Set the audio output mode. Defaults to `OutputMode::Stereo`.
    pub fn set_output_mode(&mut self, mode: OutputMode) {
        unsafe { ctru_sys::ndspSetOutputMode(mode as u32) };
    }
}

impl Channel<'_> {
    /// Reset the channel
    pub fn reset(&self) {
        unsafe { ctru_sys::ndspChnReset(self.id.into()) };
    }

    /// Initialize the channel's parameters
    pub fn init_parameters(&self) {
        unsafe { ctru_sys::ndspChnInitParams(self.id.into()) };
    }

    /// Returns whether the channel is playing any audio.
    pub fn is_playing(&self) -> bool {
        unsafe { ctru_sys::ndspChnIsPlaying(self.id.into()) }
    }

    /// Returns whether the channel's playback is currently paused.
    pub fn is_paused(&self) -> bool {
        unsafe { ctru_sys::ndspChnIsPaused(self.id.into()) }
    }

    // Returns the channel's id
    pub fn get_id(&self) -> u8 {
        self.id
    }

    /// Returns the channel's current sample's position.
    pub fn get_sample_position(&self) -> u32 {
        unsafe { ctru_sys::ndspChnGetSamplePos(self.id.into()) }
    }

    /// Returns the channel's current wave sequence's id.
    pub fn get_wave_sequence_id(&self) -> u16 {
        unsafe { ctru_sys::ndspChnGetWaveBufSeq(self.id.into()) }
    }

    /// Pause or un-pause the channel's playback.
    pub fn set_paused(&self, state: bool) {
        unsafe { ctru_sys::ndspChnSetPaused(self.id.into(), state) };
    }

    /// Set the channel's output format.
    /// Change this setting based on the used sample's format.
    pub fn set_format(&self, format: AudioFormat) {
        unsafe { ctru_sys::ndspChnSetFormat(self.id.into(), format as u16) };
    }

    /// Set the channel's interpolation mode.
    pub fn set_interpolation(&self, interp_type: InterpolationType) {
        unsafe { ctru_sys::ndspChnSetInterp(self.id.into(), interp_type as u32) };
    }

    /// Set the channel's volume mix.
    /// Docs about the buffer usage: https://libctru.devkitpro.org/channel_8h.html#a30eb26f1972cc3ec28370263796c0444
    pub fn set_mix(&self, mix: &[f32; 12]) {
        unsafe { ctru_sys::ndspChnSetMix(self.id.into(), mix.as_ptr().cast_mut()) }
    }

    /// Set the channel's rate of sampling.
    pub fn set_sample_rate(&self, rate: f32) {
        unsafe { ctru_sys::ndspChnSetRate(self.id.into(), rate) };
    }

    // `ndspChnSetAdpcmCoefs` isn't wrapped on purpose.
    // DSPADPCM is a proprietary format used by Nintendo, unavailable by "normal" means.
    // We suggest using other wave formats when developing homebrew applications.

    /// Clear the wave buffer queue and stop playback.
    pub fn clear_queue(&self) {
        unsafe { ctru_sys::ndspChnWaveBufClear(self.id.into()) };
    }

    /// Add a wave buffer to the channel's queue.
    /// If there are no other buffers in queue, playback for this buffer will start.
    ///
    /// # Warning
    ///
    /// `libctru` expects the user to manually keep the info data (in this case [WaveInfo]) alive during playback.
    /// To ensure safety, checks within [WaveInfo] will clear the whole channel queue if any queued [WaveInfo] is dropped prematurely.
    pub fn queue_wave(&self, wave: &mut WaveInfo) -> std::result::Result<(), NdspError> {
        match wave.get_status() {
            WaveStatus::Playing | WaveStatus::Queued => {
                return Err(NdspError::WaveAlreadyQueued(self.id))
            }
            _ => (),
        }

        wave.set_channel(self.id);

        unsafe { ctru_sys::ndspChnWaveBufAdd(self.id.into(), &mut wave.raw_data) };

        Ok(())
    }

    // FILTERS

    pub fn iir_mono_set_enabled(&self, enable: bool) {
        unsafe { ctru_sys::ndspChnIirMonoSetEnable(self.id.into(), enable) };
    }

    pub fn iir_mono_set_params_high_pass_filter(&self, cut_off_freq: f32) {
        unsafe { ctru_sys::ndspChnIirMonoSetParamsHighPassFilter(self.id.into(), cut_off_freq) };
    }

    pub fn iir_mono_set_params_low_pass_filter(&self, cut_off_freq: f32) {
        unsafe { ctru_sys::ndspChnIirMonoSetParamsLowPassFilter(self.id.into(), cut_off_freq) };
    }

    pub fn iir_biquad_set_enabled(&self, enable: bool) {
        unsafe { ctru_sys::ndspChnIirBiquadSetEnable(self.id.into(), enable) };
    }

    pub fn iir_biquad_set_params_high_pass_filter(&self, cut_off_freq: f32, quality: f32) {
        unsafe {
            ctru_sys::ndspChnIirBiquadSetParamsHighPassFilter(self.id.into(), cut_off_freq, quality)
        };
    }

    pub fn iir_biquad_set_params_low_pass_filter(&self, cut_off_freq: f32, quality: f32) {
        unsafe {
            ctru_sys::ndspChnIirBiquadSetParamsLowPassFilter(self.id.into(), cut_off_freq, quality)
        };
    }

    pub fn iir_biquad_set_params_notch_filter(&self, notch_freq: f32, quality: f32) {
        unsafe {
            ctru_sys::ndspChnIirBiquadSetParamsNotchFilter(self.id.into(), notch_freq, quality)
        };
    }

    pub fn iir_biquad_set_params_band_pass_filter(&self, mid_freq: f32, quality: f32) {
        unsafe {
            ctru_sys::ndspChnIirBiquadSetParamsBandPassFilter(self.id.into(), mid_freq, quality)
        };
    }

    pub fn iir_biquad_set_params_peaking_equalizer(
        &self,
        central_freq: f32,
        quality: f32,
        gain: f32,
    ) {
        unsafe {
            ctru_sys::ndspChnIirBiquadSetParamsPeakingEqualizer(
                self.id.into(),
                central_freq,
                quality,
                gain,
            )
        };
    }
}

impl AudioFormat {
    /// Returns the amount of bytes needed to store one sample
    /// Eg.
    /// 8 bit formats return 1 (byte)
    /// 16 bit formats return 2 (bytes)
    pub fn sample_size(self) -> u8 {
        match self {
            AudioFormat::PCM8Mono => 1,
            AudioFormat::PCM16Mono | AudioFormat::PCM8Stereo => 2,
            AudioFormat::PCM16Stereo => 4,
        }
    }
}

impl fmt::Display for NdspError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidChannel(id) => write!(f, "Audio Channel with id {id} doesn't exist. Valid channels have an id between 0 and 23."),
            Self::ChannelAlreadyInUse(id) => write!(f, "Audio Channel with id {id} is already being used. Drop the other instance if you want to use it here."),
            Self::WaveAlreadyQueued(id) => write!(f, "The selected WaveInfo is already playing on channel {id}.")
        }
    }
}

impl error::Error for NdspError {
    fn description(&self) -> &str {
        "Error caused within the NDSP service wrapper"
    }
}

impl<'ndsp> Drop for Channel<'ndsp> {
    fn drop(&mut self) {
        self.reset();
    }
}

impl Drop for Ndsp {
    fn drop(&mut self) {
        for i in 0..NUMBER_OF_CHANNELS {
            self.channel(i).unwrap().clear_queue();
        }
    }
}
