#!/bin/zsh

set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "usage: $0 <version, e.g. 0.1.1>"
    exit 1
fi

VERSION="${1#v}"
TAG="v${VERSION}"
TARGET="thumbv8m.main-none-eabihf"
REPOSITORY="kmupdate1/b3-rust-iot"
FIRMWARE="firmware.bin"
MANIFEST="update-manifest.json"
export B3_FIRMWARE_VERSION="${VERSION}"

cargo build --release --target "${TARGET}" --bin wf
cargo objcopy --release --target "${TARGET}" --bin wf -- -O binary "${FIRMWARE}"

SIZE=$(stat -f%z "${FIRMWARE}")
SHA256=$(shasum -a 256 "${FIRMWARE}" | awk '{print $1}')

printf '{\n  "version": "%s",\n  "firmware_url": "https://github.com/%s/releases/download/%s/%s",\n  "size": %s,\n  "sha256": "%s"\n}\n' \
    "${VERSION}" "${REPOSITORY}" "${TAG}" "${FIRMWARE}" "${SIZE}" "${SHA256}" \
    > "${MANIFEST}"

if gh release view "${TAG}" --repo "${REPOSITORY}" >/dev/null 2>&1; then
    gh release upload "${TAG}" "${FIRMWARE}" "${MANIFEST}" --clobber --repo "${REPOSITORY}"
else
    gh release create "${TAG}" "${FIRMWARE}" "${MANIFEST}" --repo "${REPOSITORY}" --title "${TAG}" --generate-notes
fi

echo "Published ${TAG}: ${SIZE} bytes, sha256=${SHA256}"
