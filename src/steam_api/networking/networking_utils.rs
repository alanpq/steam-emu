use std::{os::raw::{c_char, c_float}, ffi::c_void, ptr};
use tracing::{info, debug, error};

use vtables::VTable;
use vtables_derive::{VTable, has_vtable};

#[has_vtable]
#[derive(VTable, Debug)]
pub struct SteamNetworkingUtils {
}

impl SteamNetworkingUtils {
  pub fn new() -> Self {
    Self { vtable: get_vtable() }
  }
}

pub unsafe extern "fastcall" fn CheckPingDataUpToDate(
  self_: *mut SteamNetworkingUtils,
  _edx: *mut c_void,
  max_age_seconds: c_float,
) -> bool {
  debug!("CheckPingDataUpToDate");
  true
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 9] = [
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      CheckPingDataUpToDate as _,
      ptr::null_mut(),
    ];
    VTABLE.as_mut_ptr()
  }
}