use std::{ffi::c_void, os::raw::{c_int, c_char}, collections::HashMap, ptr};

use rand::Rng;
use tracing::debug;
use vtables::VTable;
use vtables_derive::VTable;

use crate::{uint64, uint8, steam_api::CallbackCategories};

pub type SteamAPICall_t = uint64;
pub trait SteamAPICallImpl {
  fn generate() -> SteamAPICall_t;
}

impl SteamAPICallImpl for SteamAPICall_t {
  fn generate() -> SteamAPICall_t {
    let mut rng = rand::thread_rng();
    let mut g = rng.gen();
    g+=1;
    match g == 0 { // why was this done originally its already unsigned
      true => 1,
      false => g
    }
  }
}

use bitflags::bitflags;

use super::{CCallbackBase, CallbackResults};

bitflags! {
  #[repr(transparent)]
  pub struct CallbackFlags: u8 {
    const Registered = 0x01;
    const GameServer = 0x02;
  }
}

pub type CallbackType = c_int;

pub trait Callback {

}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(Default)]
struct CallbackEntry {
  callbacks: Vec<*mut CCallbackBase>,
  results: Vec<Vec<c_char>>,
}

impl CallbackEntry {
  pub fn new() -> Self {
    Self { callbacks: Vec::new(), results: Vec::new() }
  }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SteamCallbacks {
  callbacks: HashMap<CallbackType, CallbackEntry>,
  results: CallbackResults
}

impl SteamCallbacks {
  pub fn new() -> Self {
    Self {
      callbacks: HashMap::new(),
      results: CallbackResults::new(),
    }
  }

  pub fn add_callback(&mut self, cb_type: CallbackType, callback: *mut CCallbackBase) {
    debug!("adding callback: {}", cb_type);
    debug!(?callback);
    let cb = &mut unsafe {*callback};
    
    // let res: c_int = callback_s.get_callback_type(); 5;
    if cb_type == (CallbackCategories::AppsCallbacks as c_int) + 5 { // FIXME: cb_type == SteamAPICallCompleted_t::k_iCallback
      self.results.add_call_completed(callback);
      cb.set_register(cb_type);
      return;
    }

    if !self.callback_exists(cb_type, callback) {
      let entry = self.callbacks.entry(cb_type).or_default();
      entry.callbacks.push(cb);
      cb.set_register(cb_type);
      for res in &entry.results {
        // TODO: timeout?
        let api_id = self.results.add_call_result(None, cb_type, res, Some(0.0), Some(false));
        self.results.add_call_back(api_id, cb);
      }
    }
  }

  pub fn run_callbacks(&mut self) {
    self.callbacks.iter_mut().for_each(|entry| entry.1.results.clear());
  }

  fn callback_exists(&mut self, cb_type: CallbackType, cb: *mut CCallbackBase) -> bool {
    if let Some(v) = self.callbacks.get_mut(&cb_type) {
      return v.callbacks.contains(&cb);
    }
    false
  }
}