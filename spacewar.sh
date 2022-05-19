cargo +nightly build --lib --target=i686-pc-windows-msvc
echo "Built new dll."

cp target/i686-pc-windows-msvc/debug/steam_emu.dll spacewar/steam_api.dll
cp target/i686-pc-windows-msvc/debug/steam_emu.pdb spacewar/steam_api.pdb

cp target/i686-pc-windows-msvc/debug/steam_emu.dll sdk/redistributable_bin/steam_api.dll
cp target/i686-pc-windows-msvc/debug/steam_emu.pdb sdk/redistributable_bin/steam_api.pdb
cp target/i686-pc-windows-msvc/debug/steam_emu.pdb sdk/redistributable_bin/steam_emu.pdb


echo "Patched spacewar 'steam_api.dll'"
# echo "Running spacewar..."
# spacewar/SteamworksExample.exe