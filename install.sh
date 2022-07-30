cargo build --release --manifest-path=rays/Cargo.toml
cp ./rays/target/release/librays.rlib ./src/librays.rlib
cargo build --release --manifest-path=Rust-Raytracing/Cargo.toml
cp ./Rust-Raytracing/target/release/libraytracing.rlib ./src/libraytracing.rlib
python -m pip install -e .
