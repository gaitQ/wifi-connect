#!/usr/bin/env bash

# Function to check internet connectivity
function check_internet() {
    wget --spider http://google.com 2>&1
    return $?
}

while true; do
    check_internet

    if [ $? -eq 0 ]; then
        printf 'Internet connection is available. Skipping WiFi Connect.\n'
    else
        printf 'Internet connection is unavailable. Starting WiFi Connect.\n'
        ./wifi-connect --portal-ssid $(hostname)
    fi

    # Sleep for a defined interval before rechecking; adjust the sleep duration as needed
    # echo "Waiting for the next check..."
    sleep 10
done
