#![no_std]

/**
 * This produces a pointer type for mutable pointers
 */

use core::ops::Deref;

pub struct Pointer<T> {
    pub ptr: *mut T,
}

impl<T> Pointer<T> {
    pub fn new(ptr: *mut T) -> Self {
        Self { ptr }
    }

    pub fn unwrap(&self) -> &T {
        let val: &T;
        unsafe { val = & *self.ptr }

        val
    }

    pub fn unwrap_mut(&self) -> &mut T {
        let val: &mut T;
        unsafe { val = &mut *self.ptr }

        val
    }
}

impl<T> Deref for Pointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { & *self.ptr }
    }
}
