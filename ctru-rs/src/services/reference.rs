use crate::Error;
use std::sync::Mutex;
pub(crate) struct ServiceReference {
    counter: &'static Mutex<usize>,
    close: Box<dyn Fn()>,
}

impl ServiceReference {
    pub fn new<S, E>(
        counter: &'static Mutex<usize>,
        allow_multiple: bool,
        start: S,
        close: E,
    ) -> crate::Result<Self>
    where
        S: FnOnce() -> crate::Result<()>,
        E: Fn() + 'static,
    {
        let mut value = counter.lock().unwrap(); // todo: handle poisoning

        if *value == 0 {
            start()?;
        } else if !allow_multiple {
            return Err(Error::ServiceAlreadyActive);
        }

        *value += 1;

        Ok(Self {
            counter,
            close: Box::new(close),
        })
    }
}

impl Drop for ServiceReference {
    fn drop(&mut self) {
        let mut value = self.counter.lock().unwrap(); // should probably handle poisoning - could just map_err to ignore it.
        *value -= 1;
        if *value == 0 {
            (self.close)();
        }
    }
}
