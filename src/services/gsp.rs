use ::Result;

use ::raw::services::gsp;

pub enum Event {
    Psc0,
    Psc1,
    VBlank0,
    VBlank1,
    PPF,
    P3D,
    DMA
}

fn to_raw_event(ev: Event) -> gsp::GSP_Event {
    use ::raw::services::gsp::GSP_Event::*;
    use self::Event::*;

    match ev {
        Psc0 => GSPEVENT_PSC0,
        Psc1 => GSPEVENT_PSC1,
        VBlank0 => GSPEVENT_VBlank0,
        VBlank1 => GSPEVENT_VBlank1,
        PPF => GSPEVENT_PPF,
        P3D => GSPEVENT_P3D,
        DMA => GSPEVENT_DMA
    }
}

/// Sleep until GSP event fires.
///
/// # Examples
///
/// Wait for VBlank.
///
/// ```
/// use ctru::services::apt;
/// apt::main_loop(|| {
///     wait_for_event(Event::VBlank0);
/// });
pub fn wait_for_event(ev: Event) -> () {
    unsafe {
        // TODO second argument?
        gsp::gspWaitForEvent(to_raw_event(ev), 0);
    }
}
