#![feature(abi_thiscall)]
#![feature(c_unwind)]
pub mod steam_game_server;
pub mod internal;
pub mod steam_api;
pub mod steam_client;

mod universe;
mod steam_id;

mod macros;

pub use universe::*;
pub use steam_id::*;

#[macro_use] extern crate const_cstr;

use std::{net::TcpStream, sync::Mutex, os::raw::{c_int, c_uint, c_short, c_ushort, c_char, c_uchar, c_ulonglong}, ptr};

use steam_client::SteamClient;
use tracing::{info, debug, Level};

pub const CLIENT_HSTEAMUSER: HSteamUser = 1;
pub const SERVER_HSTEAMUSER: HSteamUser = 1;


pub static mut CLIENT: *mut SteamClient = ptr::null_mut();
pub static mut SERVER_STEAM_PIPE: *mut HSteamPipe = ptr::null_mut();

#[ctor::ctor]
fn ctor() {
  let stream = TcpStream::connect("127.0.0.1:1337").unwrap();
  tracing_subscriber::fmt().with_max_level(Level::DEBUG).with_writer(Mutex::new(stream)).init();

  let steam_client = Box::new(SteamClient::new());
  unsafe {
    CLIENT = Box::leak(steam_client);
    SERVER_STEAM_PIPE = Box::leak(Box::new(0));
  }

  info!("steam_api.dll loaded!");
}

pub type int32 = c_int;

pub type uint8 = c_uchar;
pub type uint16 = c_ushort;
pub type uint32 = c_uint;
pub type uint64 = c_ulonglong;

pub type uintp = c_ulonglong;

pub type HSteamPipe = int32;
pub type HSteamUser = int32;