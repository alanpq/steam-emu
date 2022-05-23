use std::ffi::CStr;
use std::{os::raw::c_char, ffi::c_void, ptr};

use tracing::debug;

use crate::steam_client::*;
use crate::{HSteamUser, HSteamPipe};

use super::SteamClient;
use super::SteamAPI_ISteamClient_GetISteamUser;


#[no_mangle]
pub unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamGenericInterface(
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut c_void {
  let version = CStr::from_ptr(pchVersion).to_str().unwrap();
  if self_.is_null() {
    error!("null reference to self!");
  }
  let self_s = &mut *self_;

  debug!("GetISteamGenericInterface '{:?}'", version);
  // debug!(?self_);
  // debug!(hSteamUser, hSteamPipe);
  let server = match (&self_s.steam_pipes).get(&hSteamPipe).unwrap_or(&SteamPipe::NoUser) {
      SteamPipe::Server => true,
      _ => false
  };
  // debug!(server);
  // debug!("--");

  if !server && !version.starts_with("SteamNetworkingUtils") && !version.starts_with("SteamUtils")
    && hSteamUser == 0 {
      debug!("returning early!");
      return ptr::null_mut();
  }

  if version.starts_with("SteamNetworkingSockets") {
    return match server {
      true  => ptr::addr_of_mut!(self_s.gs_networking_sockets   ) as _,
      false => ptr::addr_of_mut!(self_s.steam_networking_sockets) as _,
    }
  }

  // FIXME: there are so many interfaces yet to be added here
  if version.starts_with("SteamUser") {
    return SteamAPI_ISteamClient_GetISteamUser(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("SteamUtils") {
    return SteamAPI_ISteamClient_GetISteamUtils(self_, _edx, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("SteamInput") {
    return SteamAPI_ISteamClient_GetISteamInput(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("SteamFriends") {
    return SteamAPI_ISteamClient_GetISteamFriends(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("SteamNetworkingUtils") {
    return ptr::addr_of_mut!(self_s.steam_networking_utils) as _;
  } else if version.starts_with("SteamGameServer") {
    return SteamAPI_ISteamClient_GetISteamGameServer(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("STEAMAPPS_INTERFACE_VERSION") {
    return SteamAPI_ISteamClient_GetISteamApps(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("STEAMPARENTALSETTINGS_INTERFACE_VERSION") {
    return SteamAPI_ISteamClient_GetISteamParentalSettings(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("STEAMUSERSTATS_INTERFACE_VERSION") {
    return SteamAPI_ISteamClient_GetISteamUserStats(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("STEAMREMOTESTORAGE_INTERFACE_VERSION") {
    return SteamAPI_ISteamClient_GetISteamRemoteStorage(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("STEAMHTMLSURFACE_INTERFACE_VERSION") {
    return SteamAPI_ISteamClient_GetISteamHTMLSurface(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("STEAMINVENTORY_INTERFACE") {
    return SteamAPI_ISteamClient_GetISteamInventory(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("STEAMUGC_INTERFACE_VERSION") {
    return SteamAPI_ISteamClient_GetISteamUGC(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  }
  
  ptr::null_mut()
}