use crate::{int32, HSteamUser, HSteamPipe};

use tracing::{info, debug, error};
#[no_mangle]
pub extern "C" fn SteamGameServer_GetHSteamPipe() -> HSteamPipe {
  debug!("SteamGameServer_GetHSteamPipe");
  0 // FIXME: implement
}

#[no_mangle]
pub extern "C" fn SteamGameServer_RunCallbacks() {
  debug!("SteamGameServer_RunCallbacks");
   // FIXME: implement
}

#[no_mangle]
pub extern "C" fn SteamGameServer_GetHSteamUser() -> HSteamUser {
  debug!("SteamGameServer_GetHSteamUser");
  0 // FIXME: implement
}

#[no_mangle]
pub extern "C" fn SteamGameServer_Shutdown() {
  debug!("SteamGameServer_Shutdown");
  // FIXME: implement
}