#!/bin/bash -x
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu

# create tarball for windows
rm -f rustvaders-x86_64-pc-windows.tar.gz
cp -av sprites target/x86_64-pc-windows-gnu/release
#cp -av  sdl-build/SDL2*/x86_64-w64-mingw32/lib/*dll* target/x86_64-pc-windows-gnu/release/


cd  target/x86_64-pc-windows-gnu/release
curl -L -kv https://libsdl.org/release/SDL2-2.26.0-win32-x64.zip --output SDL2-2.26.0-win32-x64.zip
unzip SDL2-2.26.0-win32-x64.zip
curl -L -kv https://github.com/libsdl-org/SDL_image/releases/download/release-2.6.2/SDL2_image-2.6.2-win32-x64.zip --output SDL2_image-2.6.2-win32-x64.zip
unzip SDL2_image-2.6.2-win32-x64.zip
rm -rf *zip rustvaders.d examples build deps .fingerprint inceremental .cargo-lock
cd -
tar czvf rustvaders-x86_64-pc-windows.tar.gz target/x86_64-pc-windows-gnu/release
rm -rf target/x86_64-pc-windows-gnu/release/*
