#!/bin/bash

rm frames/*
rm frames.zip
set -e
RUSTFLAGS="-C target-cpu=native" cargo run --release
mv frame_* frames
mkdir -p frames
zip -r frames.zip frames
ffmpeg -y -framerate 15 -pattern_type glob -i 'frames/*.png' -c:v libx264 -pix_fmt yuv420p out.mp4
