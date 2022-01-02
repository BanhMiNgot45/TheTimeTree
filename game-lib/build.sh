cargo build --release --target-dir=../game/bin
export MACOSX_CROSS_COMPILER=$HOME/macosx-cross-compiler
export C_INCLUDE_PATH=$MACOSX_CROSS_COMPILER/cross-compiler/SDK/MacOSX10.15.sdk/usr/include 
export CC=$MACOSX_CROSS_COMPILER/cross-compiler/bin/x86_64-apple-darwin14-cc 
cargo build --release --target x86_64-apple-darwin --target-dir=../game/bin
cargo build --release --target x86_64-pc-windows-gnu --target-dir=../game/bin