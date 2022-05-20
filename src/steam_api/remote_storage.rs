use std::{os::raw::c_char, ffi::c_void, ptr};
use tracing::{info, debug, error};

use vtables::VTable;
use vtables_derive::{VTable, has_vtable};

use crate::{int32, uint64};

#[has_vtable]
#[derive(VTable, Debug)]
pub struct SteamRemoteStorage {
}

impl SteamRemoteStorage {
  pub fn new() -> Self {
    Self { vtable: get_vtable() }
  }
}

pub extern "fastcall" fn GetFileCount(
  self_: *mut SteamRemoteStorage,
  _edx: *mut c_void,
) -> int32 {
  0
}

pub unsafe extern "fastcall" fn GetQuota(
  self_: *mut SteamRemoteStorage,
  _edx: *mut c_void,
  total_bytes: *mut uint64,
  available_bytes: *mut uint64
) -> bool {
  *total_bytes = 0;
  *available_bytes = 0;
  true
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 24] = [
      ptr::null_mut(), // FileWrite
      ptr::null_mut(), // FileRead
      
      ptr::null_mut(), // FileWriteAsync
      ptr::null_mut(), // FileReadAsync
      ptr::null_mut(), // FileReadAsyncComplete

      ptr::null_mut(), // FileForget
      ptr::null_mut(), // FileDelete
      ptr::null_mut(), // FileShare
      ptr::null_mut(), // SetSyncPlatforms

      ptr::null_mut(), // FileWriteStreamOpen
      ptr::null_mut(), // FileWriteStreamWriteChunk
      ptr::null_mut(), // FileWriteSreamClose
      ptr::null_mut(), // FileWriteStreamCancel

      ptr::null_mut(), // FileExists
      ptr::null_mut(), // FilePersisted
      ptr::null_mut(), // GetFileSize
      ptr::null_mut(), // GetFileTimestamp
      ptr::null_mut(), // GetSyncPlatforms

      GetFileCount as _, // GetFileCount
      ptr::null_mut(), // GetFileNameAndSize

      GetQuota as _, // GetQuota
      ptr::null_mut(), // IsCloudEnabledForAccount
      ptr::null_mut(), // IsCloudEnabledForApp
      ptr::null_mut(), // SetCloudEnabledForApp
    ];
    VTABLE.as_mut_ptr()
  }
}