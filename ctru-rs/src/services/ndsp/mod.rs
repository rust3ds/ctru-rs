//! NDSP (Audio) service

pub mod wave;
use wave::{Wave, WaveStatus};

use crate::error::ResultCode;
use crate::services::ServiceReference;

use std::cell::{RefCell, RefMut};
use std::default::Default;
use std::error;
use std::fmt;
use std::sync::Mutex;

const NUMBER_OF_CHANNELS: u8 = 24;

/// Audio output mode.
#[doc(alias = "ndspOutputMode")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum OutputMode {
    /// Single-Channel.
    Mono = ctru_sys::NDSP_OUTPUT_MONO,
    /// Dual-Channel.
    Stereo = ctru_sys::NDSP_OUTPUT_STEREO,
    /// Surround.
    Surround = ctru_sys::NDSP_OUTPUT_SURROUND,
}

/// Audio PCM format.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum AudioFormat {
    /// PCM 8bit single-channel.
    PCM8Mono = ctru_sys::NDSP_FORMAT_MONO_PCM8,
    /// PCM 16bit single-channel.
    PCM16Mono = ctru_sys::NDSP_FORMAT_MONO_PCM16,
    /// PCM 8bit dual-channel.
    PCM8Stereo = ctru_sys::NDSP_FORMAT_STEREO_PCM8,
    /// PCM 16bit dual-channel.
    PCM16Stereo = ctru_sys::NDSP_FORMAT_STEREO_PCM16,
}

/// Representation of volume mix for a channel.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AudioMix {
    raw: [f32; 12],
}

/// Interpolation used between audio frames.
#[doc(alias = "ndspInterpType")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum InterpolationType {
    /// Polyphase interpolation.
    Polyphase = ctru_sys::NDSP_INTERP_POLYPHASE,
    /// Linear interpolation.
    Linear = ctru_sys::NDSP_INTERP_LINEAR,
    /// No interpolation.
    None = ctru_sys::NDSP_INTERP_NONE,
}

/// Error enum returned by NDSP methods.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NdspError {
    /// Channel ID
    InvalidChannel(u8),
    /// Channel ID
    ChannelAlreadyInUse(u8),
    /// Channel ID
    WaveBusy(u8),
    /// Sample amount requested, Max sample amount
    SampleCountOutOfBounds(usize, usize),
}

pub struct Channel<'ndsp> {
    id: u8,
    _rf: RefMut<'ndsp, ()>, // we don't need to hold any data
}

static NDSP_ACTIVE: Mutex<usize> = Mutex::new(0);

/// Handler of the DSP service and DSP processor.
///
/// This is the main struct to handle audio playback using the 3DS' speakers and headphone jack.
/// Only one "instance" of this struct can exist at a time.
pub struct Ndsp {
    _service_handler: ServiceReference,
    channel_flags: [RefCell<()>; NUMBER_OF_CHANNELS as usize],
}

impl Ndsp {
    /// Initialize the DSP service and audio units.
    ///
    /// # Errors
    ///
    /// This function will return an error if an instance of the `Ndsp` struct already exists
    /// or if there are any issues during initialization.
    #[doc(alias = "ndspInit")]
    pub fn new() -> crate::Result<Self> {
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

    /// Return a representation of the specified channel.
    ///
    /// # Errors
    ///
    /// An error will be returned if the channel ID is not between 0 and 23 or if the specified channel is already being used.
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
    #[doc(alias = "ndspSetOutputMode")]
    pub fn set_output_mode(&mut self, mode: OutputMode) {
        unsafe { ctru_sys::ndspSetOutputMode(mode.into()) };
    }
}

impl Channel<'_> {
    /// Reset the channel
    #[doc(alias = "ndspChnReset")]
    pub fn reset(&mut self) {
        unsafe { ctru_sys::ndspChnReset(self.id.into()) };
    }

    /// Initialize the channel's parameters
    #[doc(alias = "ndspChnInitParams")]
    pub fn init_parameters(&self) {
        unsafe { ctru_sys::ndspChnInitParams(self.id.into()) };
    }

    /// Returns whether the channel is playing any audio.
    #[doc(alias = "ndspChnIsPlaying")]
    pub fn is_playing(&self) -> bool {
        unsafe { ctru_sys::ndspChnIsPlaying(self.id.into()) }
    }

    /// Returns whether the channel's playback is currently paused.
    #[doc(alias = "ndspChnIsPaused")]
    pub fn is_paused(&self) -> bool {
        unsafe { ctru_sys::ndspChnIsPaused(self.id.into()) }
    }

    // Returns the channel's id
    pub fn id(&self) -> u8 {
        self.id
    }

    /// Returns the index of the currently played sample.
    ///
    /// Because of how fast this value changes, it should only be used as a rough estimate of the current progress.
    #[doc(alias = "ndspChnGetSamplePos")]
    pub fn sample_position(&self) -> usize {
        (unsafe { ctru_sys::ndspChnGetSamplePos(self.id.into()) }) as usize
    }

    /// Returns the channel's current wave sequence's id.
    #[doc(alias = "ndspChnGetWaveBufSeq")]
    pub fn wave_sequence_id(&self) -> u16 {
        unsafe { ctru_sys::ndspChnGetWaveBufSeq(self.id.into()) }
    }

    /// Pause or un-pause the channel's playback.
    #[doc(alias = "ndspChnSetPaused")]
    pub fn set_paused(&mut self, state: bool) {
        unsafe { ctru_sys::ndspChnSetPaused(self.id.into(), state) };
    }

    /// Set the channel's output format.
    /// Change this setting based on the used sample's format.
    #[doc(alias = "ndspChnSetFormat")]
    pub fn set_format(&mut self, format: AudioFormat) {
        unsafe { ctru_sys::ndspChnSetFormat(self.id.into(), format.into()) };
    }

    /// Set the channel's interpolation mode.
    #[doc(alias = "ndspChnSetInterp")]
    pub fn set_interpolation(&mut self, interp_type: InterpolationType) {
        unsafe { ctru_sys::ndspChnSetInterp(self.id.into(), interp_type.into()) };
    }

    /// Set the channel's volume mix.
    #[doc(alias = "ndspChnSetMix")]
    pub fn set_mix(&mut self, mix: &AudioMix) {
        unsafe { ctru_sys::ndspChnSetMix(self.id.into(), mix.as_raw().as_ptr().cast_mut()) }
    }

    /// Set the channel's rate of sampling.
    #[doc(alias = "ndspChnSetRate")]
    pub fn set_sample_rate(&mut self, rate: f32) {
        unsafe { ctru_sys::ndspChnSetRate(self.id.into(), rate) };
    }

    // `ndspChnSetAdpcmCoefs` isn't wrapped on purpose.
    // DSPADPCM is a proprietary format used by Nintendo, unavailable by "normal" means.
    // We suggest using other wave formats when developing homebrew applications.

    /// Clear the wave buffer queue and stop playback.
    #[doc(alias = "ndspChnWaveBufClear")]
    pub fn clear_queue(&mut self) {
        unsafe { ctru_sys::ndspChnWaveBufClear(self.id.into()) };
    }

    /// Add a wave buffer to the channel's queue.
    /// If there are no other buffers in queue, playback for this buffer will start.
    ///
    /// # Warning
    ///
    /// `libctru` expects the user to manually keep the info data (in this case [`Wave`]) alive during playback.
    /// To ensure safety, checks within [`Wave`] will clear the whole channel queue if any queued [`Wave`] is dropped prematurely.
    #[doc(alias = "ndspChnWaveBufAdd")]
    pub fn queue_wave(&mut self, wave: &mut Wave) -> std::result::Result<(), NdspError> {
        match wave.status() {
            WaveStatus::Playing | WaveStatus::Queued => return Err(NdspError::WaveBusy(self.id)),
            _ => (),
        }

        wave.set_channel(self.id);

        unsafe { ctru_sys::ndspChnWaveBufAdd(self.id.into(), &mut wave.raw_data) };

        Ok(())
    }
}

/// Functions to handle audio filtering.
///
/// Refer to [`libctru`](https://libctru.devkitpro.org/channel_8h.html#a1da3b363c2edfd318c92276b527daae6) for more info.
impl Channel<'_> {
    /// Enables/disables monopole filters.
    #[doc(alias = "ndspChnIirMonoSetEnable")]
    pub fn iir_mono_set_enabled(&mut self, enable: bool) {
        unsafe { ctru_sys::ndspChnIirMonoSetEnable(self.id.into(), enable) };
    }

    /// Sets the monopole to be a high pass filter.
    ///
    /// # Notes
    ///
    /// This is a lower quality filter than the Biquad alternative.
    #[doc(alias = "ndspChnIirMonoSetParamsHighPassFilter")]
    pub fn iir_mono_set_params_high_pass_filter(&mut self, cut_off_freq: f32) {
        unsafe { ctru_sys::ndspChnIirMonoSetParamsHighPassFilter(self.id.into(), cut_off_freq) };
    }

    /// Sets the monopole to be a low pass filter.
    ///
    /// # Notes
    ///
    /// This is a lower quality filter than the Biquad alternative.
    #[doc(alias = "ndspChnIirMonoSetParamsLowPassFilter")]
    pub fn iir_mono_set_params_low_pass_filter(&mut self, cut_off_freq: f32) {
        unsafe { ctru_sys::ndspChnIirMonoSetParamsLowPassFilter(self.id.into(), cut_off_freq) };
    }

    /// Enables/disables biquad filters.
    #[doc(alias = "ndspChnIirBiquadSetEnable")]
    pub fn iir_biquad_set_enabled(&mut self, enable: bool) {
        unsafe { ctru_sys::ndspChnIirBiquadSetEnable(self.id.into(), enable) };
    }

    /// Sets the biquad to be a high pass filter.
    #[doc(alias = "ndspChnIirBiquadSetParamsHighPassFilter")]
    pub fn iir_biquad_set_params_high_pass_filter(&mut self, cut_off_freq: f32, quality: f32) {
        unsafe {
            ctru_sys::ndspChnIirBiquadSetParamsHighPassFilter(self.id.into(), cut_off_freq, quality)
        };
    }

    /// Sets the biquad to be a low pass filter.
    #[doc(alias = "ndspChnIirBiquadSetParamsLowPassFilter")]
    pub fn iir_biquad_set_params_low_pass_filter(&mut self, cut_off_freq: f32, quality: f32) {
        unsafe {
            ctru_sys::ndspChnIirBiquadSetParamsLowPassFilter(self.id.into(), cut_off_freq, quality)
        };
    }

    /// Sets the biquad to be a notch filter.
    #[doc(alias = "ndspChnIirBiquadSetParamsNotchFilter")]
    pub fn iir_biquad_set_params_notch_filter(&mut self, notch_freq: f32, quality: f32) {
        unsafe {
            ctru_sys::ndspChnIirBiquadSetParamsNotchFilter(self.id.into(), notch_freq, quality)
        };
    }

    /// Sets the biquad to be a band pass filter.
    #[doc(alias = "ndspChnIirBiquadSetParamsBandPassFilter")]
    pub fn iir_biquad_set_params_band_pass_filter(&mut self, mid_freq: f32, quality: f32) {
        unsafe {
            ctru_sys::ndspChnIirBiquadSetParamsBandPassFilter(self.id.into(), mid_freq, quality)
        };
    }

    /// Sets the biquad to be a peaking equalizer.
    #[doc(alias = "ndspChnIirBiquadSetParamsPeakingEqualizer")]
    pub fn iir_biquad_set_params_peaking_equalizer(
        &mut self,
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
    ///
    /// Eg.
    /// 8 bit mono formats return 1 (byte)
    /// 16 bit stereo (dual-channel) formats return 4 (bytes)
    pub const fn size(self) -> usize {
        match self {
            Self::PCM8Mono => 1,
            Self::PCM16Mono | Self::PCM8Stereo => 2,
            Self::PCM16Stereo => 4,
        }
    }
}

impl AudioMix {
    /// Creates a new [`AudioMix`] with all volumes set to 0.
    pub fn zeroed() -> Self {
        Self { raw: [0.; 12] }
    }

    /// Returns a reference to the raw data.
    pub fn as_raw(&self) -> &[f32; 12] {
        &self.raw
    }

    /// Returns a mutable reference to the raw data.
    pub fn as_raw_mut(&mut self) -> &mut [f32; 12] {
        &mut self.raw
    }

    /// Returns the values set for the "front" volume mix (left and right channel).
    pub fn front(&self) -> (f32, f32) {
        (self.raw[0], self.raw[1])
    }

    /// Returns the values set for the "back" volume mix (left and right channel).
    pub fn back(&self) -> (f32, f32) {
        (self.raw[2], self.raw[3])
    }

    /// Returns the values set for the "front" volume mix (left and right channel) for the specified auxiliary output device (either 0 or 1).
    pub fn aux_front(&self, id: usize) -> (f32, f32) {
        if id > 1 {
            panic!("invalid auxiliary output device index")
        }

        let index = 4 + id * 4;

        (self.raw[index], self.raw[index + 1])
    }

    /// Returns the values set for the "back" volume mix (left and right channel) for the specified auxiliary output device (either 0 or 1).
    pub fn aux_back(&self, id: usize) -> (f32, f32) {
        if id > 1 {
            panic!("invalid auxiliary output device index")
        }

        let index = 6 + id * 4;

        (self.raw[index], self.raw[index + 1])
    }

    /// Sets the values for the "front" volume mix (left and right channel).
    ///
    /// # Notes
    ///
    /// [`Channel`] will normalize the mix values to be within 0 and 1.
    /// However, an [`AudioMix`] instance with larger/smaller values is valid.
    pub fn set_front(&mut self, left: f32, right: f32) {
        self.raw[0] = left;
        self.raw[1] = right;
    }

    /// Sets the values for the "back" volume mix (left and right channel).
    ///
    /// # Notes
    ///
    /// [`Channel`] will normalize the mix values to be within 0 and 1.
    /// However, an [`AudioMix`] instance with larger/smaller values is valid.
    pub fn set_back(&mut self, left: f32, right: f32) {
        self.raw[2] = left;
        self.raw[3] = right;
    }

    /// Sets the values for the "front" volume mix (left and right channel) for the specified auxiliary output device (either 0 or 1).
    ///
    /// # Notes
    ///
    /// [`Channel`] will normalize the mix values to be within 0 and 1.
    /// However, an [`AudioMix`] instance with larger/smaller values is valid.
    pub fn set_aux_front(&mut self, left: f32, right: f32, id: usize) {
        if id > 1 {
            panic!("invalid auxiliary output device index")
        }

        let index = 4 + id * 4;

        self.raw[index] = left;
        self.raw[index + 1] = right;
    }

    /// Sets the values for the "back" volume mix (left and right channel) for the specified auxiliary output device (either 0 or 1).
    ///
    /// # Notes
    ///
    /// [`Channel`] will normalize the mix values to be within 0 and 1.
    /// However, an [`AudioMix`] instance with larger/smaller values is valid.
    pub fn set_aux_back(&mut self, left: f32, right: f32, id: usize) {
        if id > 1 {
            panic!("invalid auxiliary output device index")
        }

        let index = 6 + id * 4;

        self.raw[index] = left;
        self.raw[index + 1] = right;
    }
}

/// Returns an [`AudioMix`] object with "front left" and "front right" volumes set to 100%, and all other volumes set to 0%.
impl Default for AudioMix {
    fn default() -> Self {
        let mut mix = AudioMix::zeroed();
        mix.set_front(1.0, 1.0);

        mix
    }
}

impl From<[f32; 12]> for AudioMix {
    fn from(value: [f32; 12]) -> Self {
        Self { raw: value }
    }
}

impl fmt::Display for NdspError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidChannel(id) => write!(f, "audio Channel with ID {id} doesn't exist. Valid channels have an ID between 0 and 23"),
            Self::ChannelAlreadyInUse(id) => write!(f, "audio Channel with ID {id} is already being used. Drop the other instance if you want to use it here"),
            Self::WaveBusy(id) => write!(f, "the selected Wave is busy playing on channel {id}"),
            Self::SampleCountOutOfBounds(samples_requested, max_samples) => write!(f, "the sample count requested is too big (requested = {samples_requested}, maximum = {max_samples})"),
        }
    }
}

impl error::Error for NdspError {}

impl Drop for Ndsp {
    #[doc(alias = "ndspExit")]
    fn drop(&mut self) {
        for i in 0..NUMBER_OF_CHANNELS {
            self.channel(i).unwrap().reset();
        }
    }
}

from_impl!(InterpolationType, ctru_sys::ndspInterpType);
from_impl!(OutputMode, ctru_sys::ndspOutputMode);
from_impl!(AudioFormat, u16);
