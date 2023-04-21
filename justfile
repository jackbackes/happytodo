install:
    cargo install --force --path .

format:
    cargo clippy --all-targets --all-features -- -D warnings

fix:
    cargo fix --allow-dirty --allow-staged

nightly:
    rm Cargo.lock && cargo clean && rustup run nightly --install cargo run .

run:
    cargo run -- .
