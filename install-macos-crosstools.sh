#!/bin/bash -xe

#https://wapl.es/rust/2019/02/17/rust-cross-compile-linux-to-macos.html
sudo apt install \
    clang \
    gcc \
    g++ \
    zlib1g-dev \
    libmpc-dev \
    libmpfr-dev \
    libgmp-dev

# Add macOS Rust target
rustup target add x86_64-apple-darwin

cd $HOME
[ -d opt ] || mkdir opt


# Install crosstools unless already installed
[ -x $HOME/opt/osxcross/target/bin/x86_64-apple-darwin14-clang ] || {
  cd opt
  git clone https://github.com/tpoechtrager/osxcross
  cd osxcross
  wget -nc https://s3.dockerproject.org/darwin/v2/MacOSX10.10.sdk.tar.xz
  mv MacOSX10.10.sdk.tar.xz tarballs/
  UNATTENDED=yes OSX_VERSION_MIN=10.7 ./build.sh
  cd -
}


# Try the cross compiler out
$HOME/opt/osxcross/target/bin/x86_64-apple-darwin14-clang --version && {
  echo "Cross-tools mac os clang is working"
}

export PATH="$HOME/opt/osxcross/target/bin:$PATH"

export CXX="o64-clang++"
export CC="o64-clang"
export LIBTOOL="x86_64-apple-darwin14-libtool"
export AR="x86_64-apple-darwin14-ar"
export RANLIB="x86_64-apple-darwin14-ranlib"



# Download SDL source and cross-compile it for mac os:
cd $HOME/opt
[ -f $HOME/opt/SDL2-macos/lib/libSDL2.a ] || {
  rm -rf SDL-compilation
  mkdir SDL-compilation
  cd SDL-compilation
  curl -L -kv https://github.com/libsdl-org/SDL/archive/refs/tags/release-2.26.1.tar.gz --output release.tar.gz
  tar xzvf release.tar.gz
  cd SDL-release-2*


  ./configure --prefix=$HOME/opt/SDL2-macos --host=x86_64-darwin --disable-joystick
  make
  make install
  cd -
}

[ -f $HOME/opt/SDL2-image-macos/lib/libSDL2_image.a ] || {
  cd $HOME/opt
  rm -rf SDL-image-compilation
  mkdir SDL-image-compilation
  cd SDL-image-compilation
  curl -L -kv https://github.com/libsdl-org/SDL_image/archive/refs/tags/release-2.6.2.tar.gz --output release.tar.gz
  tar xzvf release.tar.gz
  cd SDL_image-release*


  ./configure --prefix=$HOME/opt/SDL2-image-macos --host=x86_64-apple-darwin14 --with-sdl-exec-prefix=$HOME/opt/SDL2-macos/
   make
   make install
}



MACLIB=$(rustc --target x86_64-apple-darwin  --print target-libdir)

cp -av $HOME/opt/SDL2-macos/lib/* "$MACLIB"
cp -av $HOME/opt/SDL2-image-macos/lib/* "$MACLIB"
