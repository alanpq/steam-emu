use std::{os::raw::c_char, ffi::c_void, ptr};
use tracing::{info, debug, error};

use vtables::VTable;
use vtables_derive::{VTable, has_vtable};

use crate::int32;

use super::{SteamListenSocket, SteamNetPollGroup};

#[has_vtable]
#[derive(VTable, Debug)]
pub struct SteamNetworkingSockets {
}

impl SteamNetworkingSockets {
  pub fn new() -> Self {
    Self { vtable: get_vtable() }
  }
}

pub extern "fastcall" fn CreateListenSocketP2P(
  self_: *mut SteamNetworkingSockets,
  _edx: *mut c_void,
  local_virtual_port: int32,
  no_options: int32,
  options: *mut c_void,
) -> SteamListenSocket {
  0 // FIXME: implement
}

pub extern "fastcall" fn CreatePollGroup(
  self_: *mut SteamNetworkingSockets,
  _edx: *mut c_void,
) -> SteamNetPollGroup {
  0 // FIXME: implement
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 26] = [
      ptr::null_mut(), // CreateListenSocketIP
      ptr::null_mut(), // ConnectByIPAddress
      CreateListenSocketP2P as _, // CreateListenSocketP2P
      ptr::null_mut(), // ConnectP2P
      ptr::null_mut(), // AcceptConnection
      ptr::null_mut(), // CloseConnection
      ptr::null_mut(), // CloseListenSocket
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
      CreatePollGroup as _, // CreatePollGroup
      ptr::null_mut(), // DestroyPollGroup
      // ...
    ];
    VTABLE.as_mut_ptr()
  }
}