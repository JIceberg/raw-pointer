#![no_std]

/**
 * This produces a pointer type wrapper for mutable pointers
 */

use core::ops::{Deref, DerefMut};

pub struct Pointer<T> {
    pub ptr: *mut T,
}

impl<T> Pointer<T> {
    pub fn new(mut data: T) -> Self {
        Self { ptr: &mut data as *mut T }
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

impl<T> DerefMut for Pointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}
