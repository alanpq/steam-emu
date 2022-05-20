use std::{os::raw::c_char, ffi::c_void, ptr};
use tracing::{info, debug, error};

use vtables::VTable;
use vtables_derive::{VTable, has_vtable};

#[has_vtable]
#[derive(VTable, Debug)]
pub struct SteamRemotePlay {
}

impl SteamRemotePlay {
  pub fn new() -> Self {
    Self { vtable: get_vtable() }
  }
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 4] = [
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      // ...
    ];
    VTABLE.as_mut_ptr()
  }
}