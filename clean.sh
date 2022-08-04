rm -rf ./pyrays.egg-info
rm -rf ./dist/git = "https://github.com/KianShepherd/Rust-Raytracing"
rm -rf ./build/
rm -f Cargo.lock
rm -rf ./target
rm -f ./src/pyrays/pyrays_rs.cpython-310-x86_64-linux-gnu.so
rm -f ./src/pyrays/pyrays_rs.cp310-win_amd64.pyd
rm -f ./src/libraytracing.rlib
rm -rf ./Rust-Raytracing/target
python -m pip uninstall pyrays -y
