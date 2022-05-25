use std::{collections::HashMap, sync::Mutex, os::raw::{c_char, c_int}, ffi::c_void, ptr};

use tracing::{info, debug, error};

use lazy_static::lazy_static;
use vtables::VTable;
use vtables_derive::{VTable, has_vtable};
use crate::{HSteamPipe, HSteamUser, int32, steam_api::{*, networking::*}, uint32};

mod methods;
mod get_generic_interfaces;
pub use methods::*;
pub use get_generic_interfaces::*;

const CLIENT_HSTEAMUSER: HSteamUser = 1;

#[repr(C)]
#[derive(Debug)]
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
  steamclient_server_inited: bool,
  user_logged_in: bool,

  steam_pipe_counter: HSteamPipe,
  steam_pipes: HashMap<HSteamPipe, SteamPipe>,

  pub callbacks_server: SteamCallbacks,
  pub callbacks_client: SteamCallbacks,
  pub callback_results_server: CallbackResults,
  pub callback_results_client: CallbackResults,

  // client stuff
  pub steam_user: SteamUser,
  pub steam_friends: SteamFriends,
  pub steam_utils: SteamUtils,
  pub steam_matchmaking: SteamMatchmaking,
  pub steam_matchmaking_servers: SteamMatchmakingServers,
  pub steam_user_stats: SteamUserStats,
  pub steam_apps: SteamApps,

  pub steam_networking: SteamNetworking,
  pub steam_networking_sockets: SteamNetworkingSockets,
  pub steam_networking_messages: SteamNetworkingMessages,

  pub steam_remote_storage: SteamRemoteStorage,
  pub steam_screenshots: SteamScreenshots,
  pub steam_http: SteamHTTP,
  pub steam_controller: SteamController,
  pub steam_ugc: SteamUGC,
  pub steam_app_list: SteamAppList,
  pub steam_music: SteamMusic,
  pub steam_music_remote: SteamMusicRemote,
  pub steam_html_surface: SteamHTMLSurface,
  pub steam_inventory: SteamInventory,
  pub steam_video: SteamVideo,
  pub steam_parental_settings: SteamParentalSettings,
  pub steam_game_search: SteamGameSearch,
  pub steam_input: SteamInput,
  pub steam_networking_utils: SteamNetworkingUtils,
  pub steam_parties: SteamParties,
  pub steam_remote_play: SteamRemotePlay,
  
  // gameserver stuff
  pub gs: SteamGameServer,
  pub gs_utils: SteamUtils,
  pub steam_game_server_stats: SteamGameServerStats,
  pub gs_networking: SteamNetworking,
  pub gs_http: SteamHTTP,
  pub gs_inventory: SteamInventory,
  pub gs_ugc: SteamUGC,
  pub gs_apps: SteamApps,
  pub gs_networking_sockets: SteamNetworkingSockets,
  pub gs_net_messages: SteamNetworkingMessages,
  // pub gs_game_coordinator: SteamGameCoordinator,
  // pub masterserver_updater: SteamMasterserverUpdater,
}

impl SteamClient {
  pub fn new() -> SteamClient {
    debug!("Init new SteamClient");
    SteamClient {
      vtable: methods::get_vtable(),
      test: 48879,
      server_init: false,
      steamclient_server_inited: false,
      user_logged_in: false,

      steam_pipe_counter: 1,
      steam_pipes: HashMap::new(),

      callbacks_server: SteamCallbacks::new(),
      callbacks_client: SteamCallbacks::new(),
      callback_results_server: CallbackResults::new(),
      callback_results_client: CallbackResults::new(),

      steam_app_list: SteamAppList::new(),
      steam_apps: SteamApps::new(),
      steam_controller: SteamController::new(),
      steam_friends: SteamFriends::new(),
      steam_game_search: SteamGameSearch::new(),
      steam_game_server_stats: SteamGameServerStats::new(),
      steam_html_surface: SteamHTMLSurface::new(),
      steam_http: SteamHTTP::new(),
      steam_input: SteamInput::new(),
      steam_inventory: SteamInventory::new(),
      steam_matchmaking_servers: SteamMatchmakingServers::new(),
      steam_matchmaking: SteamMatchmaking::new(),
      steam_music_remote: SteamMusicRemote::new(),
      steam_music: SteamMusic::new(),
      steam_networking_utils: SteamNetworkingUtils::new(),
      steam_networking: SteamNetworking::new(),
      steam_networking_sockets: SteamNetworkingSockets::new(),
      steam_networking_messages: SteamNetworkingMessages::new(),
      steam_parental_settings: SteamParentalSettings::new(),
      steam_parties: SteamParties::new(),
      steam_remote_play: SteamRemotePlay::new(),
      steam_remote_storage: SteamRemoteStorage::new(),
      steam_screenshots: SteamScreenshots::new(),
      steam_ugc: SteamUGC::new(),
      steam_user_stats: SteamUserStats::new(),
      steam_user: SteamUser::new(),
      steam_utils: SteamUtils::new(),
      steam_video: SteamVideo::new(),
      
      gs: SteamGameServer::new(),
      gs_utils: SteamUtils::new(),
      gs_networking: SteamNetworking::new(),
      gs_http: SteamHTTP::new(),
      gs_inventory: SteamInventory::new(),
      gs_ugc: SteamUGC::new(),
      gs_apps: SteamApps::new(),
      gs_networking_sockets: SteamNetworkingSockets::new(),
      gs_net_messages: SteamNetworkingMessages::new(),
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

  pub fn run_callbacks(&mut self, run_client_cb: bool, run_gameserver_cb: bool) {
    // mutex lock
    // join with background thread?

    // self.networking.run();
    // self.steam_matchmaking_servers.run_callbacks();
    // self.run_every_runcb.run()

    // self.gs.run_callbacks();

    if run_client_cb {
      // self.callback_results_client.run_call_results();
    }

    if run_gameserver_cb {
      // self.callback_results_server.run_call_result();
    }

    self.callbacks_server.run_callbacks();
    self.callbacks_client.run_callbacks();
    // FIXME: set last_cb_run
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

  pub fn register_callback(&mut self, callback: *mut CCallbackBase, callback_type: CallbackType) {
    let callback_s = unsafe {*callback};
    if callback_s.is_server() {
      self.callbacks_server.add_callback(callback_type, callback);
    } else {
      self.callbacks_client.add_callback(callback_type, callback);
    }
  }

}