#!/bin/bash
set -euo pipefail

crates=(desklift_command)
for crate in ${crates[@]}; do
    cd $crate
    cargo test --target x86_64-unknown-linux-gnu
    cd -
done
