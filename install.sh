cd ./Rust-Raytracing
git pull
cd ..
cargo build --release --lib --manifest-path=Rust-Raytracing/Cargo.toml
cp ./Rust-Raytracing/target/release/libraytracing.rlib ./src/libraytracing.rlib
python -m pip install -e .
