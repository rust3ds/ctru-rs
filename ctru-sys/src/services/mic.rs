use ::{Handle, Result};

#[derive(Clone, Copy)]
#[repr(C)]
pub enum MICU_Encoding {
    MICU_ENCODING_PCM8 = 0,
    MICU_ENCODING_PCM16 = 1,
    MICU_ENCODING_PCM8_SIGNED = 2,
    MICU_ENCODING_PCM16_SIGNED = 3,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum MICU_SampleRate {
    MICU_SAMPLE_RATE_32730 = 0,
    MICU_SAMPLE_RATE_16360 = 1,
    MICU_SAMPLE_RATE_10910 = 2,
    MICU_SAMPLE_RATE_8180 = 3,
}

extern "C" {
    pub fn micInit(buffer: *mut u8, bufferSize: u32) -> Result;
    pub fn micExit();
    pub fn micGetSampleDataSize() -> u32;
    pub fn micGetLastSampleOffset() -> u32;
    pub fn MICU_MapSharedMem(size: u32, handle: Handle) -> Result;
    pub fn MICU_UnmapSharedMem() -> Result;
    pub fn MICU_StartSampling(encoding: MICU_Encoding,
                              sampleRate: MICU_SampleRate, offset: u32,
                              size: u32, _loop: u8) -> Result;
    pub fn MICU_AdjustSampling(sampleRate: MICU_SampleRate) -> Result;
    pub fn MICU_StopSampling() -> Result;
    pub fn MICU_IsSampling(sampling: *mut u8) -> Result;
    pub fn MICU_GetEventHandle(handle: *mut Handle) -> Result;
    pub fn MICU_SetGain(gain: u8) -> Result;
    pub fn MICU_GetGain(gain: *mut u8) -> Result;
    pub fn MICU_SetPower(power: u8) -> Result;
    pub fn MICU_GetPower(power: *mut u8) -> Result;
    pub fn MICU_SetClamp(clamp: u8) -> Result;
    pub fn MICU_GetClamp(clamp: *mut u8) -> Result;
    pub fn MICU_SetAllowShellClosed(allowShellClosed: u8) -> Result;
}
