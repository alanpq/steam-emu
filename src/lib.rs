use std::{net::TcpStream, sync::Mutex, os::raw::{c_int, c_uint, c_short, c_ushort, c_char, c_uchar, c_ulonglong}};

use tracing::{info, debug, Level};

pub mod steam_game_server;
pub mod steam_internal;
pub mod steam_api;

#[ctor::ctor]
fn ctor() {
  let stream = TcpStream::connect("127.0.0.1:1337").unwrap();
  tracing_subscriber::fmt().with_max_level(Level::DEBUG).with_writer(Mutex::new(stream)).init();

  info!("steam_api.dll loaded!");
}

pub type int32 = c_int;

pub type uint8 = c_uchar;
pub type uint16 = c_ushort;
pub type uint32 = c_uint;
pub type uint64 = c_ulonglong;

pub type HSteamPipe = int32;
pub type HSteamUser = int32;