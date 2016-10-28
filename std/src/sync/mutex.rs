use cell::UnsafeCell;
use borrow::{Borrow, BorrowMut};
use ops::{Deref, DerefMut};

use super::LockResult;

use libctru::synchronization::*;

/// A mutex based on libctru's LightLock primitive
pub struct Mutex<T: ?Sized> {
    mutex: Box<LightLock>,
    data: UnsafeCell<T>,
}

/// Mutex guard
#[must_use]
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    inner: &'a Mutex<T>,
}

// NOTE: This is used when implementing condvar, which hasn't been done yet
#[allow(dead_code)]
pub fn guard_lock<'a, T: ?Sized + 'a>(guard: &'a MutexGuard<'a, T>) -> &'a LightLock {
    &guard.inner.mutex
}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Mutex<T> {
        unsafe {
            let mut mutex = Box::new(0);
            LightLock_Init(mutex.borrow_mut());
            Mutex {
                mutex: mutex,
                data: UnsafeCell::new(t),
            }
        }
    }

    pub fn into_inner(self) -> T {
        unsafe { self.data.into_inner() }
    }
}

impl<T: ?Sized> Mutex<T> {
    pub fn lock(&self) -> MutexGuard<T> {
        unsafe {
            LightLock_Lock(self.mutex.borrow());
            MutexGuard { inner: self }
        }
    }

    pub fn try_lock(&self) -> LockResult<MutexGuard<T>> {
        unsafe {
            let locked = LightLock_TryLock(self.mutex.borrow());
            if locked == 0 {
                Ok(MutexGuard { inner: self })
            } else {
                Err(())
            }
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data.get() }
    }
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

impl<'a, T: ?Sized> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        unsafe { LightLock_Unlock(self.inner.mutex.borrow());
        }
    }
}

impl<'mutex, T: ?Sized> Deref for MutexGuard<'mutex, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.inner.data.get() }
    }
}

impl<'mutex, T: ?Sized> DerefMut for MutexGuard<'mutex, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.inner.data.get() }
    }
}

impl<'a, T: ?Sized> !Send for MutexGuard<'a, T> {}
