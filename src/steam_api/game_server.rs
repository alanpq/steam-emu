use std::{os::raw::c_char, ffi::c_void, ptr};
use tracing::{info, debug, error};

use vtables::VTable;
use vtables_derive::{VTable, has_vtable};

use crate::{uint32, uint16};

use super::AppId;

#[has_vtable]
#[derive(VTable, Debug)]
pub struct SteamGameServer {
}

impl SteamGameServer {
  pub fn new() -> Self {
    Self { vtable: get_vtable() }
  }
}

pub extern "fastcall" fn InitGameServer(
  self_: *mut SteamGameServer,
  _edx: *mut c_void,
  ip: uint32,
  game_port: uint16,
  query_port: uint16,
  flags: uint32,
  game_app_id: AppId,
  version_string: *const c_char,
) -> bool {
  info!("InitGameServer");
  true
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 4] = [
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
    ];
    VTABLE.as_mut_ptr()
  }
}