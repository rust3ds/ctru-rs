use ::{Result, Handle};

#[link(name = "ctru")]
extern "C" {
    pub fn MIC_Initialize(sharedmem: *mut u32, sharedmem_size: u32,  control: u8, recording: u8, unk0: u8, unk1: u8, unk2: u8) -> Result;
    pub fn MIC_Shutdown() -> Result;
    pub fn MIC_GetSharedMemOffsetValue() -> u32;
    pub fn MIC_ReadAudioData(outbuf: *mut u8, readsize: u32, waitforevent: u32) -> u32;
    pub fn MIC_MapSharedMem(handle: Handle, size: u32) -> Result;
    pub fn MIC_UnmapSharedMem() -> Result;
    pub fn MIC_cmd3_Initialize(unk0: u8, unk1: u8, sharedmem_baseoffset: u32, sharedmem_endoffset: u32, unk2: u8) -> Result;
    pub fn MIC_cmd5() -> Result;
    pub fn MIC_GetCNTBit15(out: *mut u8) -> Result;
    pub fn MIC_GetEventHandle(handle: *mut Handle) -> Result;
    pub fn MIC_SetControl(value: u8) -> Result;
    pub fn MIC_GetControl(value: *mut u8) -> Result;
    pub fn MIC_SetRecording(value: u8) -> Result;
    pub fn MIC_IsRecoding(value: *mut u8) -> Result;
}
