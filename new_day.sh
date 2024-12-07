#!/usr/bin/env bash
set -e
set -x

DAY_OF_MONTH=$(printf "%02d" $((`date +%d` + 2)))
DAY="day$DAY_OF_MONTH"
cp -R template $DAY
cd $DAY
sed -i "s/\"daytodo\"/\"$DAY\"/g" Cargo.toml
cargo test

rm src/data.txt
aoc d -y 2024 -d $DAY_OF_MONTH -I -i ./src/data.txt