#!/bin/zsh

set -euo pipefail

TARGET="thumbv8m.main-none-eabihf"
APPLICATION="target/${TARGET}/release/wf"

echo "Building application only..."
cargo build --release --target "${TARGET}" --bin wf

echo "Waiting for RP2350 BOOTSEL device..."
picotool info -f >/dev/null

echo "Loading application into ACTIVE partition only..."
picotool load -f -t elf "${APPLICATION}"

echo "Rebooting..."
picotool reboot -f

echo "Application installation complete."
