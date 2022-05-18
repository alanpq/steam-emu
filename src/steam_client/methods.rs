use std::{os::raw::c_char, ffi::c_void, ptr};

use tracing::{info, debug, error};

use crate::{HSteamPipe, HSteamUser, steam_api::{SteamUser, SteamFriends}};

use super::{SteamClient, EAccountType};

pub extern "fastcall" fn SteamAPI_ISteamClient_CreateSteamPipe(self_: *mut SteamClient) -> HSteamPipe {
  debug!("SteamAPI_ISteamClient_CreateSteamPipe");
  0
}

pub extern "fastcall" fn SteamAPI_ISteamClient_BReleaseSteamPipe(
  self_: *mut SteamClient,
  hSteamPipe:HSteamPipe
) -> bool {
  debug!("SteamAPI_ISteamClient_BReleaseSteamPipe");
  false
}

pub extern "fastcall" fn SteamAPI_ISteamClient_ConnectToGlobalUser
  (self_: *mut SteamClient, hSteamPipe:HSteamPipe) -> HSteamUser {
  debug!("SteamAPI_ISteamClient_ConnectToGlobalUser");
  0
}

pub extern "fastcall" fn SteamAPI_ISteamClient_CreateLocalUser(
  self_: *mut SteamClient,
  hSteamPipe: *mut HSteamPipe,
  eAccountType:EAccountType
) -> HSteamUser {
  debug!("SteamAPI_ISteamClient_CreateLocalUser");
  0
}

pub extern "fastcall" fn SteamAPI_ISteamClient_ReleaseUser(
  self_: *mut SteamClient,
  hSteamPipe:HSteamPipe,
  hSteamUser:HSteamUser
) {
  debug!("SteamAPI_ISteamClient_ReleaseUser");
}

pub unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamUser(
  self_: *mut SteamClient,
  hSteamUser:HSteamUser, 
  hSteamPipe:HSteamPipe, 
  pchVersion: *const c_char
) ->  *mut SteamUser {
  debug!("GetISteamUser");
  ptr::addr_of_mut!((*self_).steam_user)
}

pub unsafe extern "C" fn SteamAPI_ISteamClient_GetISteamFriends(
  self_: *mut SteamClient, 
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamFriends {
  debug!("GetISteamFriends");
  let p = ptr::addr_of_mut!((*self_).steam_friends);
  debug!(?p);
  ptr::addr_of_mut!((*self_).steam_friends)
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 19] =  [
      SteamAPI_ISteamClient_CreateSteamPipe as _,
      SteamAPI_ISteamClient_BReleaseSteamPipe as _,
      SteamAPI_ISteamClient_ConnectToGlobalUser as _,
      SteamAPI_ISteamClient_CreateLocalUser as _,
      SteamAPI_ISteamClient_ReleaseUser as _,
      SteamAPI_ISteamClient_GetISteamUser as _,
      ptr::null_mut(),
      ptr::null_mut(),
      SteamAPI_ISteamClient_GetISteamFriends as _,
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
    ];
    VTABLE.as_mut_ptr()
  }
}