use super::AudioFormat;
use crate::error::ResultCode;
use crate::linear::LinearAllocator;

/// Base struct to represent audio wave data. This requires audio format information.
#[derive(Debug, Clone)]
pub struct WaveBuffer {
    /// Buffer data. This data must be allocated on the LINEAR memory.
    data: Box<[u8], LinearAllocator>,
    audio_format: AudioFormat,
    nsamples: usize, // We don't use the slice's length here because depending on the format it may vary
                     // adpcm_data: AdpcmData, TODO: Requires research on how this format is handled.
}

/// Informational struct holding the raw audio data and playaback info. This corresponds to [ctru_sys::ndspWaveBuf]
pub struct WaveInfo<'b> {
    /// Data block of the audio wave (plus its format information).
    buffer: &'b mut WaveBuffer,
    // Holding the data with the raw format is necessary since `libctru` will access it.
    pub(crate) raw_data: ctru_sys::ndspWaveBuf,
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum WaveStatus {
    Free = ctru_sys::NDSP_WBUF_FREE as u8,
    Queued = ctru_sys::NDSP_WBUF_QUEUED as u8,
    Playing = ctru_sys::NDSP_WBUF_PLAYING as u8,
    Done = ctru_sys::NDSP_WBUF_DONE as u8,
}

impl AudioFormat {
    /// Returns the amount of bytes needed to store one sample
    /// Eg.
    /// 8 bit formats return 1 (byte)
    /// 16 bit formats return 2 (bytes)
    pub fn sample_size(self) -> u8 {
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
    pub fn new(data: Box<[u8], LinearAllocator>, audio_format: AudioFormat) -> crate::Result<Self> {
        let nsamples: usize = data.len() / (audio_format.sample_size() as usize);

        unsafe {
            ResultCode(ctru_sys::DSP_FlushDataCache(
                data.as_ptr().cast(),
                data.len().try_into().unwrap(),
            ))?;
        }

        Ok(WaveBuffer {
            data,
            audio_format,
            nsamples,
        })
    }

    pub fn get_mut_data(&mut self) -> &mut Box<[u8], LinearAllocator> {
        &mut self.data
    }

    pub fn get_format(&self) -> AudioFormat {
        self.audio_format
    }

    pub fn get_sample_amount(&self) -> usize {
        self.nsamples
    }
}

impl<'b> WaveInfo<'b> {
    pub fn new(buffer: &'b mut WaveBuffer, looping: bool) -> Self {
        let address = ctru_sys::tag_ndspWaveBuf__bindgen_ty_1 {
            data_vaddr: buffer.data.as_ptr().cast(),
        };

        let raw_data = ctru_sys::ndspWaveBuf {
            __bindgen_anon_1: address, // Buffer data virtual address
            nsamples: buffer.nsamples.try_into().unwrap(),
            adpcm_data: std::ptr::null_mut(),
            offset: 0,
            looping,
            // The ones after this point aren't supposed to be setup by the user
            status: 0,
            sequence_id: 0,
            next: std::ptr::null_mut(),
        };

        Self { buffer, raw_data }
    }

    pub fn get_mut_wavebuffer(&'b mut self) -> &'b mut WaveBuffer {
        &mut self.buffer
    }

    pub fn get_status(&self) -> WaveStatus {
        self.raw_data.status.try_into().unwrap()
    }
}

impl TryFrom<u8> for WaveStatus {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Free),
            1 => Ok(Self::Queued),
            2 => Ok(Self::Playing),
            3 => Ok(Self::Done),
            _ => Err(String::from("Invalid WaveInfo Status code")),
        }
    }
}

impl Drop for WaveBuffer {
    fn drop(&mut self) {
        unsafe {
            // Result can't be used in any way, let's just shrug it off
            let _r = ctru_sys::DSP_InvalidateDataCache(
                self.data.as_ptr().cast(),
                self.data.len().try_into().unwrap(),
            );
        }
    }
}

impl<'b> Drop for WaveInfo<'b> {
    fn drop(&mut self) {
        // This was the only way I found I could check for improper drops of WaveInfos.
        // It should be enough to warn users of the danger.
        match self.get_status() {
            WaveStatus::Free | WaveStatus::Done => (),
            _ => panic!("WaveInfo struct has been dropped before finishing use. This could cause Undefined Behaviour."),
        }
    }
}