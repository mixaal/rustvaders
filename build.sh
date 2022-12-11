#!/bin/bash -xe
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu

export PATH="$HOME/opt/osxcross-10.10/osxcross/target/bin:$PATH"
export TARGET_CC=x86_64-apple-darwin14-clang # Why I haven't need to setup TARGET_CC for mingw cross-compilation?
$TARGET_CC --version
cargo build  --release --target x86_64-apple-darwin

export PATH="$HOME/opt/osxcross-11.3/osxcross/target/bin:$PATH"
export TARGET_CC=aarch64-apple-darwin20.4-clang # Why I haven't need to setup TARGET_CC for mingw cross-compilation?
$TARGET_CC --version
cargo build  --release --target aarch64-apple-darwin


# copy gfx and sfx into windows release target
rm -f rustvaders-x86_64-pc-windows.tar.gz
cp -av sprites target/x86_64-pc-windows-gnu/release
cp -av sfx target/x86_64-pc-windows-gnu/release

# copy gfx and sfx into apple x86 release target
rm -f rustvaders-x86_64-apple-darwin.tar.gz
cp -av sprites target/x86_64-apple-darwin/release
cp -av sfx target/x86_64-apple-darwin/release

cp $HOME/opt/SDL2-image-macos/lib/libSDL2_image-2.0.0.dylib target/x86_64-apple-darwin/release
cp $HOME/opt/SDL2-macos/lib/libSDL2-2.0.0.dylib target/x86_64-apple-darwin/release

# copy gfx and sfx into apple aarch release target
rm -f rustvaders-aarch64-apple-darwin.tar.gz
cp -av sprites target/aarch64-apple-darwin/release
cp -av sfx target/aarch64-apple-darwin/release

cp $HOME/opt/SDL2-image-macos-aarch/lib/libSDL2_image-2.0.0.dylib target/aarch64-apple-darwin/release
cp $HOME/opt/SDL2-macos-aarch/lib/libSDL2-2.0.0.dylib target/aarch64-apple-darwin/release

#cp -av  sdl-build/SDL2*/x86_64-w64-mingw32/lib/*dll* target/x86_64-pc-windows-gnu/release/


# create tarball for windows
cd  target/x86_64-pc-windows-gnu/release
curl -L -kv https://libsdl.org/release/SDL2-2.26.0-win32-x64.zip --output SDL2-2.26.0-win32-x64.zip
unzip SDL2-2.26.0-win32-x64.zip
curl -L -kv https://github.com/libsdl-org/SDL_image/releases/download/release-2.6.2/SDL2_image-2.6.2-win32-x64.zip --output SDL2_image-2.6.2-win32-x64.zip
unzip SDL2_image-2.6.2-win32-x64.zip
rm -rf *zip rustvaders.d examples build deps .fingerprint inceremental .cargo-lock
cd -
tar czvf rustvaders-x86_64-pc-windows.tar.gz target/x86_64-pc-windows-gnu/release
rm -rf target/x86_64-pc-windows-gnu/release/*

# create tarball for apple-x86
tar czvf rustvaders-x86_64-apple-darwin.tar.gz target/x86_64-apple-darwin/release


# create tarball for apple-aarch64
tar czvf rustvaders-aarch64-apple-darwin.tar.gz target/aarch64-apple-darwin/release
