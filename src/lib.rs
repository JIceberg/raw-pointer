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

    pub fn as_ptr(&self) -> *mut T {
        self.ptr
    }
}

impl<T> Clone for Pointer<T> {
    fn clone(&self) -> Pointer<T> {
        Pointer::new(self.unwrap_mut())
    }
}

impl<T> Copy for Pointer<T> {}

impl<T> From<usize> for Pointer<T> {
    fn from(item: usize) -> Self {
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
