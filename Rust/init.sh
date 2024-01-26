#!/bin/sh

for i in $(seq -w 02 20); do
    for prefix in a b; do
        filename="./src/${prefix}${i}.rs"
        echo "use proconio::input;

fn main() {

}" > $filename
    done
done