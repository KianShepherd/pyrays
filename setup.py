"""
Raspy setup.

Requires cargo.
"""

from setuptools import setup

from setuptools_rust import Binding, RustExtension


setup(
    name="raspy",
    version="1.0",
    rust_extensions=[RustExtension("raspy.raspy_rs", binding=Binding.PyO3)],
    packages=["raspy"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
