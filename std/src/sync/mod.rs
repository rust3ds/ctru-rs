mod mutex;

pub use self::mutex::{Mutex, MutexGuard};

pub type LockResult<T> = Result<T, ()>;
