use std::{os::raw::c_char, ffi::c_void, ptr};
use tracing::{info, debug, error};

use vtables::VTable;
use vtables_derive::{VTable, has_vtable};

use crate::uint64;

// type of data request, when downloading leaderboard entries
#[repr(C)]
pub enum ELeaderboardDataRequest {
	Global = 0,
	GlobalAroundUser = 1,
	Friends = 2,
	Users = 3
}

// the sort order of a leaderboard
#[repr(C)]
pub enum ELeaderboardSortMethod {
	None = 0,
	Ascending = 1,	// top-score is lowest number
	Descending = 2,	// top-score is highest number
}

// the display type (used by the Steam Community web site) for a leaderboard
#[repr(C)]
pub enum ELeaderboardDisplayType {
	None = 0, 
	Numeric = 1,			// simple numerical score
	TimeSeconds = 2,		// the score represents a time, in seconds
	TimeMilliSeconds = 3,	// the score represents a time, in milliseconds
}

#[repr(C)]
pub enum ELeaderboardUploadScoreMethod {
	None = 0,
	KeepBest = 1,	// Leaderboard will keep user's best score
	ForceUpdate = 2,	// Leaderboard will always replace score with specified
}


#[has_vtable]
#[derive(VTable, Debug)]
pub struct SteamUserStats {
}

impl SteamUserStats {
  pub fn new() -> Self {
    Self { vtable: get_vtable() }
  }
}

// FIXME: move this elsewhere
// TODO: cry (this is async related)
pub type SteamAPICall = uint64;

pub extern "fastcall" fn FindOrCreateLeaderboard(
  self_: *mut SteamUserStats,
  _edx: *mut c_void,
  leaderboard_name: *const c_char,
  leaderboard_sort_method: ELeaderboardSortMethod,
  leaderboard_display_type: ELeaderboardDisplayType,
) -> SteamAPICall {
  0
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 23] = [
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      FindOrCreateLeaderboard as _,
    ];
    VTABLE.as_mut_ptr()
  }
}