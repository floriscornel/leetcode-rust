#!/bin/bash
cargo check
cargo fmt
cargo clippy
cargo test
git add .
git commit -m $(git status | grep "new file" | grep problems | cut -d: -f2 | head -n 1 | cut -d '/' -f3 | cut -d '.' -f1)