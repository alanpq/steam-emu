use std::{os::raw::c_char, ffi::{c_void, CStr}, ptr, sync::Mutex};
use lazy_static::__Deref;
use tracing::{info, debug, error};

use vtables::VTable;
use vtables_derive::{VTable, has_vtable};

use crate::{uint32, uint16, HSteamPipe};

use super::AppId;

#[has_vtable]
#[derive(VTable, Debug)]
pub struct SteamGameServer {
  product: String,
  game_description: String,
  mod_dir: String,

  is_dedicated: bool,
}

impl SteamGameServer {
  pub fn new() -> Self {
    Self { vtable: get_vtable(),
      product: String::new(),
      game_description: String::new(),
      mod_dir: String::new(),

      is_dedicated: false,
    }
  }

  pub fn product(&self) -> &str {
    &self.product
  }
  pub fn game_description(&self) -> &str {
    &self.game_description
  }
  pub fn mod_dir(&self) -> &str {
    &self.mod_dir
  }

  pub fn is_dedicated(&self) -> bool {
    self.is_dedicated
  }

  pub fn set_product<S: AsRef<str>>(&mut self, product: S) {
    self.product = String::from(product.as_ref());
  }
  pub fn set_game_description<S: AsRef<str>>(&mut self, game_description: S) {
    self.game_description = String::from(game_description.as_ref());
  }
  pub fn set_mod_dir<S: AsRef<str>>(&mut self, mod_dir: S) {
    self.mod_dir = String::from(mod_dir.as_ref());
  }

  pub fn set_dedicated(&mut self, dedicated: bool) {
    self.is_dedicated = dedicated;
  }
}

pub extern "fastcall" fn InitGameServer(
  self_: *mut SteamGameServer,
  _edx: *mut c_void,
  ip: uint32,
  game_port: uint16,
  query_port: uint16,
  flags: uint32,
  game_app_id: AppId,
  version_string: *const c_char,
) -> bool {
  info!("InitGameServer");
  true
}

pub unsafe extern "fastcall" fn SetProduct(
  self_: *mut SteamGameServer,
  _edx: *mut c_void,
  val: *const c_char,
) {
  (*self_).set_product(CStr::from_ptr(val).to_str().unwrap());
}
pub unsafe extern "fastcall" fn SetGameDescription(
  self_: *mut SteamGameServer,
  _edx: *mut c_void,
  val: *const c_char,
) {
  (*self_).set_game_description(CStr::from_ptr(val).to_str().unwrap());
}
pub unsafe extern "fastcall" fn SetModDir(
  self_: *mut SteamGameServer,
  _edx: *mut c_void,
  val: *const c_char,
) {
  (*self_).set_mod_dir(CStr::from_ptr(val).to_str().unwrap());
}
pub unsafe extern "fastcall" fn SetDedicatedServer(
  self_: *mut SteamGameServer,
  _edx: *mut c_void,
  val: bool,
) {
  (*self_).set_dedicated(val);
}

/// Login to a generic, anonymous account.
///
/// Note: in previous versions of the SDK, this was automatically called within SteamGameServer_Init,
/// but this is no longer the case.
pub unsafe extern "fastcall" fn LogOnAnonymous(
  self_: *mut SteamGameServer,
  _edx: *mut c_void,
) {
  // FIXME: implement
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 11] = [
      // Basic server data.  These properties, if set, must be set before before calling LogOn.  They
      // may not be changed after logged in.
      SetProduct as _, // SetProduct
      SetGameDescription as _, // SetGameDescription
      SetModDir as _, // SetModDir
      SetDedicatedServer as _, // SetDedicatedServer

      ptr::null_mut(), // LogOn
      LogOnAnonymous as _, // LogOnAnonymous
      
      ptr::null_mut(), // LogOff
      ptr::null_mut(), // BLoggedOn
      ptr::null_mut(), // BSecure
      ptr::null_mut(), // GetSteamID
      ptr::null_mut(), // WasRestartRequested
      // ...
    ];
    VTABLE.as_mut_ptr()
  }
}