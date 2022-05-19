#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::{os::raw::c_char, ffi::c_void, ptr};

use tracing::{info, debug, error};

use crate::{HSteamPipe, HSteamUser, steam_api::{SteamUser, SteamFriends, SteamUtils, SteamScreenshots, SteamGameSearch, SteamRemoteStorage, SteamNetworking, SteamApps, SteamGameServerStats, SteamUserStats, SteamMatchmakingServers, SteamMatchmaking}};

use super::{SteamClient, EAccountType};

#[allow(non_camel_case_types)]

extern "C" fn SteamAPI_ISteamClient_CreateSteamPipe(self_: *mut SteamClient) -> HSteamPipe {
  debug!("SteamAPI_ISteamClient_CreateSteamPipe");
  0
}

extern "C" fn SteamAPI_ISteamClient_BReleaseSteamPipe(
  self_: *mut SteamClient,
  hSteamPipe:HSteamPipe
) -> bool {
  debug!("SteamAPI_ISteamClient_BReleaseSteamPipe");
  false
}

extern "C" fn SteamAPI_ISteamClient_ConnectToGlobalUser
  (self_: *mut SteamClient, hSteamPipe:HSteamPipe) -> HSteamUser {
  debug!("SteamAPI_ISteamClient_ConnectToGlobalUser");
  0
}

extern "C" fn SteamAPI_ISteamClient_CreateLocalUser(
  self_: *mut SteamClient,
  hSteamPipe: *mut HSteamPipe,
  eAccountType:EAccountType
) -> HSteamUser {
  debug!("SteamAPI_ISteamClient_CreateLocalUser");
  0
}

extern "C" fn SteamAPI_ISteamClient_ReleaseUser(
  self_: *mut SteamClient,
  hSteamPipe:HSteamPipe,
  hSteamUser:HSteamUser
) {
  debug!("SteamAPI_ISteamClient_ReleaseUser");
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamUser(
  self_: *mut SteamClient,
  _edx: *mut c_void,
  hSteamUser:HSteamUser,
  hSteamPipe:HSteamPipe,
  pchVersion: *const c_char
) ->  *mut SteamUser {
  let p = ptr::addr_of_mut!((*self_).steam_user);
  debug!("GetISteamUser -> {:?}", p);
  ptr::addr_of_mut!((*self_).steam_user)
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamFriends(
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamFriends {
  let p = ptr::addr_of_mut!((*self_).steam_friends);
  debug!("GetISteamFriends -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamUtils(
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamUtils {
  let p = ptr::addr_of_mut!((*self_).steam_utils);
  debug!("GetISteamUtils -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamMatchmaking(
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamMatchmaking {
  let p = ptr::addr_of_mut!((*self_).steam_matchmaking);
  debug!("GetISteamMatchmaking -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamMatchmakingServers(
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamMatchmakingServers {
  let p = ptr::addr_of_mut!((*self_).steam_matchmaking_servers);
  debug!("GetISteamMatchmakingServers -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamGenericInterface(
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut c_void {
  debug!("GetISteamGenericInterface -> 0x0");
  ptr::null_mut()
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamUserStats (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamUserStats {
  let p = ptr::addr_of_mut!((*self_).steam_user_stats);
  debug!("GetISteamUserStats -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamGameServerStats (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamGameServerStats {
  let p = ptr::addr_of_mut!((*self_).steam_game_server_stats);
  debug!("GetISteamGameServerStats -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamApps (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamApps {
  let p = ptr::addr_of_mut!((*self_).steam_apps);
  debug!("GetISteamApps -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamNetworking (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamNetworking {
  let p = ptr::addr_of_mut!((*self_).steam_networking);
  debug!("GetISteamNetworking -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamRemoteStorage (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamRemoteStorage {
  let p = ptr::addr_of_mut!((*self_).steam_remote_storage);
  debug!("GetISteamRemoteStorage -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamScreenshots (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamScreenshots {
  let p = ptr::addr_of_mut!((*self_).steam_screenshots);
  debug!("GetISteamScreenshots -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamGameSearch (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamGameSearch {
  let p = ptr::addr_of_mut!((*self_).steam_game_search);
  debug!("GetISteamGameSearch -> {:?}", p);
  p
}

// FIXME: fix from here down

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamHTTP (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamGameSearch {
  let p = ptr::addr_of_mut!((*self_).steam_game_search);
  debug!("GetISteamHTTP -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamController (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamGameSearch {
  let p = ptr::addr_of_mut!((*self_).steam_game_search);
  debug!("GetISteamController -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamUGC (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamGameSearch {
  let p = ptr::addr_of_mut!((*self_).steam_game_search);
  debug!("GetISteamUGC -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamAppList (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamGameSearch {
  let p = ptr::addr_of_mut!((*self_).steam_game_search);
  debug!("GetISteamAppList -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamMusic (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamGameSearch {
  let p = ptr::addr_of_mut!((*self_).steam_game_search);
  debug!("GetISteamMusic -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamMusicRemote (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamGameSearch {
  let p = ptr::addr_of_mut!((*self_).steam_game_search);
  debug!("GetISteamMusicRemote -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamHTMLSurface (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamGameSearch {
  let p = ptr::addr_of_mut!((*self_).steam_game_search);
  debug!("GetISteamHTMLSurface -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamInventory (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamGameSearch {
  let p = ptr::addr_of_mut!((*self_).steam_game_search);
  debug!("GetISteamInventory -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamVideo (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamGameSearch {
  let p = ptr::addr_of_mut!((*self_).steam_game_search);
  debug!("GetISteamVideo -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamParentalSettings (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamGameSearch {
  let p = ptr::addr_of_mut!((*self_).steam_game_search);
  debug!("GetISteamParentalSettings -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamInput (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamGameSearch {
  let p = ptr::addr_of_mut!((*self_).steam_game_search);
  debug!("GetISteamInput -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamParties (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamGameSearch {
  let p = ptr::addr_of_mut!((*self_).steam_game_search);
  debug!("GetISteamParties -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamRemotePlay (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut SteamGameSearch {
  let p = ptr::addr_of_mut!((*self_).steam_game_search);
  debug!("GetISteamRemotePlay -> {:?}", p);
  p
}

unsafe extern "fastcall" fn SteamAPI_ISteamClient_DEPRECATED_GetISteamUnifiedMessages (
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut c_void {
  let p = ptr::addr_of_mut!((*self_).steam_game_search);
  debug!("GetISteamUnifiedMessages -> {:?}", p);
  p as _
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 40] =  [
      SteamAPI_ISteamClient_CreateSteamPipe as _,
      SteamAPI_ISteamClient_BReleaseSteamPipe as _,
      SteamAPI_ISteamClient_ConnectToGlobalUser as _,
      SteamAPI_ISteamClient_CreateLocalUser as _,
      SteamAPI_ISteamClient_ReleaseUser as _,
      SteamAPI_ISteamClient_GetISteamUser as _,
      ptr::null_mut(), // GetISteamGameServer
      ptr::null_mut(), // SetLocalIPBinding
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
      ptr::null_mut(), // GetIPCCallCount
      ptr::null_mut(), // SetWarningMessageHook
      ptr::null_mut(), // BShutdownIfAllPipesClosed
      SteamAPI_ISteamClient_GetISteamHTTP as _,
      SteamAPI_ISteamClient_DEPRECATED_GetISteamUnifiedMessages as _,
      SteamAPI_ISteamClient_GetISteamController as _,
      SteamAPI_ISteamClient_GetISteamUGC as _,
      SteamAPI_ISteamClient_GetISteamAppList as _,
      SteamAPI_ISteamClient_GetISteamMusic as _,
      SteamAPI_ISteamClient_GetISteamMusicRemote as _,
      SteamAPI_ISteamClient_GetISteamHTMLSurface as _,
      ptr::null_mut(), // DEPRECATED_Set_SteamAPI_CPostAPIResultInProcess
      ptr::null_mut(), // DEPRECATED_Remove_SteamAPI_CPostAPIResultInProcess
      ptr::null_mut(), // Set_SteamAPI_CCheckCallbackRegisteredInProcess
      SteamAPI_ISteamClient_GetISteamInventory as _,
      SteamAPI_ISteamClient_GetISteamVideo as _,
      SteamAPI_ISteamClient_GetISteamParentalSettings as _,
      SteamAPI_ISteamClient_GetISteamInput as _,
      SteamAPI_ISteamClient_GetISteamParties as _,
      SteamAPI_ISteamClient_GetISteamRemotePlay as _,
    ];
    VTABLE.as_mut_ptr()
  }
}