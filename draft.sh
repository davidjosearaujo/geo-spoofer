#!/bin/bash

# =================================================================
# Usage: ./geo-spoofer.sh <LOCALE> <MCCMNC> <LATITUDE> <LONGITUDE>
#
# Arguments:
# 1. <LOCALE>:   Language-Country code (e.g., en-US, es-ES, ja-JP)
# 2. <MCCMNC>:   Mobile Country Code + Mobile Network Code (e.g., 310260)
# 3. <LATITUDE>: GPS Latitude (e.g., 40.7580)
# 4. <LONGITUDE>: GPS Longitude (e.g., -73.9855)
#
# Example (Spanish, Spain, Telefónica, Madrid):
# ./geo-spoofer.sh es-ES 21407 40.4168 -3.7038
# =================================================================

# 1. Define Variables and Input Validation
LOCALE_STRING="$1"
MCCMNC="$2"
GPS_LATITUDE="$3"
GPS_LONGITUDE="$4"

if [ -z "$GPS_LONGITUDE" ]; then
    echo "Usage Error: Missing arguments."
    echo "Usage: $0 <LOCALE> <MCCMNC> <LATITUDE> <LONGITUDE>"
    exit 1
fi

# --- SPLIT LOCALE STRING (ONLY WHEN REQUIRED) ---
# Extracts the individual codes for separate properties (Telephony, Language/Country fallback)
LANG_ISO=$(echo "$LOCALE_STRING" | cut -d'-' -f1)
COUNTRY_ISO=$(echo "$LOCALE_STRING" | cut -d'-' -f2 | tr '[:lower:]' '[:upper:]')

echo "--- Spoofing Android Environment ---"
echo "Target Locale: ${LOCALE_STRING}"
echo "Target Carrier: ${MCCMNC} (Country: ${COUNTRY_ISO})"
echo "Target GPS: ${GPS_LATITUDE}, ${GPS_LONGITUDE}"

# ------------------------------------------------
# --- PART 1: SPOOF LANGUAGE AND LOCALE ---
# ------------------------------------------------
echo "1. Setting System Locale..."

# Set the combined property directly using the user's input (e.g., en-US)
adb shell setprop persist.sys.locale "$LOCALE_STRING"

# Set individual properties as a compatibility/fallback mechanism
adb shell setprop persist.sys.language "$LANG_ISO"
adb shell setprop persist.sys.country "$COUNTRY_ISO"

# Critical step: Restart Zygote to apply system-wide locale changes
echo "   Restarting Zygote (Soft Reboot)..."
adb shell "setprop ctl.restart zygote"
sleep 10 # Wait for the emulator to stabilize

# ------------------------------------------------
# --- PART 2: SPOOF TELEPHONY/SIM CARRIER DATA ---
# ------------------------------------------------
echo "2. Setting Telephony Carrier Properties..."

# Set the SIM operator properties (requires the split COUNTRY_ISO code)
adb shell setprop gsm.sim.operator.iso-country "$COUNTRY_ISO"
adb shell setprop gsm.sim.operator.numeric "$MCCMNC"

# Set the connected Network properties (requires the split COUNTRY_ISO code)
adb shell setprop gsm.operator.iso-country "$COUNTRY_ISO"
adb shell setprop gsm.operator.numeric "$MCCMNC"

echo "   Telephony data set."

# ------------------------------------------------
# --- PART 3: SPOOF GPS LOCATION ---
# ------------------------------------------------
echo "3. Setting GPS Location..."

# Use the 'geo fix' command (format is <longitude> <latitude>)
adb emu geo fix "$GPS_LONGITUDE" "$GPS_LATITUDE"

echo "--- Spoofing Complete ---"

# -------------------------------------------------------------
# --- PART 4: INTERCEPTING THIRD PARTY HTTP IP GEO LOCATORS ---
# -------------------------------------------------------------
# Involves intercepting and modifying HTTP requests/responses to spoof HTTP Request-based geolocation.
# This part could require iptables rules, and modification of HTTP headers or responses.
# TODO