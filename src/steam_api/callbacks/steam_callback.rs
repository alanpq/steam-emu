use std::{ffi::c_void, ptr::{self, addr_of}};

use tracing::debug;
use vtables::VTable;
use vtables_derive::VTable;

use super::{CallbackType, CallbackFlags, SteamAPICall_t};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[derive(VTable)]
pub struct CCallbackBase {
  pub vtable: *const c_void,
  pub cb_flags: CallbackFlags,
  pub callback_type: CallbackType,
}

impl CCallbackBase {
  /// referred to as iCallback in cpp-land
  pub fn get_callback_type(&self) -> CallbackType {
    unsafe {self.get_virtual::<fn() -> CallbackType>(2)()}
  }

  pub fn run(&mut self, param: *mut c_void) {
    debug!("run {:?}", addr_of!(self.vtable));
    unsafe {self.get_virtual::<
      extern "fastcall" fn(self_: *mut CCallbackBase, _edx: *mut c_void, param: *mut c_void) // _edx: *mut c_void,
    >(0) (
      self,
      ptr::null_mut(),
      param
    );};
  }

  pub fn run_extra_params(&mut self, param: *mut c_void, io_failure: bool, api_call: SteamAPICall_t) {
    debug!("run_extra_params");
    unsafe { self.get_virtual::<
      extern "C" fn(self_: *mut CCallbackBase, *mut c_void, bool, SteamAPICall_t)>(1)(
        ptr::addr_of_mut!(*self),
        param,
        io_failure,
        api_call
      );
    };
  }

  pub fn set_register(&mut self, cb_type: CallbackType) {
    self.cb_flags |= CallbackFlags::Registered;
    self.callback_type = cb_type;
  }

  pub fn set_unregister(&mut self) {
    self.cb_flags &= !CallbackFlags::Registered;
  }

  pub fn is_server(&self) -> bool {
    self.cb_flags.contains(CallbackFlags::GameServer)
  }
}

impl PartialEq for CCallbackBase {
    fn eq(&self, other: &Self) -> bool {
        self.vtable == other.vtable && self.cb_flags == other.cb_flags && self.callback_type == other.callback_type
    }
}

impl Eq for CCallbackBase {}