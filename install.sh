#!/bin/zsh

set -euo pipefail

TARGET="thumbv8m.main-none-eabihf"
BIN="iot"
BIN="wf"
ELF="target/${TARGET}/release/${BIN}"
UF2="${BIN}.uf2"
MOUNT_POINT="/tmp/rp2350"

echo "========================================"
echo " B3C RP2350 installer"
echo "========================================"

#
# 1. Build
#
echo
echo "[1/5] Building..."

cargo build \
    --bin "${BIN}" \
    --release \
    --target "${TARGET}"

#
# 2. ELF -> UF2
#
echo
echo "[2/5] Creating UF2..."

picotool uf2 convert \
    -t elf \
    "${ELF}" \
    "${UF2}"

if [[ ! -f "${UF2}" ]]; then
    echo "ERROR: ${UF2} was not created."
    exit 1
fi

#
# 3. Find RP2350
#
echo
echo "[3/5] Looking for RP2350..."

DEVICE=$(
    diskutil list external |
    awk '
        /Windows_FAT_16[[:space:]]+RP2350/ {
            print "/dev/" $NF
            exit
        }
    '
)

if [[ -z "${DEVICE}" ]]; then
    echo
    echo "ERROR: RP2350 boot volume not found."
    echo
    echo "Hold BOOTSEL while connecting the Pico."
    echo
    echo "Detected external disks:"
    diskutil list external
    exit 1
fi

echo "Found: ${DEVICE}"

#
# 4. Mount manually
#
echo
echo "[4/5] Mounting ${DEVICE}..."

sudo mkdir -p "${MOUNT_POINT}"

# Clean up a previous manual mount.
if mount | grep -q "on ${MOUNT_POINT} "; then
    echo "Unmounting previous ${MOUNT_POINT}..."
    sudo umount "${MOUNT_POINT}"
fi

sudo mount_msdos "${DEVICE}" "${MOUNT_POINT}"

#
# Sanity check:
# A real RP2350 BOOTSEL filesystem must contain INFO_UF2.TXT.
#
if [[ ! -f "${MOUNT_POINT}/INFO_UF2.TXT" ]]; then
    echo
    echo "ERROR: ${DEVICE} does not look like an RP2350 BOOTSEL volume."
    echo "INFO_UF2.TXT was not found."
    echo
    echo "NOT COPYING ANYTHING."

    sudo umount "${MOUNT_POINT}" || true
    exit 1
fi

echo
echo "RP2350 boot volume verified:"
cat "${MOUNT_POINT}/INFO_UF2.TXT"

#
# 5. Flash
#
echo
echo "[5/5] Flashing ${UF2}..."

cp "${UF2}" "${MOUNT_POINT}/"

#
# UF2 bootloader normally disconnects/reboots immediately after
# receiving the file, so cp succeeding is enough here.
#
sync || true

echo
echo "========================================"
echo " Flash complete."
echo " RP2350 should now reboot."
echo "========================================"
