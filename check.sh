#! /bin/sh
set -x
cargo fmt --check &&
cargo clippy -- -Dwarnings &&
cargo clippy --tests -- -Dwarnings &&
# cargo test --doc &&
cargo test &&
cargo doc --no-deps --document-private-items
