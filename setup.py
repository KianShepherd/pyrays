"""
Raspy setup.

Requires cargo.
"""

import os

from setuptools import setup

from setuptools_rust import Binding, RustExtension


os.environ['RUSTFLAGS'] = '--extern rays=src/librays.rlib'


setup(
    name="raspy",
    version="1.0",
    rust_extensions=[RustExtension("raspy.raspy_rs", binding=Binding.PyO3)],
    packages=["raspy"],
    zip_safe=False,
)
