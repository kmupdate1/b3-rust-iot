#!/bin/zsh

set -euo pipefail

TARGET="thumbv8m.main-none-eabihf"
BOOTLOADER="target/${TARGET}/release/b3-rp235x-bootloader"
APPLICATION="target/${TARGET}/release/wf"

echo "Building OTA bootloader and application..."
cargo build --release --target "${TARGET}" --bin b3-rp235x-bootloader
cargo build --release --target "${TARGET}" --bin wf

echo "Waiting for RP2350 BOOTSEL device..."
picotool info -f >/dev/null

echo "Loading bootloader..."
picotool load -f -t elf "${BOOTLOADER}"

echo "Loading application into ACTIVE partition..."
picotool load -f -t elf "${APPLICATION}"

echo "Rebooting..."
picotool reboot -f

echo "OTA bootstrap installation complete."
