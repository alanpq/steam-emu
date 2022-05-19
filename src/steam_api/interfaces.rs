use std::{os::raw::c_char, ffi::{CStr, CString}};

const_cstr! {
  pub OLD_CLIENT                   = "SteamClient017";
  pub OLD_GAMESERVER               = "SteamGameServer012";
  pub OLD_GAMESERVER_STATS         = "SteamGameServerStats001";
  pub OLD_USER                     = "SteamUser018";
  pub OLD_FRIENDS                  = "SteamFriends015";
  pub OLD_UTILS                    = "SteamUtils007";
  pub OLD_MATCHMAKING              = "SteamMatchMaking009";
  pub OLD_MATCHMAKING_SERVERS      = "SteamMatchMakingServers002";
  pub OLD_USERSTATS                = "STEAMUSERSTATS_INTERFACE_VERSION011";
  pub OLD_APPS                     = "STEAMAPPS_INTERFACE_VERSION007";
  pub OLD_NETWORKING               = "SteamNetworking005";
  pub OLD_REMOTE_STORAGE_INTERFACE = "STEAMREMOTESTORAGE_INTERFACE_VERSION013";
  pub OLD_SCREENSHOTS              = "STEAMSCREENSHOTS_INTERFACE_VERSION002";
  pub OLD_HTTP                     = "STEAMHTTP_INTERFACE_VERSION002";
  pub OLD_UNIFIED_MESSAGES         = "STEAMUNIFIEDMESSAGES_INTERFACE_VERSION001";
  pub OLD_CONTROLLER               = "SteamController003";
  pub OLD_UGC_INTERFACE            = "STEAMUGC_INTERFACE_VERSION007";
  pub OLD_APPLIST                  = "STEAMAPPLIST_INTERFACE_VERSION001";
  pub OLD_MUSIC                    = "STEAMMUSIC_INTERFACE_VERSION001";
  pub OLD_MUSIC_REMOTE             = "STEAMMUSICREMOTE_INTERFACE_VERSION001";
  pub OLD_HTML_SURFACE             = "STEAMHTMLSURFACE_INTERFACE_VERSION_003";
  pub OLD_INVENTORY                = "STEAMINVENTORY_INTERFACE_V001";
  pub OLD_VIDEO                    = "STEAMVIDEO_INTERFACE_V001";
  pub OLD_MASTERSERVER_UPDATER     = "SteamMasterServerUpdater001";
}
