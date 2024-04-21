use std::{ffi::c_void, os::raw::{c_int, c_char}, collections::HashMap, ptr, sync::mpsc::{Sender, Receiver}};

use rand::Rng;
use tracing::{debug, error};
use vtables::VTable;
use vtables_derive::VTable;

use core::ops::Deref;

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
  fn get_type(&self) -> CallbackType;
}



#[repr(C)]
#[derive(Debug, Clone)]
#[derive(Default)]
struct CallbackEntry {
  callbacks: Vec<CCallbackBase>,
  results: Vec<Vec<u8>>,
}

impl CallbackEntry {
  pub fn new() -> Self {
    Self { callbacks: Vec::new(), results: Vec::new() }
  }
}

pub const DEFAULT_CB_TIMEOUT: f64 = 0.002;

#[derive(Clone, Debug)]
pub struct CBExecuteTask {
  pub cb_type: CallbackType,
  pub result: Vec<u8>,
  pub timeout: f64,
  pub dont_post_if_already: bool,
}
impl CBExecuteTask {
  pub const fn builder() -> CBExecuteTaskBuilder {
    CBExecuteTaskBuilder::new()
  }
}

pub struct CBExecuteTaskBuilder {
  cb_type: Option<CallbackType>,
  result: Option<Vec<u8>>,
  timeout: f64,
  dont_post_if_already: bool
}
impl CBExecuteTaskBuilder {
  pub const fn new() -> Self {
    Self {
      cb_type: None,
      result: None,
      timeout: DEFAULT_CB_TIMEOUT,
      dont_post_if_already: false,
    }
  }

  pub fn with_result(&mut self, result: Vec<u8>) {
    self.result = Some(result);
  }

  pub fn with_timeout(&mut self, timeout: f64) {
    self.timeout = timeout;
  }

  /// Post even if callback has already been posted.
  pub fn force_post(&mut self) {
    self.dont_post_if_already = true;
  }

  pub fn build(self) -> CBExecuteTask {
    CBExecuteTask { cb_type: self.cb_type.unwrap(), result: self.result.unwrap(), timeout: self.timeout, dont_post_if_already: self.dont_post_if_already }
  }
}

#[repr(C)]
#[derive(Debug)]
pub struct SteamCallbacks {
  pub results: CallbackResults,

  callbacks: HashMap<CallbackType, CallbackEntry>,
  rx: Receiver<CBExecuteTask>,
}

impl SteamCallbacks {
  pub fn new(rx: Receiver<CBExecuteTask>) -> Self {
    Self {
      results: CallbackResults::new(),

      callbacks: HashMap::new(),
      rx,
    }
  }

  pub fn add_callback(&mut self, cb_type: CallbackType, callback: *mut CCallbackBase) {
    debug!("adding callback: {}", cb_type);
    debug!(?callback);
    let mut cb = unsafe {*callback};
    debug!("fetched type: {}", cb.get_callback_type());

    // let res: c_int = callback_s.get_callback_type(); 5;
    if cb_type == (CallbackCategories::AppsCallbacks as c_int) + 5 { // FIXME: cb_type == SteamAPICallCompleted_t::k_iCallback
      debug!("cb_type is SteamAPICallCompleted_t");
      self.results.add_call_completed(cb);
      cb.set_register(cb_type);
      return;
    }

    if !self.callback_exists(cb_type, cb) {
      debug!("cb does not exist");
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

  pub fn pull_execute_tasks(&mut self) {
    let msgs: Vec<CBExecuteTask> = self.rx.try_iter().collect();
    for msg in msgs {
      self.add_callback_result(
        msg.cb_type,
        msg.result,
        msg.timeout,
        msg.dont_post_if_already,
      );
    }
  }

  pub fn run_callbacks(&mut self) {
    self.callbacks.iter_mut().for_each(|entry| entry.1.results.clear());
  }

  fn callback_exists(&mut self, cb_type: CallbackType, cb: CCallbackBase) -> bool {
    if let Some(v) = self.callbacks.get(&cb_type) {
      return v.callbacks.contains(&cb);
    }
    false
  }


  pub fn add_callback_result(
    &mut self,
    cb_type: CallbackType,
    result: Vec<u8>,
    timeout: f64,
    dont_post_if_already: bool,
  ) {
    let cb = self.callbacks.get_mut(&cb_type);
    if cb.is_none() {
      error!("could not find CallbackEntry for cb_type: {}", cb_type);
      return;
    }
    let cb = cb.unwrap();
    if dont_post_if_already {
      if cb.results.iter().filter(|o| o.len() == result.len()).find(|o| **o == result).is_some() {
        return; // cb already posted
      }
    }

    for cb in &cb.callbacks {
      let api_id = self.results.add_call_result(None, cb_type, &result, Some(timeout), Some(false));
      self.results.add_call_back(api_id, *cb);
    }

    if cb.callbacks.is_empty() {
      self.results.add_call_result(None, cb_type, &result, Some(timeout), Some(false));
    }
    cb.results.push(result);
  }

}