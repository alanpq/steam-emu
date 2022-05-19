#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::{os::raw::c_char, ffi::c_void, ptr};

use tracing::{info, debug, error};

use crate::{HSteamPipe, HSteamUser, steam_api::{SteamUser, SteamFriends, SteamUtils, SteamScreenshots, SteamGameSearch, SteamRemoteStorage, SteamNetworking, SteamApps, SteamGameServerStats, SteamUserStats, SteamMatchmakingServers, SteamMatchmaking}};

use super::{SteamClient, EAccountType};

#[allow(non_camel_case_types)]

extern "fastcall" fn SteamAPI_ISteamClient_CreateSteamPipe(self_: *mut SteamClient) -> HSteamPipe {
  debug!("SteamAPI_ISteamClient_CreateSteamPipe");
  0
}

extern "fastcall" fn SteamAPI_ISteamClient_BReleaseSteamPipe(
  self_: *mut SteamClient,
  hSteamPipe:HSteamPipe
) -> bool {
  debug!("SteamAPI_ISteamClient_BReleaseSteamPipe");
  false
}

extern "fastcall" fn SteamAPI_ISteamClient_ConnectToGlobalUser
  (self_: *mut SteamClient, hSteamPipe:HSteamPipe) -> HSteamUser {
  debug!("SteamAPI_ISteamClient_ConnectToGlobalUser");
  0
}

extern "fastcall" fn SteamAPI_ISteamClient_CreateLocalUser(
  self_: *mut SteamClient,
  hSteamPipe: *mut HSteamPipe,
  eAccountType:EAccountType
) -> HSteamUser {
  debug!("SteamAPI_ISteamClient_CreateLocalUser");
  0
}

extern "fastcall" fn SteamAPI_ISteamClient_ReleaseUser(
  self_: *mut SteamClient,
  hSteamPipe:HSteamPipe,
  hSteamUser:HSteamUser
) {
  debug!("SteamAPI_ISteamClient_ReleaseUser");
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamUser(
  self_: *mut SteamClient,
  hSteamUser:HSteamUser, 
  hSteamPipe:HSteamPipe, 
  pchVersion: *const c_char
) ->  *mut SteamUser {
  debug!("GetISteamUser");
  ptr::addr_of_mut!((*self_).steam_user)
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamFriends(
  self_: *mut SteamClient, 
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamFriends {
  let p = ptr::addr_of_mut!((*self_).steam_friends);
  debug!("GetISteamFriends -> {:?}", p);
  ptr::addr_of_mut!((*self_).steam_friends)
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamUtils(
  self_: *mut SteamClient, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamUtils {
  debug!("GetISteamUtils -> 0x0");
  ptr::null_mut()
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamMatchmaking(
  self_: *mut SteamClient, 
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamMatchmaking {
  debug!("GetISteamMatchmaking -> 0x0");
  ptr::null_mut()
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamMatchmakingServers(
  self_: *mut SteamClient, 
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamMatchmakingServers {
  debug!("GetISteamMatchmakingServers -> 0x0");
  ptr::null_mut()
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamGenericInterface(
  self_: *mut SteamClient, 
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut c_void {
  debug!("GetISteamGenericInterface -> 0x0");
  ptr::null_mut()
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamUserStats (
  self_: *mut SteamClient, 
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamUserStats {
  debug!("GetISteamUserStats -> 0x0");
  ptr::null_mut()
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamGameServerStats (
  self_: *mut SteamClient, 
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamGameServerStats {
  debug!("GetISteamGameServerStats -> 0x0");
  ptr::null_mut()
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamApps (
  self_: *mut SteamClient, 
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamApps {
  debug!("GetISteamApps -> 0x0");
  ptr::null_mut()
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamNetworking (
  self_: *mut SteamClient, 
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamNetworking {
  debug!("GetISteamNetworking -> 0x0");
  ptr::null_mut()
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamRemoteStorage (
  self_: *mut SteamClient, 
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamRemoteStorage {
  debug!("GetISteamRemoteStorage -> 0x0");
  ptr::null_mut()
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamScreenshots (
  self_: *mut SteamClient, 
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamScreenshots {
  debug!("GetISteamScreenshots -> 0x0");
  ptr::null_mut()
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamGameSearch (
  self_: *mut SteamClient, 
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamGameSearch {
  debug!("GetISteamGameSearch -> 0x0");
  ptr::null_mut()
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 20] =  [
      SteamAPI_ISteamClient_CreateSteamPipe as _,
      SteamAPI_ISteamClient_BReleaseSteamPipe as _,
      SteamAPI_ISteamClient_ConnectToGlobalUser as _,
      SteamAPI_ISteamClient_CreateLocalUser as _,
      SteamAPI_ISteamClient_ReleaseUser as _,
      SteamAPI_ISteamClient_GetISteamUser as _,
      ptr::null_mut(),
      ptr::null_mut(),
      SteamAPI_ISteamClient_GetISteamFriends as _,
      SteamAPI_ISteamClient_GetISteamUtils as _,
      SteamAPI_ISteamClient_GetISteamMatchmaking as _,
      SteamAPI_ISteamClient_GetISteamMatchmakingServers as _,
      SteamAPI_ISteamClient_GetISteamGenericInterface as _,
      SteamAPI_ISteamClient_GetISteamUserStats as _,
      SteamAPI_ISteamClient_GetISteamGameServerStats as _,
      SteamAPI_ISteamClient_GetISteamApps as _,
      SteamAPI_ISteamClient_GetISteamNetworking as _,
      SteamAPI_ISteamClient_GetISteamRemoteStorage as _,
      SteamAPI_ISteamClient_GetISteamScreenshots as _,
      SteamAPI_ISteamClient_GetISteamGameSearch as _,
    ];
    VTABLE.as_mut_ptr()
  }
}