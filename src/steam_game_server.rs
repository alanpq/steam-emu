use crate::{int32, HSteamUser, HSteamPipe};

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