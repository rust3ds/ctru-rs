//! Audio wave representation.

use super::{AudioFormat, NdspError};
use crate::linear::LinearAllocator;

/// Informational struct holding the raw audio data and playback info. This corresponds to [`ctru_sys::ndspWaveBuf`].
pub struct Wave {
    /// Data block of the audio wave (and its format information).
    buffer: Box<[u8], LinearAllocator>,
    audio_format: AudioFormat,
    // Holding the data with the raw format is necessary since `libctru` will access it.
    pub(crate) raw_data: ctru_sys::ndspWaveBuf,
    played_on_channel: Option<u8>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
/// Enum representing the playback status of a [`Wave`].
pub enum WaveStatus {
    /// Wave has never been used.
    Free = ctru_sys::NDSP_WBUF_FREE as u8,
    /// Wave is currently queued for usage.
    Queued = ctru_sys::NDSP_WBUF_QUEUED as u8,
    /// Wave is currently playing.
    Playing = ctru_sys::NDSP_WBUF_PLAYING as u8,
    /// Wave has finished playing.
    Done = ctru_sys::NDSP_WBUF_DONE as u8,
}

impl Wave {
    /// Build a new playable wave object from a raw buffer on LINEAR memory and a some info.
    pub fn new(
        buffer: Box<[u8], LinearAllocator>,
        audio_format: AudioFormat,
        looping: bool,
    ) -> Self {
        let sample_count = buffer.len() / audio_format.size();

        // Signal to the DSP processor the buffer's RAM sector.
        // This step may seem delicate, but testing reports failure most of the time, while still having no repercussions on the resulting audio.
        unsafe {
            let _r = ctru_sys::DSP_FlushDataCache(buffer.as_ptr().cast(), buffer.len() as u32);
        }

        let address = ctru_sys::tag_ndspWaveBuf__bindgen_ty_1 {
            data_vaddr: buffer.as_ptr().cast(),
        };

        let raw_data = ctru_sys::ndspWaveBuf {
            __bindgen_anon_1: address, // Buffer data virtual address
            nsamples: sample_count as u32,
            adpcm_data: std::ptr::null_mut(),
            offset: 0,
            looping,
            // The ones after this point aren't supposed to be setup by the user
            status: 0,
            sequence_id: 0,
            next: std::ptr::null_mut(),
        };

        Self {
            buffer,
            audio_format,
            raw_data,
            played_on_channel: None,
        }
    }

    /// Return a slice to the audio data (on the LINEAR memory).
    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }

    /// Return a mutable slice to the audio data (on the LINEAR memory).
    ///
    /// # Errors
    ///
    /// This function will return an error if the [`Wave`] is currently busy,
    /// with the id to the channel in which it's queued.
    pub fn get_buffer_mut(&mut self) -> Result<&mut [u8], NdspError> {
        match self.status() {
            WaveStatus::Playing | WaveStatus::Queued => {
                Err(NdspError::WaveBusy(self.played_on_channel.unwrap()))
            }
            _ => Ok(&mut self.buffer),
        }
    }

    /// Return this wave's playback status.
    pub fn status(&self) -> WaveStatus {
        self.raw_data.status.try_into().unwrap()
    }

    /// Get the amounts of samples *read* by the NDSP process.
    ///
    /// # Notes
    ///
    /// This value varies depending on [`Wave::set_sample_count`].
    pub fn sample_count(&self) -> usize {
        self.raw_data.nsamples as usize
    }

    /// Get the format of the audio data.
    pub fn format(&self) -> AudioFormat {
        self.audio_format
    }

    // Set the internal flag for the id of the channel playing this wave.
    //
    // Internal Use Only.
    pub(crate) fn set_channel(&mut self, id: u8) {
        self.played_on_channel = Some(id)
    }

    /// Set the amount of samples to be read.
    /// This function doesn't resize the internal buffer.
    ///
    /// # Note
    ///
    /// Operations of this kind are particularly useful to allocate memory pools
    /// for VBR (Variable BitRate) Formats, like OGG Vorbis.
    ///
    /// # Errors
    ///
    /// This function will return an error if the sample size exceeds the buffer's capacity
    /// or if the [`Wave`] is currently queued.
    pub fn set_sample_count(&mut self, sample_count: usize) -> Result<(), NdspError> {
        match self.status() {
            WaveStatus::Playing | WaveStatus::Queued => {
                return Err(NdspError::WaveBusy(self.played_on_channel.unwrap()));
            }
            _ => (),
        }

        let max_count = self.buffer.len() / self.audio_format.size();

        if sample_count > max_count {
            return Err(NdspError::SampleCountOutOfBounds(sample_count, max_count));
        }

        self.raw_data.nsamples = sample_count as u32;

        Ok(())
    }
}

impl TryFrom<u8> for WaveStatus {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Free),
            1 => Ok(Self::Queued),
            2 => Ok(Self::Playing),
            3 => Ok(Self::Done),
            _ => Err("Invalid Wave Status code"),
        }
    }
}

impl Drop for Wave {
    fn drop(&mut self) {
        // This was the only way I found I could check for improper drops of `Wave`.
        // A panic was considered, but it would cause issues with drop order against `Ndsp`.
        match self.status() {
            WaveStatus::Free | WaveStatus::Done => (),
            // If the status flag is "unfinished"
            _ => {
                // The unwrap is safe, since it must have a value in the case the status is "unfinished".
                unsafe { ctru_sys::ndspChnWaveBufClear(self.played_on_channel.unwrap().into()) };
            }
        }

        unsafe {
            // Flag the buffer's RAM sector as unused
            // This step has no real effect in normal applications and is skipped even by devkitPRO's own examples.
            let _r = ctru_sys::DSP_InvalidateDataCache(
                self.buffer.as_ptr().cast(),
                self.buffer.len().try_into().unwrap(),
            );
        }
    }
}
