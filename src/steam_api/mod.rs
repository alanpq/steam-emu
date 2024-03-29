#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::{ffi::c_void, os::raw::{c_char, c_int}, sync::{Mutex, Arc, RwLock}, ptr};

use tracing::{info, debug, error, span, Level};

use crate::{uint32, uint8, HSteamUser, HSteamPipe, uint64, steam_client::{SteamClient}, internal::{GLOBAL_COUNTER, SteamInternal_CreateInterface}, CLIENT};

use lazy_static::lazy_static;

mod callbacks;

mod app_list;
mod apps;
mod controller;
mod friends;
mod game_search;
mod game_server;
mod game_server_stats;
mod html_surface;
mod http;
mod input;
mod interfaces;
mod inventory;
mod matchmaking_servers;
mod matchmaking;
mod music_remote;
mod music;
mod parental_settings;
mod parties;
mod remote_play;
mod remote_storage;
mod screenshots;
mod ugc;
mod user_stats;
mod user;
mod utils;
mod video;

pub use callbacks::*;

pub use app_list::*;
pub use apps::*;
pub use controller::*;
pub use friends::*;
pub use game_search::*;
pub use game_server::*;
pub use game_server_stats::*;
pub use html_surface::*;
pub use http::*;
pub use input::*;
pub use inventory::*;
pub use matchmaking_servers::*;
pub use matchmaking::*;
pub use music_remote::*;
pub use music::*;
pub use parental_settings::*;
pub use parties::*;
pub use remote_play::*;
pub use remote_storage::*;
pub use screenshots::*;
pub use ugc::*;
pub use user_stats::*;
pub use user::*;
pub use utils::*;
pub use video::*;

pub mod networking;

lazy_static! {
  static ref USER_STEAM_PIPE: RwLock<HSteamPipe> = RwLock::new(0);
}


fn load_old_interface_versions() {
  static loaded: bool = false;
  if loaded {return;}


}

pub unsafe fn get_steam_client() -> *mut SteamClient {
  CLIENT
}

pub fn safe_get_steam_client() -> &'static mut SteamClient {
  let p = unsafe {CLIENT};
  if p.is_null() {
    error!("SteamClient is null!");
    panic!();
  }
  unsafe {&mut *p}
}

#[no_mangle]
pub unsafe extern "C" fn SteamAPI_Init() -> bool {
  debug!("SteamAPI_Init");
  if *(USER_STEAM_PIPE.read().unwrap()) != 0 {return true;}
  
  // FIXME: load steam_interfaces.txt - load_old_interface_versions() (https://gitlab.com/Mr_Goldberg/goldberg_emulator/-/blob/master/dll/dll.cpp#L234)
  
  let mut pipe = USER_STEAM_PIPE.write().unwrap();
  let client = get_steam_client();
  *pipe = SteamClient::create_steam_pipe();
  client.as_mut().unwrap().connect_to_global_user(*pipe);
  let mut counter = GLOBAL_COUNTER.write().unwrap();
  *counter += 1;
  true
}
#[no_mangle]
pub extern "C" fn SteamAPI_Shutdown() {
  debug!("SteamAPI_Shutdown");
}
#[no_mangle]
pub unsafe extern "C" fn SteamAPI_RunCallbacks() {
  let span = span!(Level::DEBUG, "SteamAPI_RunCallbacks");
  let _enter = span.enter();
  safe_get_steam_client().run_callbacks(true, false);
}

#[no_mangle]
pub unsafe extern "C" fn SteamAPI_GetHSteamPipe() -> HSteamPipe {
  *(USER_STEAM_PIPE.read().unwrap())
}
#[no_mangle]
pub unsafe extern "C" fn SteamAPI_GetHSteamUser() -> HSteamUser {
  1
}

#[no_mangle]
pub unsafe extern "C" fn SteamClient() -> *mut SteamClient {
  debug!("SteamClient");
  // load_old_interface_versions()
  if !(*get_steam_client()).is_user_logged_in() { return ptr::null_mut(); }
  return SteamInternal_CreateInterface(interfaces::OLD_CLIENT.as_ptr()) as _;
}

// FIXME: SteamUGC_v016
#[cfg(target_os = "windows")]
use windows::Win32::System::Diagnostics::Debug::EXCEPTION_POINTERS;

#[cfg(not(target_os = "windows"))]
type EXCEPTION_POINTERS = c_void;

#[no_mangle]
pub unsafe extern "C" fn SteamAPI_WriteMiniDump(
  uStructuredExceptionCode: uint32, // The structured exception code.
  pvExceptionInfo: *mut EXCEPTION_POINTERS,     // The EXCEPTION_POINTERS containing the action exception info
  uBuildID: uint32                  // A Build ID to track what version of the app submitted this minidump.
                                    // This is not the same as a Steam build ID and is used only for crash reporting.
) {
  error!("====== SteamAPI_WriteMiniDump ======");
  error!("Structured Exception Code: {:x}", uStructuredExceptionCode);
  #[cfg(target_os = "windows")] {
    let exception = *pvExceptionInfo;
    let mut rec_ptr = exception.ExceptionRecord;

    
    while !rec_ptr.is_null() {
      let record = *rec_ptr;
      let address = record.ExceptionAddress;
      error!(?address);
      let code = record.ExceptionCode;
      let message = code.to_hresult().message();
      error!(?code);
      error!(%message);
      rec_ptr = record.ExceptionRecord;
    }
  }
}

// FIXME: IsSteamRunning
// FIXME: SteamApps_v008
// FIXME: SteamHTTP_v003
// FIXME: SteamUser_v021
// FIXME: SteamInput_v006
// FIXME: SteamMusic_v001
// FIXME: SteamUtils_v010
// FIXME: SteamVideo_v002
// FIXME: ISteamInput_Init
// FIXME: ISteamMusic_Play

#[no_mangle]
pub unsafe extern "C" fn SteamAPI_RegisterCallback(
  pCallback: *mut CCallbackBase, 
  iCallback: CallbackType
) {
  debug!("SteamAPI_RegisterCallback");
  let cb = *pCallback;
  debug!(?cb);
  // FIXME: mutex is used here
  (*get_steam_client()).register_callback(pCallback, iCallback)
}

// FIXME: ISteamMusic_Pause
// FIXME: SteamAppList_v001
// FIXME: SteamFriends_v017
// FIXME: SteamParties_v002
#[no_mangle]
pub unsafe extern "C" fn SteamAPI_RegisterCallResult(
  pCallback: *mut CCallbackBase, 
  hAPICall: SteamAPICall_t
) {
  debug!("SteamAPI_RegisterCallResult");
  let cb = *pCallback;
  debug!(?cb);
}

#[no_mangle]
pub extern "C" fn SteamAPI_SetMiniDumpComment(pchMsg: *const c_char) {
  
}

#[no_mangle]
pub unsafe extern "C" fn SteamAPI_UnregisterCallback(
  pCallback: *mut CCallbackBase
) {
  debug!("SteamAPI_UnregisterCallback");
}


// FIXME: GetSteamInstallPath
// FIXME: ISteamUser_GetVoice
// FIXME: ManualDispatch_Init
// FIXME: servernetadr_t_Init
// FIXME: SteamInventory_v003
// FIXME: SteamUserStats_v012
// FIXME: ISteamHTTP_SetCookie
// FIXME: ISteamInput_RunFrame
// FIXME: ISteamInput_Shutdown
// FIXME: ISteamMusic_PlayNext
// FIXME: ISteamUGC_CreateItem
// FIXME: ISteamUGC_DeleteItem
// FIXME: ISteamUser_BLoggedOn
// FIXME: ISteamUtils_GetAppID
// FIXME: servernetadr_t_GetIP
// FIXME: servernetadr_t_SetIP
// FIXME: SetTryCatchCallbacks
// FIXME: SteamController_v008
// FIXME: SteamGameSearch_v001
// FIXME: SteamGameServer_v014
// FIXME: SteamNetworking_v006
// FIXME: SteamRemotePlay_v001

#[no_mangle]
pub unsafe extern "C" fn SteamAPI_UnregisterCallResult(
  pCallback: *mut CCallbackBase, 
  hAPICall: SteamAPICall_t
) {
  debug!("SteamAPI_UnregisterCallResult");
}
// FIXME: ISteamApps_InstallDLC
// FIXME: ISteamController_Init
// FIXME: ISteamMusic_GetVolume
// FIXME: ISteamMusic_SetVolume
// FIXME: ISteamUGC_SetItemTags
// FIXME: ISteamUGC_SetLanguage
// FIXME: ISteamUser_GetSteamID
#[no_mangle]
pub extern "C" fn SteamAPI_RestartAppIfNecessary(
  unOwnAppID: uint32
) -> bool {
  false
}
// FIXME: servernetadr_t_Assign 
// FIXME: SteamHTMLSurface_v005
// FIXME: SteamMatchmaking_v009
// FIXME: SteamMusicRemote_v001
// FIXME: SteamScreenshots_v003
// FIXME: ISteamApps_GetAppOwner
// FIXME: ISteamApps_GetDLCCount
// FIXME: ISteamGameServer_LogOn
// FIXME: ISteamHTMLSurface_Find
// FIXME: ISteamHTMLSurface_Init
// FIXME: ISteamMusic_BlsEnabled
// FIXME: SteamAPI_ISteamMusic_BIsPlaying
// FIXME: SteamAPI_ISteamUGC_DownloadItem
// FIXME: SteamAPI_ISteamUGC_GetItemState
// FIXME: SteamAPI_ISteamUGC_SetItemTitle
// FIXME: SteamAPI_ISteamUtils_FilterText
// FIXME: SteamAPI_SteamIPAddress_t_IsSet
// FIXME: SteamAPI_ISteamApps_BIsCybercafe
// FIXME: SteamAPI_ISteamApps_BIsVACBanned
// FIXME: SteamAPI_ISteamApps_UninstallDLC
// FIXME: SteamAPI_ISteamFriends_HasFriend
// FIXME: SteamAPI_ISteamGameServer_LogOff
// FIXME: SteamAPI_ISteamHTMLSurface_KeyUp
// FIXME: SteamAPI_ISteamInput_SetLEDColor
// FIXME: SteamAPI_ISteamParties_JoinParty
// FIXME: SteamAPI_ISteamUGC_AddDependency
// FIXME: SteamAPI_ISteamUGC_SetSearchText
// FIXME: SteamAPI_ISteamUGC_SubscribeItem
// FIXME: SteamAPI_ISteamUser_BIsBehindNAT
// FIXME: SteamAPI_ISteamVideo_GetVideoURL
// FIXME: SteamAPI_ManualDispatch_RunFrame
// FIXME: SteamAPI_SteamGameServerUGC_v016
// FIXME: SteamAPI_SteamRemoteStorage_v016
// FIXME: SteamAPI_gameserveritem_t_GetName
// FIXME: SteamAPI_gameserveritem_t_SetName
// FIXME: SteamAPI_ISteamAppList_GetAppName
// FIXME: SteamAPI_ISteamApps_BIsSubscribed
// FIXME: SteamAPI_ISteamApps_BIsTimedTrial
// FIXME: SteamAPI_ISteamApps_GetAppBuildId
// FIXME: SteamAPI_ISteamClient_ReleaseUser
// FIXME: SteamAPI_ISteamFriends_GetClanTag
// FIXME: SteamAPI_ISteamGameSearch_EndGame
// FIXME: SteamAPI_ISteamGameServer_BSecure
// FIXME: SteamAPI_ISteamHTMLSurface_GoBack
// FIXME: SteamAPI_ISteamHTMLSurface_Reload
// FIXME: SteamAPI_ISteamInput_BWaitForData
// FIXME: SteamAPI_ISteamMusic_PlayPrevious
// FIXME: SteamAPI_ISteamUGC_AddExcludedTag
// FIXME: SteamAPI_ISteamUGC_AddRequiredTag
// FIXME: SteamAPI_ISteamUGC_GetQueryUGCTag
// FIXME: SteamAPI_ISteamUGC_SetItemContent
// FIXME: SteamAPI_ISteamUGC_SetItemPreview
// FIXME: SteamAPI_ISteamUGC_SetMatchAnyTag
// FIXME: SteamAPI_ISteamUser_AdvertiseGame
// FIXME: SteamAPI_ISteamUser_GetHSteamUser
// FIXME: SteamAPI_ISteamUtils_GetImageRGBA
// FIXME: SteamAPI_ISteamUtils_GetImageSize
// FIXME: SteamAPI_ISteamUtils_GetIPCountry
// FIXME: SteamAPI_servernetadr_t_Construct
// FIXME: SteamAPI_SteamGameServerHTTP_v003
// FIXME: SteamAPI_CheckCallbackRegistered_t
// FIXME: SteamAPI_ISteamApps_BIsLowViolence
// FIXME: SteamAPI_ISteamApps_GetFileDetails
// FIXME: SteamAPI_ISteamClient_GetISteamUGC
// FIXME: SteamAPI_ISteamController_RunFrame
// FIXME: SteamAPI_ISteamController_Shutdown
// FIXME: SteamAPI_ISteamFriends_GetClanName
// FIXME: SteamAPI_ISteamFriends_IsFollowing
// FIXME: SteamAPI_ISteamHTMLSurface_KeyChar
// FIXME: SteamAPI_ISteamHTMLSurface_KeyDown
// FIXME: SteamAPI_ISteamHTMLSurface_LoadURL
// FIXME: SteamAPI_ISteamHTMLSurface_MouseUp
// FIXME: SteamAPI_ISteamHTMLSurface_SetSize
// FIXME: SteamAPI_ISteamInput_GetMotionData
// FIXME: SteamAPI_ISteamScreenshots_TagUser
// FIXME: SteamAPI_ISteamUGC_GetUserItemVote
// FIXME: SteamAPI_ISteamUGC_SetItemMetadata
// FIXME: SteamAPI_ISteamUGC_SetUserItemVote
// FIXME: SteamAPI_ISteamUGC_StartItemUpdate
// FIXME: SteamAPI_ISteamUGC_UnsubscribeItem
// FIXME: SteamAPI_ISteamUser_EndAuthSession
// FIXME: SteamAPI_servernetadr_t_IsLessThan
// FIXME: SteamAPI_SteamGameServerStats_v001
// FIXME: SteamAPI_SteamGameServerUtils_v010
// FIXME: SteamAPI_gameserveritem_t_Construct
// FIXME: SteamAPI_ISteamApps_BIsAppInstalled
// FIXME: SteamAPI_ISteamApps_BIsDlcInstalled
// FIXME: SteamAPI_ISteamClient_GetISteamApps
// FIXME: SteamAPI_ISteamClient_GetISteamHTTP
// FIXME: SteamAPI_ISteamClient_GetISteamUser
// FIXME: SteamAPI_ISteamFriends_GetClanCount
// FIXME: SteamAPI_ISteamFriends_GetClanOwner
// FIXME: SteamAPI_ISteamFriends_IsClanPublic
// FIXME: SteamAPI_ISteamGameServer_BLoggedOn
// FIXME: SteamAPI_ISteamGameServer_SetModDir
// FIXME: SteamAPI_ISteamGameServer_SetRegion
// FIXME: SteamAPI_ISteamHTMLSurface_Shutdown
// FIXME: SteamAPI_ISteamHTMLSurface_StopFind
// FIXME: SteamAPI_ISteamHTMLSurface_StopLoad
// FIXME: SteamAPI_ISteamHTTP_SendHTTPRequest
// FIXME: SteamAPI_ISteamInventory_TradeItems
// FIXME: SteamAPI_ISteamParties_CreateBeacon
// FIXME: SteamAPI_ISteamUGC_AddAppDependency
// FIXME: SteamAPI_ISteamUGC_RemoveDependency
// FIXME: SteamAPI_ISteamUGC_SetReturnOnlyIDs
// FIXME: SteamAPI_ISteamUGC_ShowWorkshopEULA
// FIXME: SteamAPI_ISteamUGC_SubmitItemUpdate
// FIXME: SteamAPI_ISteamUGC_SuspendDownloads
// FIXME: SteamAPI_ISteamUser_DecompressVoice
// FIXME: SteamAPI_ISteamUserStats_StoreStats
// FIXME: SteamAPI_ISteamUtils_InitFilterText
// FIXME: SteamAPI_ISteamVideo_GetOPFSettings
// FIXME: SteamAPI_ISteamVideo_IsBroadcasting
// FIXME: SteamAPI_ReleaseCurrentThreadMemory
// FIXME: SteamAPI_SteamParentalSettings_v001
// FIXME: SteamAPICallCompleted_t_k_iCallback
// FIXME: SteamAPI_ISteamAppList_GetAppBuildId
// FIXME: SteamAPI_ISteamApps_BIsSubscribedApp
// FIXME: SteamAPI_ISteamApps_GetAppInstallDir
// FIXME: SteamAPI_ISteamClient_GetISteamInput
// FIXME: SteamAPI_ISteamClient_GetISteamMusic
// FIXME: SteamAPI_ISteamClient_GetISteamUtils
// FIXME: SteamAPI_ISteamClient_GetISteamVideo
// FIXME: SteamAPI_ISteamFriends_SetPlayedWith
// FIXME: SteamAPI_ISteamGameSearch_AcceptGame
// FIXME: SteamAPI_ISteamGameServer_GetSteamID
// FIXME: SteamAPI_ISteamGameServer_SetMapName
// FIXME: SteamAPI_ISteamGameServer_SetProduct
// FIXME: SteamAPI_ISteamHTMLSurface_AddHeader
// FIXME: SteamAPI_ISteamHTMLSurface_GoForward
// FIXME: SteamAPI_ISteamHTMLSurface_MouseDown
// FIXME: SteamAPI_ISteamHTMLSurface_MouseMove
// FIXME: SteamAPI_ISteamHTMLSurface_SetCookie
// FIXME: SteamAPI_ISteamHTTP_DeferHTTPRequest
// FIXME: SteamAPI_ISteamInventory_ConsumeItem