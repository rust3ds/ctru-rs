use ::Result;
use ::c_void;

extern "C" {
    pub fn newsInit() -> Result;
    pub fn newsExit();
    pub fn NEWS_AddNotification(title: *const u16, titleLength: u32,
                                message: *const u16, messageLength: u32,
                                imageData: *const c_void,
                                imageSize: u32, jpeg: u8) -> Result;
}
