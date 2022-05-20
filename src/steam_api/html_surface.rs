use std::{os::raw::c_char, ffi::c_void, ptr};
use tracing::{info, debug, error};

use vtables::VTable;
use vtables_derive::{VTable, has_vtable};

use crate::uint32;

#[has_vtable]
#[derive(VTable, Debug)]
pub struct SteamHTMLSurface {
}

impl SteamHTMLSurface {
  pub fn new() -> Self {
    Self { vtable: get_vtable() }
  }
}

extern "fastcall" fn return_true(
  self_: *mut SteamHTMLSurface,
  _edx: *mut c_void,
) -> bool {
  true
}

pub type HHTMLBrowser = uint32;
extern "fastcall" fn RemoveBrowser(
  self_: *mut SteamHTMLSurface,
  _edx: *mut c_void,
  browser_handle: HHTMLBrowser,
) {
}

extern "fastcall" fn SetSize(
  self_: *mut SteamHTMLSurface,
  _edx: *mut c_void,
  browser_handle: HHTMLBrowser,
  width: uint32,
  height: uint32,
) {
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 21] = [
      return_true as _, // Init
      return_true as _, // Shutdown
      
      ptr::null_mut(), // ???
      
      ptr::null_mut(), // CreateBrowser
      RemoveBrowser as _, // RemoveBrowser
      
      ptr::null_mut(), // LoadURL
      SetSize as _, // SetSize
      
      ptr::null_mut(), // StopLoad

      ptr::null_mut(), // Reload
      ptr::null_mut(), // GoBack
      ptr::null_mut(), // GoForward
      
      ptr::null_mut(), // AddHeader
      ptr::null_mut(), // ExecuteJavascript

      ptr::null_mut(), // MouseUp
      ptr::null_mut(), // MouseDown
      ptr::null_mut(), // MouseDoubleClick
      ptr::null_mut(), // MouseMove
      ptr::null_mut(), // MouseWheel

      ptr::null_mut(), // KeyDown
      ptr::null_mut(), // KeyUp
      ptr::null_mut(), // KeyChar
      // ...
    ];
    VTABLE.as_mut_ptr()
  }
}