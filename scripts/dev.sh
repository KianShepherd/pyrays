./scripts/clean.sh
pip install .[all]
python tests/test_scenes/test_sphere/test_sphere_single.py
python tests/test_scenes/test_sphere/test_sphere_multi.py
python tests/test_scenes/test_triangle/test_triangle_single.py
python tests/test_scenes/test_triangle/test_triangle_multi.py
python tests/test_scenes/test_triangle/test_triangle_culled_single.py
python tests/test_scenes/test_triangle/test_triangle_culled_multi.py
pytest --cov=pyrays
pflake8
cd src/raytrace-rs
cargo clippy
cd ../..