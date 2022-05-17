use std::{net::TcpStream, sync::Mutex, os::raw::{c_int, c_uint, c_short, c_ushort, c_char}};

use tracing::info;

#[ctor::ctor]
fn ctor() {
  let stream = TcpStream::connect("127.0.0.1:1337").unwrap();
  tracing_subscriber::fmt().with_writer(Mutex::new(stream)).init();

  info!("hi");
}

#[no_mangle]
pub extern "C" fn SteamAPI_Init() -> bool {
  info!("init steamapi");
  false
}

type int32 = c_int;
type uint32 = c_uint;
type uint16 = c_ushort;

type HSteamPipe = int32;
type HSteamUser = int32;
#[no_mangle]
pub extern "C" fn SteamGameServer_GetHSteamPipe() -> HSteamPipe {
  0 // FIXME: implement
}

#[no_mangle]
pub extern "C" fn SteamGameServer_RunCallbacks() {
   // FIXME: implement
}

#[no_mangle]
pub extern "C" fn SteamGameServer_GetHSteamUser() -> HSteamUser {
  0 // FIXME: implement
}

#[no_mangle]
pub extern "C" fn SteamGameServer_Shutdown() {}


pub enum EServerMode {
  eServerModeInvalid,
  eServerModeNoAuthentication,
  eServerModeAuthentication,
  eServerModeAuthenticationAndSecure,
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