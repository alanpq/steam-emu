
use std::os::raw::c_char;

use crate::{uint64, uint32, EUniverse};

use bitfield::bitfield;

#[repr(C)]
#[derive(Debug)]
pub struct CSteamID {
  steamid: CSteamID_SteamID,
}

impl CSteamID {
  #[inline]
  pub fn new() -> Self {
    CSteamID { steamid: CSteamID_SteamID::new() }
  }
}

impl Into<uint64> for CSteamID {
    fn into(self) -> uint64 {
        self.steamid.0
    }
}

bitfield! {
  #[repr(C)]
  #[derive(Debug)]
  pub struct CSteamID_SteamID(uint64);
  u8, universe_, set_universe: 8,0;
  u8, account_type, set_account_type: 12, 8;
  account_inst, set_account_inst: 32, 12;
  account_id, set_account_id: 64, 32;
}

impl CSteamID_SteamID {
  #[inline]
  pub fn new() -> Self {
    Self(0)
  }

  #[inline]
  pub fn universe(&self) -> EUniverse {
    unsafe { ::std::mem::transmute(self.universe_()) }
  }
}