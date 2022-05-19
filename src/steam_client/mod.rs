use std::{collections::HashMap, sync::Mutex, os::raw::c_char, ffi::c_void, ptr};

use tracing::{info, debug, error};

use lazy_static::lazy_static;
use vtables::VTable;
use vtables_derive::{VTable, has_vtable};
use crate::{HSteamPipe, HSteamUser, int32, steam_api::*};

mod methods;
mod get_generic_interfaces;
pub use methods::*;
pub use get_generic_interfaces::*;

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

  steam_user: SteamUser,
  steam_friends: SteamFriends,
  steam_utils: SteamUtils,
  steam_matchmaking: SteamMatchmaking,
  steam_matchmaking_servers: SteamMatchmakingServers,
  steam_user_stats: SteamUserStats,
  steam_game_server_stats: SteamGameServerStats,
  steam_input: SteamInput,
  steam_apps: SteamApps,
  steam_networking: SteamNetworking,
  steam_remote_storage: SteamRemoteStorage,
  steam_screenshots: SteamScreenshots,
  steam_game_search: SteamGameSearch,
}

impl SteamClient {
  pub fn new() -> SteamClient {
    debug!("Init new SteamClient");
    SteamClient {
      vtable: methods::get_vtable(),
      test: 48879,
      server_init: false,
      user_logged_in: false,

      steam_user: SteamUser::new(),
      steam_friends: SteamFriends::new(),
      steam_utils: SteamUtils::new(),
      steam_matchmaking: SteamMatchmaking::new(),
      steam_matchmaking_servers: SteamMatchmakingServers::new(),
      steam_user_stats: SteamUserStats::new(),
      steam_game_server_stats: SteamGameServerStats::new(),
      steam_input: SteamInput::new(),
      steam_apps: SteamApps::new(),
      steam_networking: SteamNetworking::new(),
      steam_remote_storage: SteamRemoteStorage::new(),
      steam_screenshots: SteamScreenshots::new(),
      steam_game_search: SteamGameSearch::new(),
      
    }
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