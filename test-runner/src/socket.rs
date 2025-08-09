use ctru::prelude::*;

use super::TestRunner;

/// Show test output via a network socket to `3dslink`. This runner is only useful
/// on real hardware, since `3dslink` doesn't work with emulators.
///
/// See [`Soc::redirect_to_3dslink`] for more details.
///
/// [`Soc::redirect_to_3dslink`]: ctru::services::soc::Soc::redirect_to_3dslink
pub struct SocketRunner {
    soc: Soc,
}

impl TestRunner for SocketRunner {
    type Context<'this> = &'this Soc;

    fn new() -> Self {
        let mut soc = Soc::new().expect("failed to initialize network service");
        soc.redirect_to_3dslink(true, true)
            .expect("failed to redirect to socket");
        Self { soc }
    }

    fn setup(&mut self) -> Self::Context<'_> {
        &self.soc
    }
}
