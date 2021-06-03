#![no_std]

/**
 * This produces a pointer type wrapper for mutable pointers
 */

use core::ops::{Deref, DerefMut};
use core::convert::From;

pub struct Pointer<T> {
    pub ptr: *mut T,
}

impl<T> Pointer<T> {
    pub fn new(data_ptr: &mut T) -> Self {
        Self { ptr: data_ptr as *mut T }
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

impl<T> From<u64> for Pointer<T> {
    fn from(item: u64) -> Self {
        Self { ptr: item as *mut T }
    }
}

impl<T> From<u32> for Pointer<T> {
    fn from(item: u32) -> Self {
        Self { ptr: item as *mut T }
    }
}

impl<T> From<u16> for Pointer<T> {
    fn from(item: u16) -> Self {
        Self { ptr: item as *mut T }
    }
}

impl<T> From<u8> for Pointer<T> {
    fn from(item: u8) -> Self {
        Self { ptr: item as *mut T }
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
