$PKGNAME = "clock"
$PKGVERSION = (cargo metadata --format-version 1 | jq -r ".packages[] | select(.name == `"$PKGNAME`") | .version")
$ARCH = $env:PROCESSOR_ARCHITECTURE

Remove-Item -Path ".\final\$PKGNAME-$PKGVERSION-Windows-$ARCH" -Recurse -Force -ErrorAction SilentlyContinue

rustup component add rust-src --toolchain nightly

$env:RUSTFLAGS = "-Zlocation-detail=none -Zfmt-debug=none"
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features="optimize_for_size" --release

New-Item -ItemType Directory -Path ".\final\$PKGNAME-$PKGVERSION-Windows-$ARCH" -Force | Out-Null
Move-Item -Path ".\target\release\$PKGNAME.exe" -Destination ".\final\$PKGNAME-$PKGVERSION-Windows-$ARCH\$PKGNAME.exe"
Set-Location ".\final\$PKGNAME-$PKGVERSION-Windows-$ARCH"

Set-Location "..\.."