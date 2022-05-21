use std::{os::raw::c_char, ffi::c_void, ptr};
use tracing::{info, debug, error};

use vtables::VTable;
use vtables_derive::{VTable, has_vtable};

use crate::uint32;

#[has_vtable]
#[derive(VTable, Debug)]
pub struct SteamUtils {
}

impl SteamUtils {
  pub fn new() -> Self {
    Self { vtable: get_vtable() }
  }
}

pub extern "fastcall" fn GetAppID() -> uint32 {
  480
}

pub extern "fastcall" fn IsSteamChinaLauncher() -> bool {
  false
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 31] = [
      ptr::null_mut(), // GetSecondsSinceAppActive
      ptr::null_mut(), // GetSecondsSinceComputerActive
      ptr::null_mut(), // GetConnectedUniverse
      ptr::null_mut(), // GetServerRealTime
      ptr::null_mut(), // GetIPCountry
      ptr::null_mut(), // GetImageSize
      ptr::null_mut(), // GetImageRGBA
      ptr::null_mut(), // GetCSERIPPort?
      ptr::null_mut(), // GetCurrentBatteryPower
      GetAppID as _, // GetAppID
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      IsSteamChinaLauncher as _, // 
    ];
    VTABLE.as_mut_ptr()
  }
}