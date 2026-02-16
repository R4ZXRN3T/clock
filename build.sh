rm -rf "./final/clock-1.0.0-Linux-x86_64"

rustup component add rust-src --toolchain nightly

RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none" cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features="optimize_for_size" --release

mkdir -p "./final/clock-1.0.0-Linux-x86_64"
mv "./target/release/clock" "./final/clock-1.0.0-Linux-x86_64/clock"
cd "./final/clock-1.0.0-Linux-x86_64"
chmod +x clock
upx --ultra-brute clock
upx -t clock

cd "../.."

rm -rf "./target/"