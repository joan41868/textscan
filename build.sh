#!/usr/bin/env bash

cargo build --release
cp target/release/textscan .

echo 'Done!';