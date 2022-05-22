use std::{os::raw::c_char, ffi::{c_void, CStr, CString}, ptr};
use tracing::{info, debug, error};

use vtables::VTable;
use vtables_derive::{VTable, has_vtable};

use crate::{int32, CSteamID};

#[has_vtable]
#[derive(VTable, Debug)]
pub struct SteamFriends {
}

impl SteamFriends {
  pub fn new() -> Self {
    Self { vtable: get_vtable() }
  }
}

pub extern "fastcall" fn GetFriendCount(
  self_: *mut SteamFriends,
  _edx: *mut c_void,
  friend_flags: int32,
) -> int32 {
  0
}

pub extern "fastcall" fn GetFriendByIndex(
  self_: *mut SteamFriends,
  _edx: *mut c_void,
  friend: int32,
  friend_flags: int32,
) -> *mut c_void {
  ptr::null_mut()
}

pub extern "fastcall" fn GetFriendPersonaName(
  self_: *mut SteamFriends,
  _edx: *mut c_void,
  friend_id: CSteamID,
) -> *const c_char {
  // XX: this is a memory leak
  let str = Box::leak(Box::new(CString::new("friend").unwrap()));
  str.as_ptr()
}

pub extern "fastcall" fn GetFriendsGroupCount(
  self_: *mut SteamFriends,
  _edx: *mut c_void,
) -> int32 {
  0
}

pub extern "fastcall" fn SetRichPresence(
  self_: *mut SteamFriends,
  _edx: *mut c_void,
  key: *const c_char,
  val: *const c_char
) -> bool {
  false
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 44] = [
      ptr::null_mut(), // GetPersonaName
      ptr::null_mut(), // SetPersonaName
      ptr::null_mut(), // GetPersonaState
      GetFriendCount as _, // GetFriendCount
      GetFriendByIndex as _, // GetFriendByIndex
      ptr::null_mut(), // GetFriendRelationship
      ptr::null_mut(), // GetFriendPersonaState
      GetFriendPersonaName as _, // GetFriendPersonaName
      ptr::null_mut(), // GetFriendGamePlayed
      ptr::null_mut(), // GetFriendPersonaNameHistory
      ptr::null_mut(), // GetFriendSteamLevel
      ptr::null_mut(), // GetPlayerNickname
      GetFriendsGroupCount as _, // GetFriendsGroupCount
      ptr::null_mut(), // GetFriendsGroupIDByIndex
      ptr::null_mut(), // GetFriendsGroupName
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), // 
      ptr::null_mut(), //
      SetRichPresence as _, // SetRichPresence
      // ...
    ];
    VTABLE.as_mut_ptr()
  }
}