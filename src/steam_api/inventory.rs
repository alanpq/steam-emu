use std::{os::raw::c_char, ffi::c_void, ptr};
use tracing::{info, debug, error};

use vtables::VTable;
use vtables_derive::{VTable, has_vtable};

use super::SteamAPICall_t;
use crate::{uint32, int32};

pub type SteamInventoryResult_t = int32;

#[has_vtable]
#[derive(VTable, Debug)]
pub struct SteamInventory {
}

impl SteamInventory {
  pub fn new() -> Self {
    Self { vtable: get_vtable() }
  }
}

// virtual bool GetAllItems( SteamInventoryResult_t *pResultHandle ) = 0;

pub extern "fastcall" fn GetAllItems(
  self_: *mut SteamInventory,
  _edx: *mut c_void,
  result_handle: *mut SteamInventoryResult_t,
) -> bool {
  false
}

pub extern "fastcall" fn GenerateItems(
  self_: *mut SteamInventory,
  _edx: *mut c_void,
  result_handle: *mut c_void,
  array_item_defs: *mut c_void,
  array_quantity: *mut c_void,
  array_length: uint32,
) -> bool {
  false
}

pub extern "fastcall" fn GrantPromoItems(
  self_: *mut SteamInventory,
  _edx: *mut c_void,
  result_handle: *mut c_void,
) -> bool {
  false
}

pub extern "fastcall" fn LoadItemDefinitions(
  self_: *mut SteamInventory,
  _edx: *mut c_void,
) -> bool {
  false
}

pub extern "fastcall" fn RequestPrices(
  self_: *mut SteamInventory,
  _edx: *mut c_void,
) -> SteamAPICall_t {
  0
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 27] = [
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      GetAllItems as _, // GetAllItems
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      GenerateItems as _, // GenerateItems
      GrantPromoItems as _, // GrantPromoItems
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      LoadItemDefinitions as _, // LoadItemDefinitions
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      RequestPrices as _, // RequestPrices
      // ...
    ];
    VTABLE.as_mut_ptr()
  }
}