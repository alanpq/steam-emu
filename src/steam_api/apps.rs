use std::{os::raw::c_char, ffi::c_void, ptr};
use tracing::{info, debug, error};

use vtables::VTable;
use vtables_derive::{VTable, has_vtable};

use crate::int32;

#[has_vtable]
#[derive(VTable, Debug)]
pub struct SteamApps {
}

impl SteamApps {
  pub fn new() -> Self {
    Self { vtable: get_vtable() }
  }
}

pub extern "fastcall" fn SteamAPI_ISteamApps_GetLaunchCommandLine(
  self_: *mut SteamApps,
  _edx: *mut c_void,
  pszCommandLine: *mut c_char,
  cubCommandLine: int32,
) -> int32 {
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
      SteamAPI_ISteamApps_GetLaunchCommandLine as _,
    ];
    VTABLE.as_mut_ptr()
  }
}