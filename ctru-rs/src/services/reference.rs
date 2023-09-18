use crate::Error;
use std::sync::{Mutex, MutexGuard, TryLockError};

pub(crate) struct ServiceReference {
    _guard: MutexGuard<'static, ()>,
    close: Box<dyn Fn() + Send + Sync>,
}

impl ServiceReference {
    pub fn new<S, E>(counter: &'static Mutex<()>, start: S, close: E) -> crate::Result<Self>
    where
        S: FnOnce() -> crate::Result<()>,
        E: Fn() + Send + Sync + 'static,
    {
        let _guard = match counter.try_lock() {
            Ok(lock) => lock,
            Err(e) => match e {
                TryLockError::Poisoned(guard) => {
                    // If the MutexGuard is poisoned that means that the "other" service instance (of which the thread panicked)
                    // was NOT properly closed. To avoid any weird behaviour, we try closing the service now, to then re-open a fresh instance.
                    //
                    // It's up to our `close()` implementations to avoid panicking/doing weird stuff again.
                    close();

                    guard.into_inner()
                }
                TryLockError::WouldBlock => return Err(Error::ServiceAlreadyActive),
            },
        };

        start()?;

        Ok(Self {
            _guard,
            close: Box::new(close),
        })
    }
}

impl Drop for ServiceReference {
    fn drop(&mut self) {
        (self.close)();
    }
}
