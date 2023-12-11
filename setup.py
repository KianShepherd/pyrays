"""
PyRays setup.

Requires cargo nightly.
"""

import os
from pathlib import Path

from setuptools import setup

from setuptools_rust import Binding, RustExtension


os.environ['RUSTFLAGS'] = '-C opt-level=3 -C target-cpu=native -C target-feature=+adx,+aes,+avx,+avx2,+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+sse4a,avx512bf16,+avx512bitalg,+avx512bw,+avx512cd,+avx512dq,+avx512f,+avx512ifma,+avx512vbmi,+avx512vbmi2,+avx512vl,+avx512vnni,+avx512vpopcntdq'

setup(
    name="pyrays",
    version="1.0.0",
    rust_extensions=[RustExtension("pyrays.pyrays_rs", binding=Binding.PyO3)],
    packages=["pyrays"],
    package_dir={'pyrays': str(Path('src/pyrays'))},
    zip_safe=False,
)
