use std::{os::raw::{c_char, c_double, c_uint, c_int}, time::Instant, ffi::c_void, ptr};

use tracing::{error, debug, span, Level};

use crate::uint32;
use crate::steam_api::callbacks::callbacks::SteamAPICallImpl;

use super::{CCallbackBase, SteamAPICall_t, CallbackType};
use serde::{Serialize, Deserialize};


const STEAM_CALLRESULT_TIMEOUT: f64 = 120.0;
const STEAM_CALLRESULT_WAIT_FOR_CB: f64 = 0.01;
const DEFAULT_CB_TIMEOUT: f64 = 0.002;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SteamCallResult {
  api_call: SteamAPICall_t,
  callbacks: Vec<CCallbackBase>,
  result: Vec<u8>,
  to_delete: bool,
  reserved: bool,
  created: Instant,
  run_in: f64,
  run_call_completed_cb: bool,
  callback_type: CallbackType,
}

#[inline]
fn did_timeout(then: Instant, timeout_secs: f64) -> bool {
  if timeout_secs <= 0.0 {return true;}
  Instant::now().duration_since(then).as_secs_f64() > timeout_secs
}

impl SteamCallResult {
  pub fn new(call: SteamAPICall_t, icb: CallbackType, result: Vec<u8>, r_in: f64, run_cc_cb: bool) -> Self {
    Self {
      api_call: call,
      callbacks: Vec::new(),
      result,
      to_delete: false,
      reserved: false,
      created: Instant::now(),
      run_in: r_in,
      run_call_completed_cb: run_cc_cb,
      callback_type: icb,
    }
  }

  pub fn has_cb(&self) -> bool {
    self.callbacks.len() > 0
  }
  pub fn timed_out(&self) -> bool {
    did_timeout(self.created, STEAM_CALLRESULT_TIMEOUT)
  }
  pub fn call_completed(&self) -> bool {
    !self.reserved && did_timeout(self.created, self.run_in)
  }
  pub fn can_execute(&self) -> bool {
    !self.to_delete && self.call_completed() && (self.has_cb() || did_timeout(self.created, STEAM_CALLRESULT_WAIT_FOR_CB))
  }
}

impl PartialEq for SteamCallResult {
  fn eq(&self, other: &Self) -> bool {
    self.api_call == other.api_call &&
    self.callbacks == other.callbacks
  }
}
impl Eq for SteamCallResult {}

/*
bool check_timedout(std::chrono::high_resolution_clock::time_point old, double timeout)
{
    if (timeout == 0.0) return true;

    std::chrono::high_resolution_clock::time_point now = std::chrono::high_resolution_clock::now();
    if (std::chrono::duration_cast<std::chrono::duration<double>>(now - old).count() > timeout) {
        return true;
    }

    return false;
}

 */


type CbAll = unsafe extern "C" fn(results: Vec<u8>, callback: c_int);

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CallbackResults {
  call_results: Vec<SteamCallResult>,
  completed_cbs: Vec<*mut CCallbackBase>,
  cb_all: *mut CbAll,
}

impl CallbackResults {
  pub fn new() -> Self {
    Self {
      call_results: Vec::new(),
      completed_cbs: Vec::new(),
      cb_all: ptr::null_mut(),
    }
  }

  pub fn set_cb_all(&mut self, cb_all: *mut CbAll) {
    self.cb_all = cb_all;
  }

  pub fn add_call_completed(&mut self, cb: CCallbackBase) {
    
  }

  pub fn add_call_back(&mut self, api_call: SteamAPICall_t, mut cb: CCallbackBase) {
    // panic!();
    let span = span!(Level::DEBUG, "add_call_back");
    let _enter = span.enter();
    debug!("checking for call result");
    if let Some(cb_res) = self.call_results.iter_mut().find(|o| o.api_call == api_call) {
      debug!("found call result");
      cb_res.callbacks.push(cb);
      debug!("pushed callback");
      // let mut cb = unsafe {*cb};
      debug!(?cb);
      cb.set_register(cb.get_callback_type());
      debug!("set registered flag");
    }
  }

  pub fn add_call_result(
    &mut self,
    api_call: Option<SteamAPICall_t>,
    cb_type: CallbackType,
    result: &Vec<u8>,
    timeout: Option<f64>,
    run_call_completed_cb: Option<bool>
  ) -> SteamAPICall_t{
    let api_call = api_call.unwrap_or(SteamAPICall_t::generate());
    let timeout = timeout.unwrap_or(DEFAULT_CB_TIMEOUT);
    let run_call_completed_cb = run_call_completed_cb.unwrap_or(false);

    if let Some(cb) = self.call_results.iter_mut().find(
      |o| o.api_call == api_call
    ) {
      if cb.reserved {
        let created = cb.created;
        let tmp_cbs = cb.callbacks.clone();
        *cb = SteamCallResult::new(api_call, cb_type, result.clone(), timeout, run_call_completed_cb);
        cb.callbacks = tmp_cbs;
        cb.created = created;
        return cb.api_call;
      }
    } else {
      let res = SteamCallResult::new(api_call, cb_type, result.clone(), timeout, run_call_completed_cb);
      let call = res.api_call;
      self.call_results.push(res);
      return call;
    }

    error!("Failed to add_call_result");
    0
  }

  pub fn run_call_results(&mut self) {
    debug!("run_call_results");
    let cur_size = self.call_results.len();
    for call_res in &mut self.call_results {
      if !call_res.to_delete {
        if call_res.can_execute() {
          // let res = call_res.result.clone();
          let api_call = call_res.api_call;
          let run_call_completed_cb = call_res.run_call_completed_cb;
          let cb_type = call_res.callback_type;
          if run_call_completed_cb {
            call_res.run_call_completed_cb = false;
          }

          call_res.to_delete = true;
          if call_res.has_cb() {
            for cb in &mut call_res.callbacks {
              // let mut cb = unsafe {**cb};
              let callback_type = cb.get_callback_type();
              debug!("Calling callresult {:?} {}", cb, callback_type);
              // unlock global mutex
              debug!(run_call_completed_cb);
              debug!(?call_res.result);
              if run_call_completed_cb {
                let addr = ptr::addr_of_mut!(call_res.result);
                debug!("cb.run_extra_params({:?}, ..)", addr);
                cb.run_extra_params(
                  ptr::addr_of_mut!(call_res.result) as _,
                  false,
                  api_call
                );
              } else {
                let addr = ptr::addr_of_mut!(call_res.result);
                debug!("cb.run({:?})", addr);
                cb.run(addr as _)
              }
              // relock global mutex
              debug!("callresult done");
            }
          }

          if run_call_completed_cb {
            let callbacks = self.completed_cbs.clone();
            // create SteamAPICallCompleted_t instance
            let mut data = SteamAPICallCompleted_t {
              async_call: api_call,
              callback_type: cb_type,
              cub_param: call_res.result.len() as uint32,
            };

            for cb in & self.completed_cbs {
              debug!("Call complete cb {} {:?} {}", cb_type, cb, api_call);
              let mut cb = unsafe{**cb};
              cb.run(ptr::addr_of_mut!(data) as _);
            }

            if !self.cb_all.is_null() {
              unsafe {(*self.cb_all)(call_res.result.clone(), cb_type)};
            }

          } else {
            if !self.cb_all.is_null() {
              unsafe {(*self.cb_all)(call_res.result.clone(), cb_type);}
            }
          }

        }
      }
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct SteamAPICallCompleted_t {
  async_call: SteamAPICall_t,
  callback_type: CallbackType,
  cub_param: uint32,
}