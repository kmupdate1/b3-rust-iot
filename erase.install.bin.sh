#!/bin/zsh

set -euo pipefail

TARGET="thumbv8m.main-none-eabihf"
APPLICATION="target/${TARGET}/release/wf"

DFU_START="0x10210000"
DFU_END="0x10400000"

echo "Building application..."
cargo build --release --target "${TARGET}" --bin wf

echo "Waiting for RP2350 BOOTSEL device..."
picotool info -f >/dev/null

echo "Erasing DFU partition..."
picotool erase -f -r "${DFU_START}" "${DFU_END}"

echo "Loading application into ACTIVE partition..."
picotool load -f -t elf "${APPLICATION}"

echo "Rebooting..."
picotool reboot -f

echo "Done."
echo "  Bootloader: unchanged"
echo "  State:      unchanged"
echo "  ACTIVE:     application installed"
echo "  DFU:        erased"
