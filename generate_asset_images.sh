#!/bin/bash

set -euo pipefail

input_file="$1"

convert "$input_file" -resize 1024x1024 -strip assets/icon-1024.png
convert "$input_file" -resize 256x256 -strip assets/icon-256.png
convert "$input_file" -resize 192x192 -strip -alpha off assets/icon_ios_touch_192.png
convert "$input_file" -resize 512x512 -strip assets/maskable_icon_x512.png

# favicon.ico
convert "$input_file" -resize 256x256 favicon-256.png
convert "$input_file" -resize 64x64 favicon-64.png
convert "$input_file" -resize 32x32 favicon-32.png
convert "$input_file" -resize 16x16 favicon-16.png
convert favicon-256.png favicon-64.png favicon-32.png favicon-16.png assets/favicon.ico
rm favicon-256.png favicon-64.png favicon-32.png favicon-16.png
