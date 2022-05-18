use std::{os::raw::c_char, ffi::{c_void, CStr}, ptr::{null, self}, sync::RwLock};

use crate::{uint32, uint16, HSteamUser, uintp, steam_api::{get_steam_client}};
use tracing::{info, debug, error};

use lazy_static::lazy_static;

pub enum EServerMode {
  eServerModeInvalid,
  eServerModeNoAuthentication,
  eServerModeAuthentication,
  eServerModeAuthenticationAndSecure,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// FIXME: change these from *mut c_void
pub struct CSteamAPIContext {
    pub m_pSteamClient: *mut c_void,//ISteamClient,
    pub m_pSteamUser: *mut c_void,//ISteamUser,
    pub m_pSteamFriends: *mut c_void,//ISteamFriends,
    pub m_pSteamUtils: *mut c_void,//ISteamUtils,
    pub m_pSteamMatchmaking: *mut c_void,//ISteamMatchmaking,
    pub m_pSteamGameSearch: *mut c_void,//ISteamGameSearch,
    pub m_pSteamUserStats: *mut c_void,//ISteamUserStats,
    pub m_pSteamApps: *mut c_void,//ISteamApps,
    pub m_pSteamMatchmakingServers: *mut c_void,//ISteamMatchmakingServers,
    pub m_pSteamNetworking: *mut c_void,//ISteamNetworking,
    pub m_pSteamRemoteStorage: *mut c_void,//ISteamRemoteStorage,
    pub m_pSteamScreenshots: *mut c_void,//ISteamScreenshots,
    pub m_pSteamHTTP: *mut c_void,//ISteamHTTP,
    pub m_pController: *mut c_void,//ISteamController,
    pub m_pSteamUGC: *mut c_void,//ISteamUGC,
    pub m_pSteamAppList: *mut c_void,//ISteamAppList,
    pub m_pSteamMusic: *mut c_void,//ISteamMusic,
    pub m_pSteamMusicRemote: *mut c_void,//ISteamMusicRemote,
    pub m_pSteamHTMLSurface: *mut c_void,//ISteamHTMLSurface,
    pub m_pSteamInventory: *mut c_void,//ISteamInventory,
    pub m_pSteamVideo: *mut c_void,//ISteamVideo,
    pub m_pSteamParentalSettings: *mut c_void,//ISteamParentalSettings,
    pub m_pSteamInput: *mut c_void,//ISteamInput,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ContextInitData {
  p_fn: fn(pCtx: *mut CSteamAPIContext),
  counter: uintp,
  ctx: CSteamAPIContext,
}

lazy_static! {
  pub static ref GLOBAL_COUNTER: RwLock<uintp> = RwLock::new(1);
}

#[no_mangle]
pub extern "C" fn SteamInternal_ContextInit(
  pContextInitData: &mut ContextInitData
) -> &mut CSteamAPIContext {
  debug!("SteamInternal_ContextInit");
  // let mut ctx = unsafe { pContextInitData.as_mut() };
  let mut ctx = pContextInitData;
  // match ctx {
    // Some(ctx) => {
      debug!(ctx.counter);
      debug!(?ctx.ctx);
      // FIXME: wtf is happening here
      let counter = GLOBAL_COUNTER.read().unwrap();
      if ctx.counter != *counter {
        debug!("SteamInternal_ContextInit initializing...");
        (ctx.p_fn)(ptr::addr_of_mut!(ctx.ctx));
        ctx.counter = *counter;
      }
      // debug!("{:?}", ptr::addr_of_mut!(ctx.ctx));
      // ptr::addr_of_mut!(ctx.ctx)
      &mut ctx.ctx
    // },
    // None => panic!("SteamInternal_ContextInit: invalid pContextInitData!"),
// }
}

#[no_mangle]
pub unsafe extern "C" fn SteamInternal_FindOrCreateUserInterface(
  hSteamUser: HSteamUser, 
  pszVersion: *const c_char
) -> *mut c_void {
  debug!("SteamInternal_FindOrCreateUserInterface");
  ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn SteamInternal_FindOrCreateGameServerInterface(
  hSteamUser: HSteamUser, 
  pszVersion: *const c_char
) -> *mut c_void {
  debug!("SteamInternal_FindOrCreateGameServerInterface");
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
  debug!("SteamInternal_GameServer_Init");
  false
}

#[no_mangle]
pub unsafe extern "C" fn SteamInternal_CreateInterface(
  ver: *const c_char
) -> *mut c_void {
  debug!("SteamInternal_CreateInterface");
  let client = get_steam_client().as_ref().unwrap();
  if !client.is_user_logged_in() && !client.is_server_init() {return ptr::null_mut();}
  let str = unsafe {CStr::from_ptr(ver).to_str().unwrap()};
  debug!(ver=str);
  return create_client_interface(str);
}

pub unsafe fn create_client_interface(ver: &str) -> *mut c_void {
  debug!("create_client_interface");
  if !ver.contains("SteamClient") { return ptr::null_mut(); }
  // FIXME: actual versions of steamclient
  // (currently hardcoded for SteamClient017)
  let c = get_steam_client();
  debug!(?c);
  let vtable = (*c).vtable;
  debug!(?vtable);
  c as _
}