use std::{collections::HashMap, sync::Mutex};

use tracing::{info, debug, error};

use lazy_static::lazy_static;
use crate::HSteamPipe;

pub enum SteamPipe {
  NoUser,
  Client,
  Server
}

lazy_static! {
  static ref STEAM_PIPES: Mutex<HashMap<HSteamPipe, SteamPipe>> = Mutex::new(HashMap::new());
}


pub struct SteamClient {

}

impl SteamClient {
  pub fn new() -> SteamClient {
    debug!("Init new SteamClient");
    SteamClient {
      
    }
  }

  pub fn create_steam_pipe() -> HSteamPipe {
    debug!("CreateSteamPipe");
    let mut pipes = STEAM_PIPES.lock().unwrap();
    let pipe = pipes.len() as HSteamPipe + 1;
    pipes.insert(pipe, SteamPipe::NoUser);
    pipe
  }
}