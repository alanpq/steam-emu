
use std::{ptr, ffi::c_void};

use tracing::{info, debug, error};

use vtables::VTable;
use vtables_derive::{has_vtable, VTable};

use crate::steam_api::SteamUser;

#[has_vtable]
#[derive(VTable)]
#[derive(Debug, Copy, Clone)]
// FIXME: change these from *mut c_void
pub struct CSteamAPIContext {
  pub vtable: *mut *mut usize,
}

extern "fastcall" fn stub() {
  error!("stub hit!");
}

extern "fastcall" fn Get_SteamUser() -> *mut SteamUser {
  ptr::null_mut()
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 27] = [
      stub as _,
      Get_SteamUser as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
      stub as _,
    ];
    VTABLE.as_mut_ptr()
  }
}