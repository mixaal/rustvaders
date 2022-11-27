cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu

# create tarball for windows
rm -f rustvaders-x86_64-pc-windows.tar.gz
cp -av sprites target/x86_64-pc-windows-gnu/release
tar czvf rustvaders-x86_64-pc-windows.tar.gz target/x86_64-pc-windows-gnu/release
rm -rf target/x86_64-pc-windows-gnu/release/sprites
