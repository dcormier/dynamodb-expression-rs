#! /bin/sh
cargo fmt --check &&
cargo clippy -- -Dwarnings &&
cargo clippy --tests -- -Dwarnings &&
cargo test &&
cargo doc
