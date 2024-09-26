RUSTFLAGS="-C target-feature=-a --cfg debug_assertions" cargo build --release --target riscv64imac-unknown-none-elf --features build-with-clang --example lto_bug
ckb-debugger --bin target/riscv64imac-unknown-none-elf/release/examples/lto_bug
