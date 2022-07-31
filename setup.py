"""
PyRays setup.

Requires cargo.
"""

import os

from setuptools import setup

from setuptools_rust import Binding, RustExtension


# os.environ['RUSTFLAGS'] = '--extern raytracing=/home/kian/workspace/pyrays/src/libraytracing.rlib'
os.environ['RUSTFLAGS'] = '-C opt-level=3'

setup(
    name="pyrays",
    version="1.0",
    rust_extensions=[RustExtension("pyrays.pyrays_rs", binding=Binding.PyO3)],
    packages=["pyrays"],
    zip_safe=False,
)
