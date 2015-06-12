use ::Result;
use ::raw::srv;

pub fn init() -> Result {
    unsafe {
        return srv::srvInit();
    }
}
pub fn exit() -> Result {
    unsafe {
        return srv::srvExit();
    }
}
