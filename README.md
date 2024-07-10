# steam-emu
Inspired by [Goldberg's Steam Emulator](https://gitlab.com/Mr_Goldberg/goldberg_emulator), steam-emu is a Rust-based emulator designed to replicate the [Steamworks API](https://partner.steamgames.com/doc/sdk/api) shared libraries.

> **NOTE**: This is not a complete or functional implementation and is intended solely for educational purposes.

# Building
To build the library & debug binary, run:
```
cargo build
```

# Usage
Replace your target exectuable's `steam_api(64).dll`/`libsteam_api.so` with steam-emu's.

To get debug logs from steam-emu, run the `steam_emu_debug` binary:
```
cargo run --bin steam_emu_debug
```
This will open a TCP server on `127.0.0.1:1337`, that the steam-emu library will try to connect to, when it is loaded by the target executable.
