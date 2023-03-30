"""
PyRays setup.

Requires cargo.
"""

import os
from pathlib import Path

from setuptools import setup

from setuptools_rust import Binding, RustExtension


os.environ['RUSTFLAGS'] = '-C opt-level=3 -C target-cpu=native'

setup(
    name="pyrays",
    version="1.0.0",
    rust_extensions=[RustExtension("pyrays.pyrays_rs", binding=Binding.PyO3)],
    packages=["pyrays"],
    package_dir={'pyrays': str(Path('src/pyrays'))},
    zip_safe=False,
)
