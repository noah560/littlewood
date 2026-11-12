#!/bin/bash

rm frames/*
rm frames.zip
set -e
cargo run --release
mv frame_* frames
mkdir -p frames
ffmpeg -y -framerate 15 -pattern_type glob -i 'frames/*.png' -c:v libx264 -pix_fmt yuv420p out.mp4
zip -r frames.zip frames
