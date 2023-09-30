use crate::Error;
use std::sync::Mutex;
pub(crate) struct ServiceReference {
    counter: &'static Mutex<usize>,
    close: Box<dyn Fn() + Send + Sync>,
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
        E: Fn() + Send + Sync + 'static,
    {
        let mut value = counter
            .lock()
            .expect("Mutex Counter for ServiceReference is poisoned"); // todo: handle poisoning

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
        let mut value = self
            .counter
            .lock()
            .expect("Mutex Counter for ServiceReference is poisoned"); // todo: handle poisoning
        *value -= 1;
        if *value == 0 {
            (self.close)();
        }
    }
}
