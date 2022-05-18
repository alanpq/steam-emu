use std::{collections::HashMap, sync::Mutex};

use tracing::{info, debug, error};

use lazy_static::lazy_static;
use crate::{HSteamPipe, HSteamUser};


const CLIENT_HSTEAMUSER: HSteamUser = 1;

pub static mut CLIENT: SteamClient = SteamClient {
  server_init: false,
  user_logged_in: false,
};

pub enum SteamPipe {
  NoUser,
  Client,
  Server
}

lazy_static! {
  static ref STEAM_PIPES: Mutex<HashMap<HSteamPipe, SteamPipe>> = Mutex::new(HashMap::new());
}


pub struct SteamClient {
  server_init: bool,
  user_logged_in: bool,
}

impl SteamClient {
  pub fn new() -> SteamClient {
    debug!("Init new SteamClient");
    SteamClient {
      server_init: false,
      user_logged_in: false,
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