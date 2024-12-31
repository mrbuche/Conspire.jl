cd("conspire_wrapper")
run(`cargo clippy --release -- -D warnings`)
run(`cargo build --release`)
