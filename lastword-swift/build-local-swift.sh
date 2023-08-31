pushd ../lastword-ffi/

rustup install nightly-x86_64-apple-darwin
rustup component add rust-src --toolchain nightly-x86_64-apple-darwin
rustup target add aarch64-apple-ios x86_64-apple-ios
rustup target add aarch64-apple-ios-sim --toolchain nightly

cargo run --features uniffi/cli --bin uniffi-bindgen generate src/lastword.udl --language swift --out-dir ../lastword-swift/Sources/Lastword --no-format

cargo build --package lastword-ffi --features uniffi/cli --profile release-smaller --target x86_64-apple-ios
cargo build --package lastword-ffi --features uniffi/cli --profile release-smaller --target aarch64-apple-ios
cargo +nightly build --package lastword-ffi --features uniffi/cli --release -Z build-std --target aarch64-apple-ios-sim

mkdir -p target/lipo-ios-sim/release-smaller
lipo target/aarch64-apple-ios-sim/release/liblastwordffi.a target/x86_64-apple-ios/release-smaller/liblastwordffi.a -create -output target/lipo-ios-sim/release-smaller/liblastwordffi.a

popd

mv Sources/Lastword/lastword.swift Sources/Lastword/Lastword.swift
cp Sources/Lastword/lastwordFFI.h lastwordFFI.xcframework/ios-arm64/lastwordFFI.framework/Headers
cp Sources/Lastword/lastwordFFI.h lastwordFFI.xcframework/ios-arm64_x86_64-simulator/lastwordFFI.framework/Headers
cp ../lastword-ffi/target/aarch64-apple-ios/release-smaller/liblastwordffi.a lastwordFFI.xcframework/ios-arm64/lastwordFFI.framework/lastwordFFI
cp ../lastword-ffi/target/lipo-ios-sim/release-smaller/liblastwordffi.a lastwordFFI.xcframework/ios-arm64_x86_64-simulator/lastwordFFI.framework/lastwordFFI
rm Sources/Lastword/lastwordFFI.h
rm Sources/Lastword/lastwordFFI.modulemap
