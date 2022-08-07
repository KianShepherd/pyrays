cd ./raytrace-rs
git pull
cd ..
cargo build --release --lib --manifest-path=raytrace-rs/Cargo.toml
cp ./raytrace-rs/target/release/libraytracing.rlib ./src/libraytracing.rlib
python -m pip install -e .
