import sys
import re
from setuptools import setup
from rust_setuptools import (build_rust_cmdclass, build_install_lib_cmdclass,
                             RustDistribution)
from setuptools.command.test import test as TestCommand


class PyTest(TestCommand):
    user_options = [('pytest-args=', 'a', "Arguments to pass into py.test")]

    def initialize_options(self):
        TestCommand.initialize_options(self)
        self.pytest_args = []

    def finalize_options(self):
        TestCommand.finalize_options(self)
        self.test_args = []
        self.test_suite = True

    def run_tests(self):
        import pytest

        errno = pytest.main(self.pytest_args)
        sys.exit(errno)


PY2 = sys.version_info[0] < 3
PY3 = sys.version_info[0] >= 3


# with open('rparser/__init__.py', 'r') as fd:
#     version = re.search(r'^__version__\s*=\s*[\'"]([^\'"]*)[\'"]',
#                         fd.read(), re.MULTILINE).group(1)
with open('Cargo.toml', 'r') as fd:
    version = re.search(r'^version\s*=\s*[\'"]([^\'"]*)[\'"]',
                        fd.read(), re.MULTILINE).group(1)

if not version:
    raise RuntimeError('Cannot find version information')


setup(
    name='rparser',
    version=version,
    description='My parser for some things',
    author='Sergey Pashinin',
    author_email='sergey@pashinin.com',
    url='https://github.com/pashinin-com/rparser',
    requires=[],
    packages=[  # directories to include
        # 'rparser'
    ],
    distclass=RustDistribution,
    tests_require=['nose'],
    test_suite='nose.collector',
    cmdclass={
        'build_rust': build_rust_cmdclass(
            # [('.', 'rparser')],
            [('.', '.')],
            extra_cargo_args=[
                '--features', 'py3',
            ] if PY3 else
            [
                "--features", 'py2',
                '--no-default-features'
            ]
        ),
        'install_lib': build_install_lib_cmdclass(),
        'test': PyTest,
    },

    zip_safe=False,

    # https://pypi.python.org/pypi?%3Aaction=list_classifiers
    classifiers=(
        # Development Status :: 1 - Planning
        # Development Status :: 2 - Pre-Alpha
        # Development Status :: 3 - Alpha
        # Development Status :: 4 - Beta
        # Development Status :: 5 - Production/Stable
        # Development Status :: 6 - Mature
        # Development Status :: 7 - Inactive
        'Development Status :: 2 - Pre-Alpha',
        'Intended Audience :: Developers',
        'Natural Language :: English',
        'License :: OSI Approved :: GNU General Public License v3 (GPLv3)',
        'Programming Language :: Python',
        'Programming Language :: Python :: 2',
        'Programming Language :: Python :: 2.7',
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.5',
        'Programming Language :: Python :: 3.6',
        'Programming Language :: Python :: Implementation :: CPython',
        'Programming Language :: Python :: Implementation :: PyPy',
    ),
    platforms=["Windows", "Linux", "Mac OS-X"],
    # platforms='any',
)
