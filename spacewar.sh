cargo build --lib --target=i686-pc-windows-msvc
echo "Built new dll."

cp target/i686-pc-windows-msvc/debug/steam_emu.dll spacewar/steam_api.dll
cp target/i686-pc-windows-msvc/debug/steam_emu.pdb spacewar/steam_api.pdb


echo "Patched spacewar 'steam_api.dll'"
echo "Running spacewar..."
spacewar/SteamworksExample.exe