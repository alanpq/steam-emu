use std::{os::raw::c_char, ffi::{c_void, CStr}, ptr::{null, self}, sync::RwLock, intrinsics::transmute};

use crate::{uint32, uint16, HSteamUser, uintp, steam_api::{get_steam_client, SteamAPI_GetHSteamPipe, self}, steam_client::*, SERVER_STEAM_PIPE};
use tracing::{info, debug, error, span, Level};

use lazy_static::lazy_static;

use self::steam_api_context::CSteamAPIContext;

mod steam_api_context;

pub enum EServerMode {
  eServerModeInvalid,
  eServerModeNoAuthentication,
  eServerModeAuthentication,
  eServerModeAuthenticationAndSecure,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ContextInitData {
  p_fn: fn(pCtx: *mut CSteamAPIContext),
  counter: uintp,
  ctx: CSteamAPIContext,
}

lazy_static! {
  pub static ref GLOBAL_COUNTER: RwLock<uintp> = RwLock::new(0);
}

#[no_mangle]
pub extern "C" fn SteamInternal_ContextInit(
  pContextInitData: *mut c_void
) -> *mut CSteamAPIContext {
  unsafe {
    let ctx: *mut ContextInitData = transmute(pContextInitData);
    let mut ctx = *ctx;

    // debug!(?ctx);
    // FIXME: wtf is happening here
    let counter = GLOBAL_COUNTER.read().unwrap();
    if ctx.counter != *counter {
      let span = span!(Level::DEBUG, "SteamInternal_ContextInit");
      {
        let _guard = span.enter();
        (ctx.p_fn)(ptr::addr_of_mut!(ctx.ctx));
      }
      ctx.counter = *counter;
    }
    &mut ctx.ctx
// }
  }
}

#[no_mangle]
pub unsafe extern "C" fn SteamInternal_FindOrCreateUserInterface(
  hSteamUser: HSteamUser, 
  pszVersion: *const c_char
) -> *mut c_void {
  let v = CStr::from_ptr(pszVersion).to_str().unwrap();
  // debug!("SteamInternal_FindOrCreateUserInterface -> {}", v);
  SteamAPI_ISteamClient_GetISteamGenericInterface(get_steam_client(),  hSteamUser, SteamAPI_GetHSteamPipe(), pszVersion)
}

#[no_mangle]
pub unsafe extern "C" fn SteamInternal_FindOrCreateGameServerInterface(
  hSteamUser: HSteamUser,
  pszVersion: *const c_char
) -> *mut c_void {
  let v = CStr::from_ptr(pszVersion).to_str().unwrap();
  // debug!("SteamInternal_FindOrCreateGameServerInterface -> {}", v);
  SteamAPI_ISteamClient_GetISteamGenericInterface(get_steam_client(), hSteamUser, SteamAPI_GetHSteamPipe(), pszVersion)
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
  let client = unsafe {&mut *get_steam_client()};
  unsafe {
    SteamAPI_ISteamClient_CreateLocalUser(get_steam_client(), SERVER_STEAM_PIPE, EAccountType::k_EAccountTypeGameServer);
  }
  { // TODO: replace this with atomic u64
    let mut counter = GLOBAL_COUNTER.write().unwrap();
    *counter += 1;
  }
  //g_pSteamClientGameServer is only used in pre 1.37 (where the interface versions are not provided by the game)
  // g_pSteamClientGameServer = SteamGameServerClient();
  let un_flags = match eServerMode {
    // EServerMode::eServerModeAuthenticationAndSecure => k_unServerFlagSecure // TODO: implement (vac related)
    _ => 0
  };
  steam_api::InitGameServer(ptr::addr_of_mut!(client.gs), ptr::null_mut(), unIP, usGamePort, usQueryPort, un_flags, 0, pchVersionString)
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