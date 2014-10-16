#![feature(macro_rules)]
#![allow(unused_variable)]
#![allow(dead_code)]

extern crate boehm_bindings;

use std::{fmt, mem, ptr};

pub struct Boehm<T> {
    ptr : *mut T
}

impl<T> Boehm<T> {
  pub fn new(data : T) -> Boehm<T> {
    let mem_sz = mem::size_of::<T>() as u64;
    let ptr = unsafe { boehm_bindings::GC_malloc(mem_sz) as *mut T };

    if ptr.is_null() {
        fail!("Allocation Failure: Boehm::new() could not allocate enough memory")
    }

    unsafe { ptr::write(ptr, data) };

    Boehm { ptr : ptr }
  }
}

impl<T> Deref<T> for Boehm<T> {
  fn deref<'a>(&'a self) -> &'a T {
    unsafe{ &*self.ptr }
  }
}

impl<T> DerefMut<T> for Boehm<T> {
  fn deref_mut<'a>(&'a mut self) -> &'a mut T {
    unsafe{ &mut *self.ptr }
  }
}

impl<T: 'static + fmt::Show> fmt::Show for Boehm<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    unsafe{ (*self.ptr).fmt(f) }
  }
}

pub fn init() {
  unsafe { boehm_bindings::GC_init() }
}

pub fn free_bytes() -> u64 {
  unsafe { boehm_bindings::GC_get_free_bytes() }
}