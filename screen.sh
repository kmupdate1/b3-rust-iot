#!/bin/zsh

BAUD=115200

echo "Waiting for RP235x USB serial..."

while true; do
    DEVICE=$(ls /dev/cu.usbmodem* 2>/dev/null | head -n 1)

    if [[ -n "$DEVICE" ]]; then
        echo "Connecting to $DEVICE..."
        screen "$DEVICE" "$BAUD"

        echo
        echo "Serial disconnected. Waiting for reconnect..."
    fi

    sleep 3
done
