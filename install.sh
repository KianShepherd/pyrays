cargo build --release --manifest-path=rays/Cargo.toml
cp ./rays/target/release/librays.rlib ./src/librays.rlib
pip install -e .
