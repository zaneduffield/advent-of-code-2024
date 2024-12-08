#!/usr/bin/env bash

set -uex

SRC=$(dirname "${BASH_SOURCE[0]}")/src

day=$1
mod_name="day_$(printf '%02d' "$1")"

mkdir -p "$SRC/$mod_name"
sed "s|xx|$day|g" "$SRC/day_xx/mod.rs" > "$SRC/$mod_name/mod.rs"
printf "\npub mod $mod_name;" >> "$SRC/lib.rs"
sed -i -zE 's/(.*)(\n\s*println!\("done.*)/\1    run_day_with_generator!('"$mod_name, \"$day\");\n\2/" "$SRC/bin/bin.rs"

curl "https://adventofcode.com/2024/day/$day/input" --cookie "$(cat cookies)" > "input/2024/day$day.txt"