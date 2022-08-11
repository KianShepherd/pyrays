pip install .[all]
pytest --cov=pyrays > pytest.txt
cd src/raytrace-rs
cargo tarpaulin -v -t 9999 > ../../paulin.txt
cd ../..
python coverage_calc.py
rm -f pytest.txt
rm -f paulin.txt
