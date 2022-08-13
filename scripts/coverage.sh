pip install .[all]
pytest --cov=pyrays > tests/cov/pytest.txt
cd src/raytrace-rs
cargo tarpaulin -v -t 9999 > ../../tests/cov/paulin.txt
cd ../..
python tests/cov/coverage_calc.py
rm -f tests/cov/pytest.txt
rm -f tests/cov/paulin.txt
