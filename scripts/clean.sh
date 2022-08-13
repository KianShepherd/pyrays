rm -rf pyrays.egg-info
rm -rf dist/git = "https://github.com/KianShepherd/raytrace-rs"
rm -rf build/
rm -f Cargo.lock
rm -rf target
rm -f src/pyrays/pyrays_rs.cpython-310-x86_64-linux-gnu.so
rm -f src/pyrays/pyrays_rs.cp310-win_amd64.pyd
rm -f src/libraytracing.rlib
rm -rf raytrace-rs/target
rm -f .coverage
rm -rf coverage
rm -f tests/.coverage
rm -rf tests/.benchmarks
rm -rf tests/__pycache__
rm -rf src/pyrays/__pycache__
rm -rf .benchmarks
rm -rf .pytest_cache
rm -rf tests/test_scenes/test_sphere/__pycache__
rm -f tests/test_scenes/test_sphere/test_sphere_single.png
rm -f tests/test_scenes/test_sphere/test_sphere_multi.png
rm -rf tests/test_scenes/test_triangle/__pycache__
rm -f tests/test_scenes/test_triangle/test_triangle_single.png
rm -f tests/test_scenes/test_triangle/test_triangle_multi.png
rm -f tests/test_scenes/test_triangle/test_triangle_culled_single.png
rm -f tests/test_scenes/test_triangle/test_triangle_culled_multi.png
python -m pip uninstall Pillow -y
python -m pip uninstall pyrays -y
