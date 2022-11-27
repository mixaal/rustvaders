sudo apt install gcc-mingw-w64
rustup target add x86_64-pc-windows-gnu

mkdir sdl-build
cd sdl-build

[ -f SDL2-devel-2.26.0-mingw.tar.gz ] || curl -L -kv https://github.com/libsdl-org/SDL/releases/download/release-2.26.0/SDL2-devel-2.26.0-mingw.tar.gz --output SDL2-devel-2.26.0-mingw.tar.gz
[ -f SDL2_image-devel-2.6.2-mingw.tar.gz ] || curl -L -kv https://www.libsdl.org/projects/SDL_image/release/SDL2_image-devel-2.6.2-mingw.tar.gz --output SDL2_image-devel-2.6.2-mingw.tar.gz


[ -d SDL2-2.26.0 ] || tar xzvf SDL2-devel-2.26.0-mingw.tar.gz
[ -d SDL2_image-2.6.2 ] ||  tar xzvf SDL2_image-devel-2.6.2-mingw.tar.gz

WINLIB=$(rustc --target x86_64-pc-windows-gnu  --print target-libdir)


[ -d SDL2-2.26.0 ] && {
   cd SDL2-2.26.0
   cp -av x86_64-w64-mingw32/lib/libSDL2* "$WINLIB"
   cd -
}


[ -d SDL2_image-2.6.2 ] && {
   cd SDL2_image-2.6.2
   cp -av x86_64-w64-mingw32/lib/libSDL2_image.* "$WINLIB"
   cd -
}
