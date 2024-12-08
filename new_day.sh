#!/usr/bin/env bash

DAY_OF_MONTH=$(printf "%02d" $((`date +%d` + 1)))
DAY="day$DAY_OF_MONTH"
cp -R template $DAY
cd $DAY
sed -i "s/\"daytodo\"/\"$DAY\"/g" Cargo.toml
cargo test

rm src/data.txt
# Download puzzle input
aoc d -y 2024 -d $DAY_OF_MONTH -I -i ./src/data.txt
# Print puzzle
aoc