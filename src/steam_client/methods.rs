use std::{os::raw::c_char, ffi::c_void, ptr};

use tracing::{info, debug, error};

use crate::{HSteamPipe, HSteamUser};

use super::{SteamClient, EAccountType};

pub extern "fastcall" fn SteamAPI_ISteamClient_CreateSteamPipe
  (self_: *mut SteamClient) -> HSteamPipe {
  debug!("SteamAPI_ISteamClient_CreateSteamPipe");
  0
}

pub extern "fastcall" fn SteamAPI_ISteamClient_BReleaseSteamPipe
  (self_: *mut SteamClient, hSteamPipe:HSteamPipe) -> bool {
  debug!("SteamAPI_ISteamClient_BReleaseSteamPipe");
  false
}

pub extern "fastcall" fn SteamAPI_ISteamClient_ConnectToGlobalUser
  (self_: *mut SteamClient, hSteamPipe:HSteamPipe) -> HSteamUser {
  debug!("SteamAPI_ISteamClient_ConnectToGlobalUser");
  0
}

pub extern "fastcall" fn SteamAPI_ISteamClient_CreateLocalUser
  (self_: *mut SteamClient, hSteamPipe: *mut HSteamPipe, eAccountType:EAccountType) -> HSteamUser {
  debug!("SteamAPI_ISteamClient_CreateLocalUser");
  0
}

pub extern "fastcall" fn SteamAPI_ISteamClient_ReleaseUser
  (self_: *mut SteamClient, hSteamPipe:HSteamPipe, hSteamUser:HSteamUser) {
  debug!("SteamAPI_ISteamClient_ReleaseUser");
}

pub unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamUser(self_: *mut SteamClient, hSteamUser:HSteamUser, hSteamPipe:HSteamPipe, pchVersion: *const c_char) ->  *mut c_void {
  debug!("GetISteamUser");
  return (*self_).steam_user
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 6] =  [
      SteamAPI_ISteamClient_CreateSteamPipe as _,
      SteamAPI_ISteamClient_BReleaseSteamPipe as _,
      SteamAPI_ISteamClient_ConnectToGlobalUser as _,
      SteamAPI_ISteamClient_CreateLocalUser as _,
      SteamAPI_ISteamClient_ReleaseUser as _,
      SteamAPI_ISteamClient_GetISteamUser as _,
    ];
    VTABLE.as_mut_ptr()
  }
}