//! Audio wave.
//!
//! This modules has all methods and structs required to work with audio waves meant to be played via the [`ndsp`](crate::services::ndsp) service.

use super::{AudioFormat, Error};
use crate::linear::LinearAllocation;

/// Informational struct holding the raw audio data and playback info.
///
/// You can play audio [`Wave`]s by using [`Channel::queue_wave()`](super::Channel::queue_wave).
pub struct Wave<Buffer: LinearAllocation + AsRef<[u8]>> {
    /// Data block of the audio wave (and its format information).
    buffer: Buffer,
    audio_format: AudioFormat,
    // Holding the data with the raw format is necessary since `libctru` will access it.
    pub(crate) raw_data: ctru_sys::ndspWaveBuf,
    played_on_channel: Option<u8>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
/// Playback status of a [`Wave`].
pub enum Status {
    /// Wave has never been used.
    Free = ctru_sys::NDSP_WBUF_FREE,
    /// Wave is currently queued for usage.
    Queued = ctru_sys::NDSP_WBUF_QUEUED,
    /// Wave is currently playing.
    Playing = ctru_sys::NDSP_WBUF_PLAYING,
    /// Wave has finished playing.
    Done = ctru_sys::NDSP_WBUF_DONE,
}

impl<Buffer> Wave<Buffer>
where
    Buffer: LinearAllocation + AsRef<[u8]>,
{
    /// Build a new playable wave object from a raw buffer on [LINEAR memory](`crate::linear`) and some info.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # fn main() {
    /// # let _runner = test_runner::GdbRunner::default();
    /// #
    /// use ctru::linear::LinearAllocator;
    /// use ctru::services::ndsp::{AudioFormat, wave::Wave};
    ///
    /// // Zeroed box allocated in the LINEAR memory.
    /// let audio_data: Box<[_], _> = Box::new_in([0u8; 96], LinearAllocator);
    ///
    /// let wave = Wave::new(audio_data, AudioFormat::PCM16Stereo, false);
    /// # }
    /// ```
    pub fn new(buffer: Buffer, audio_format: AudioFormat, looping: bool) -> Self {
        let buf = buffer.as_ref();
        let sample_count = buf.len() / audio_format.size();

        // Signal to the DSP processor the buffer's RAM sector.
        // This step may seem delicate, but testing reports failure most of the time, while still having no repercussions on the resulting audio.
        unsafe {
            let _r = ctru_sys::DSP_FlushDataCache(buf.as_ptr().cast(), buf.len() as u32);
        }

        let address = ctru_sys::tag_ndspWaveBuf__bindgen_ty_1 {
            data_vaddr: buf.as_ptr().cast(),
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

    /// Returns the original data structure used for the audio data.
    pub fn get_raw_buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Returns a mutable reference to the original data structure used for the audio data.
    ///
    /// # Errors
    ///
    /// This function will return an error if the [`Wave`] is currently busy,
    /// with the id to the channel in which it's queued.
    pub fn get_raw_buffer_mut(&mut self) -> Result<&mut Buffer, Error> {
        match self.status() {
            Status::Playing | Status::Queued => {
                Err(Error::WaveBusy(self.played_on_channel.unwrap()))
            }
            _ => Ok(&mut self.buffer),
        }
    }

    /// Returns a slice to the audio data (on the LINEAR memory).
    pub fn get_buffer(&self) -> &[u8] {
        self.get_raw_buffer().as_ref()
    }

    /// Returns a mutable slice to the audio data (on the LINEAR memory).
    ///
    /// # Errors
    ///
    /// This function will return an error if the [`Wave`] is currently busy,
    /// with the id to the channel in which it's queued.
    pub fn get_buffer_mut(&mut self) -> Result<&mut [u8], Error>
    where
        Buffer: AsMut<[u8]>,
    {
        Ok(self.get_raw_buffer_mut()?.as_mut())
    }

    /// Returns this wave's playback status.
    ///
    /// # Example
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # fn main() {
    /// # let _runner = test_runner::GdbRunner::default();
    /// #
    /// # use ctru::linear::LinearAllocator;
    /// # let _audio_data: Box<[_], _> = Box::new_in([0u8; 96], LinearAllocator);
    /// #
    /// use ctru::services::ndsp::{AudioFormat, wave::{Wave, Status}};
    ///
    /// // Provide your own audio data.
    /// let wave = Wave::new(_audio_data, AudioFormat::PCM16Stereo, false);
    ///
    /// // The `Wave` is free if never played before.
    /// assert!(matches!(wave.status(), Status::Free));
    /// # }
    /// ```
    pub fn status(&self) -> Status {
        self.raw_data.status.try_into().unwrap()
    }

    /// Returns the amount of samples *read* by the NDSP process.
    ///
    /// # Notes
    ///
    /// This value varies depending on [`Wave::set_sample_count`].
    pub fn sample_count(&self) -> usize {
        self.raw_data.nsamples as usize
    }

    /// Returns the format of the audio data.
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
    ///
    /// # Note
    ///
    ///
    /// This function doesn't resize the internal buffer. Operations of this kind are particularly useful to allocate memory pools
    /// for VBR (Variable BitRate) formats, like OGG Vorbis.
    ///
    /// # Errors
    ///
    /// This function will return an error if the sample size exceeds the buffer's capacity
    /// or if the [`Wave`] is currently queued.
    pub fn set_sample_count(&mut self, sample_count: usize) -> Result<(), Error> {
        match self.status() {
            Status::Playing | Status::Queued => {
                return Err(Error::WaveBusy(self.played_on_channel.unwrap()));
            }
            _ => (),
        }

        let max_count = self.buffer.as_ref().len() / self.audio_format.size();

        if sample_count > max_count {
            return Err(Error::SampleCountOutOfBounds(sample_count, max_count));
        }

        self.raw_data.nsamples = sample_count as u32;

        Ok(())
    }
}

impl TryFrom<u8> for Status {
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

impl<Buffer> Drop for Wave<Buffer>
where
    Buffer: LinearAllocation + AsRef<[u8]>,
{
    fn drop(&mut self) {
        // This was the only way I found I could check for improper drops of `Wave`.
        // A panic was considered, but it would cause issues with drop order against `Ndsp`.
        match self.status() {
            Status::Free | Status::Done => (),
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
                self.get_buffer().as_ptr().cast(),
                self.get_buffer().len().try_into().unwrap(),
            );
        }
    }
}
