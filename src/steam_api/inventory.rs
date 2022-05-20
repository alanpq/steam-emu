use std::{os::raw::c_char, ffi::c_void, ptr};
use tracing::{info, debug, error};

use vtables::VTable;
use vtables_derive::{VTable, has_vtable};

use super::SteamAPICall_t;

#[has_vtable]
#[derive(VTable, Debug)]
pub struct SteamInventory {
}

impl SteamInventory {
  pub fn new() -> Self {
    Self { vtable: get_vtable() }
  }
}

pub extern "fastcall" fn RequestPrices(
  self_: *mut SteamInventory,
  _edx: *mut c_void,
) -> SteamAPICall_t {
  0
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 27] = [
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      RequestPrices as _, // RequestPrices
      // ...
    ];
    VTABLE.as_mut_ptr()
  }
}