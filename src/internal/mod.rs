use std::{os::raw::c_char, ffi::{c_void, CStr}, ptr::{null, self}, sync::RwLock, intrinsics::transmute};

use crate::{uint32, uint16, HSteamUser, uintp, steam_api::{get_steam_client}};
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
  debug!("SteamInternal_ContextInit");
  unsafe {
    let ctx: *mut ContextInitData = transmute(pContextInitData);
    let mut ctx = *ctx;
  // let mut ctx = pContextInitData;
  // match ctx {
    // Some(ctx) => {
      ctx.ctx.vtable = steam_api_context::get_vtable();
      debug!(?ctx);
      // FIXME: wtf is happening here
      let counter = GLOBAL_COUNTER.read().unwrap();
      if ctx.counter != *counter {
        debug!("SteamInternal_ContextInit initializing...");
        let span = span!(Level::DEBUG, "SteamInternal_ContextInit");
        {
          let _guard = span.enter();
          (ctx.p_fn)(ptr::addr_of_mut!(ctx.ctx));
        }
        ctx.counter = *counter;
        debug!("set that counter");
      }
      // ptr::addr_of_mut!(ctx.ctx);
      &mut ctx.ctx
    // },
    // None => panic!("SteamInternal_ContextInit: invalid pContextInitData!"),
// }
  }
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