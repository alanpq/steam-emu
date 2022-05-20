use std::{os::raw::{c_char, c_float}, ffi::c_void, ptr};
use tracing::{info, debug, error};

use vtables::VTable;
use vtables_derive::{VTable, has_vtable};

use crate::uint32;

#[repr(C)]
pub enum EParentalFeature {
	Invalid = 0,
	Store = 1,
	Community = 2,
	Profile = 3,
	Friends = 4,
	News = 5,
	Trading = 6,
	Settings = 7,
	Console = 8,
	Browser = 9,
	ParentalSetup = 10,
	Library = 11,
	Test = 12,
	SiteLicense = 13,
	Max
}

#[has_vtable]
#[derive(VTable, Debug)]
pub struct SteamParentalSettings {
}

impl SteamParentalSettings {
  pub fn new() -> Self {
    Self { vtable: get_vtable() }
  }
}

// FIXME: move this elsewhere
pub type AppId = uint32;

extern "fastcall" fn ReturnFalse(self_: *mut SteamParentalSettings) -> bool {
  false
} 

pub extern "fastcall" fn IsAppBlocked(
  self_: *mut SteamParentalSettings,
  _edx: *mut c_void,
  app_id: AppId,
) -> bool {
  false
}

pub extern "fastcall" fn IsFeatureBlocked(
  self_: *mut SteamParentalSettings,
  _edx: *mut c_void,
  feature: EParentalFeature,
) -> bool {
  false
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 6] = [
      ReturnFalse as _,
      ReturnFalse as _,
      IsAppBlocked as _,
      IsAppBlocked as _, // BIsAppInBlockList
      IsFeatureBlocked as _,
      IsFeatureBlocked as _, // BIsFeatureInBlockList
    ];
    VTABLE.as_mut_ptr()
  }
}