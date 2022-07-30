from shutil import copy

from distutils.command.install import install
from setuptools import setup
from setuptools_rust import Binding, RustExtension


class post_build(install):
    def run(self):
        copy('/home/kian/workspace/raspy/target/release/libraspy_rs.so', '/home/kian/workspace/raspy/raspy/raspy_rs.so')


setup(
    name="raspy",
    version="1.0",
    rust_extensions=[RustExtension("raspy.raspy_rs", binding=Binding.PyO3)],
    packages=["raspy"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
    cmdclass={'install':post_build}
)
