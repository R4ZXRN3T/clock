Remove-Item -Recurse -Force ".\final\clock-1.0.0-Windows" -ErrorAction SilentlyContinue
rustup component add rust-src --toolchain nightly
$env:RUSTFLAGS = "-Zlocation-detail=none -Zfmt-debug=none"
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=optimize_for_size --release
New-Item -ItemType Directory -Force ".\final\clock-1.0.0-Windows" | Out-Null
Move-Item ".\target\release\clock.exe" ".\final\clock-1.0.0-Windows\clock.exe"

Push-Location ".\final\clock-1.0.0-Windows"
Pop-Location
Remove-Item -Recurse -Force ".\target" -ErrorAction SilentlyContinue
