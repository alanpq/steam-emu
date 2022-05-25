use crate::{int32, HSteamUser, HSteamPipe, SERVER_HSTEAMUSER, steam_api::get_steam_client};

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
pub unsafe extern "C" fn SteamGameServer_GetHSteamUser() -> HSteamUser {
  // debug!("SteamGameServer_GetHSteamUser");
  match (*get_steam_client()).is_server_init() {
    true => SERVER_HSTEAMUSER,
    false => 0
  }
}

#[no_mangle]
pub extern "C" fn SteamGameServer_Shutdown() {
  debug!("SteamGameServer_Shutdown");
  // FIXME: implement
}