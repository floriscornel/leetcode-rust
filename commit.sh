#!/bin/bash
git add src
PROBLEM=$(git status | grep problems | cut -d: -f2 | head -n 1 | cut -d '/' -f3 | cut -d '.' -f1)
[ -z "$PROBLEM" ] && exit 1
git fetch --all
git pull && cargo check && cargo fmt && cargo clippy && cargo test && git commit -m "$PROBLEM" && git push