use std::{os::raw::c_char, ffi::c_void, ptr::{null, self}};

use crate::{uint32, uint16, HSteamUser};

pub enum EServerMode {
  eServerModeInvalid,
  eServerModeNoAuthentication,
  eServerModeAuthentication,
  eServerModeAuthenticationAndSecure,
}

#[no_mangle]
pub unsafe extern "C" fn SteamInternal_ContextInit(
  pContextInitData: *mut c_void
) -> *mut c_void {
  ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn SteamInternal_FindOrCreateUserInterface(
  hSteamUser: HSteamUser, 
  pszVersion: *const c_char
) -> *mut c_void {
  ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn SteamInternal_FindOrCreateGameServerInterface(
  hSteamUser: HSteamUser, 
  pszVersion: *const c_char
) -> *mut c_void {
  ptr::null_mut()
}


#[no_mangle]
pub extern "C" fn SteamInternal_GameServer_Init(
  unIP: uint32,
  usLegacySteamPort: uint16,
  usGamePort: uint16,
  usQueryPort: uint16,
  eServerMode: EServerMode,
  pchVersionString: *const c_char
) -> bool {
  false
}

#[no_mangle]
pub unsafe extern "C" fn SteamInternal_CreateInterface(
  ver: *const c_char
) -> *mut c_void {
  ptr::null_mut()
}