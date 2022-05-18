use std::{collections::HashMap, sync::Mutex, os::raw::c_char, ffi::c_void, ptr};

use tracing::{info, debug, error};

use lazy_static::lazy_static;
use vtables::VTable;
use vtables_derive::{VTable, has_vtable, virtual_index};
use crate::{HSteamPipe, HSteamUser, int32};

mod methods;

const CLIENT_HSTEAMUSER: HSteamUser = 1;

pub enum SteamPipe {
  NoUser,
  Client,
  Server
}

lazy_static! {
  static ref STEAM_PIPES: Mutex<HashMap<HSteamPipe, SteamPipe>> = Mutex::new(HashMap::new());
}

#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EAccountType {
  k_EAccountTypeInvalid = 0,
  k_EAccountTypeIndividual = 1,
  k_EAccountTypeMultiseat = 2,
  k_EAccountTypeGameServer = 3,
  k_EAccountTypeAnonGameServer = 4,
  k_EAccountTypePending = 5,
  k_EAccountTypeContentServer = 6,
  k_EAccountTypeClan = 7,
  k_EAccountTypeChat = 8,
  k_EAccountTypeConsoleUser = 9,
  k_EAccountTypeAnonUser = 10,
  k_EAccountTypeMax = 11,
}

#[has_vtable]
#[derive(VTable, Debug)]
pub struct SteamClient {
  pub vtable: *mut *mut usize,
  test: int32,
  server_init: bool,
  user_logged_in: bool,

  steam_user: *mut c_void,
}

#[allow(non_snake_case)]
impl SteamClient {
  #[virtual_index(0)]
  fn CreateSteamPipe(&self) -> HSteamPipe {}

  #[virtual_index(1)]
  fn BReleaseSteamPipe(&self, hSteamPipe:HSteamPipe) -> bool {}
  
  #[virtual_index(2)]
  fn ConnectToGlobalUser(&self, hSteamPipe:HSteamPipe) -> HSteamUser {}
  
  #[virtual_index(3)]
  fn CreateLocalUser(&self, hSteamPipe: *mut HSteamPipe, eAccountType:EAccountType) -> HSteamUser {}
  
  #[virtual_index(4)]
  fn ReleaseUser(&self, hSteamPipe:HSteamPipe, hSteamUser:HSteamUser) {}
  
  #[virtual_index(5)]
  fn GetISteamUser(&self, hSteamUser:HSteamUser, hSteamPipe:HSteamPipe, pchVersion: *const c_char) ->  *mut c_void {}
}
impl SteamClient {
  pub fn new() -> SteamClient {
    debug!("Init new SteamClient");
    let mut c = SteamClient {
      test: 48879,
      server_init: false,
      user_logged_in: false,
      steam_user: ptr::null_mut(),
      vtable: methods::get_vtable(),
    };
    c
  }

  pub fn create_steam_pipe() -> HSteamPipe {
    debug!("CreateSteamPipe");
    let mut pipes = STEAM_PIPES.lock().unwrap();
    let pipe = pipes.len() as HSteamPipe + 1;
    pipes.insert(pipe, SteamPipe::NoUser);
    pipe
  }

  pub fn connect_to_global_user(&mut self, hSteamPipe: HSteamPipe) -> HSteamUser {
    debug!("SteamClient::connect_to_global_user {}", hSteamPipe);
    let mut pipes = STEAM_PIPES.lock().unwrap();
    if pipes.len() < 1 { return 0; }

    self.user_log_in();

    pipes.insert(hSteamPipe, SteamPipe::Client);
    return CLIENT_HSTEAMUSER;
  }

  pub fn init_server(&mut self) {
    self.server_init = true;
  }

  pub fn shutdown_server(&mut self) {
    self.server_init = false;
  }

  pub fn is_server_init(&self) -> bool {
    self.server_init
  }

  pub fn user_log_in(&mut self) {
    // TODO: user login
    self.user_logged_in = true;
  }

  pub fn shutdown_client(&mut self) {
    self.user_logged_in = false;
  }

  pub fn is_user_logged_in(&self) -> bool {
    self.user_logged_in
  }

}