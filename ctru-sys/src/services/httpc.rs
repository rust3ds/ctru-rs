use ::{Handle, Result};

#[repr(C)]
#[derive(Copy)]
pub struct httpcContext {
    pub servhandle: Handle,
    pub httphandle: u32,
}
impl ::core::clone::Clone for httpcContext {
    fn clone(&self) -> Self { *self }
}
impl ::core::default::Default for httpcContext {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum HTTPC_RequestStatus {
    HTTPC_STATUS_REQUEST_IN_PROGRESS = 5,
    HTTPC_STATUS_DOWNLOAD_READY = 7,
}

extern "C" {
    pub fn httpcInit() -> Result;
    pub fn httpcExit();
    pub fn httpcOpenContext(context: *mut httpcContext,
                            url: *mut u8,
                            use_defaultproxy: u32) -> Result;
    pub fn httpcCloseContext(context: *mut httpcContext) -> Result;
    pub fn httpcAddRequestHeaderField(context: *mut httpcContext,
                                      name: *mut u8,
                                      value: *mut u8)
     -> Result;
    pub fn httpcBeginRequest(context: *mut httpcContext) -> Result;
    pub fn httpcReceiveData(context: *mut httpcContext, buffer: *mut u8,
                            size: u32) -> Result;
    pub fn httpcGetRequestState(context: *mut httpcContext,
                                out: *mut HTTPC_RequestStatus) -> Result;
    pub fn httpcGetDownloadSizeState(context: *mut httpcContext,
                                     downloadedsize: *mut u32,
                                     contentsize: *mut u32) -> Result;
    pub fn httpcGetResponseStatusCode(context: *mut httpcContext,
                                      out: *mut u32, delay: u64) -> Result;
    pub fn httpcGetResponseHeader(context: *mut httpcContext,
                                  name: *mut u8,
                                  value: *mut u8,
                                  valuebuf_maxsize: u32) -> Result;
    pub fn httpcDownloadData(context: *mut httpcContext, buffer: *mut u8,
                             size: u32, downloadedsize: *mut u32) -> Result;
    pub fn HTTPC_Initialize(handle: Handle) -> Result;
    pub fn HTTPC_InitializeConnectionSession(handle: Handle,
                                             contextHandle: Handle) -> Result;
    pub fn HTTPC_CreateContext(handle: Handle,
                               url: *mut u8,
                               contextHandle: *mut Handle) -> Result;
    pub fn HTTPC_CloseContext(handle: Handle, contextHandle: Handle)
     -> Result;
    pub fn HTTPC_SetProxyDefault(handle: Handle, contextHandle: Handle)
     -> Result;
    pub fn HTTPC_AddRequestHeaderField(handle: Handle, contextHandle: Handle,
                                       name: *mut u8,
                                       value: *mut u8)
     -> Result;
    pub fn HTTPC_BeginRequest(handle: Handle, contextHandle: Handle)
     -> Result;
    pub fn HTTPC_ReceiveData(handle: Handle, contextHandle: Handle,
                             buffer: *mut u8, size: u32) -> Result;
    pub fn HTTPC_GetRequestState(handle: Handle, contextHandle: Handle,
                                 out: *mut HTTPC_RequestStatus) -> Result;
    pub fn HTTPC_GetDownloadSizeState(handle: Handle, contextHandle: Handle,
                                      downloadedsize: *mut u32,
                                      contentsize: *mut u32) -> Result;
    pub fn HTTPC_GetResponseHeader(handle: Handle, contextHandle: Handle,
                                   name: *mut u8,
                                   value: *mut u8,
                                   valuebuf_maxsize: u32) -> Result;
    pub fn HTTPC_GetResponseStatusCode(handle: Handle, contextHandle: Handle,
                                       out: *mut u32) -> Result;
}
