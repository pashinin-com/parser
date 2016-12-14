import re
from setuptools import setup
from rust_setuptools import (build_rust_cmdclass, build_install_lib_cmdclass,
                             RustDistribution)


with open('rparser/__init__.py', 'r') as fd:
    version = re.search(r'^__version__\s*=\s*[\'"]([^\'"]*)[\'"]',
                        fd.read(), re.MULTILINE).group(1)

if not version:
    raise RuntimeError('Cannot find version information')


setup(
    name='rparser',
    version=version,
    description='My parser for some things',
    author='Sergey Pashinin',
    author_email='sergey@pashinin.com',
    url='https://github.com/pashinin-com/parser',
    requires=[],
    packages=['rparser'],
    distclass=RustDistribution,
    cmdclass={
        'build_rust': build_rust_cmdclass([('.', 'rparser')]),
        'install_lib': build_install_lib_cmdclass()
    },

    zip_safe=False,

    classifiers=(
        'Development Status :: 5 - Production/Stable',
        'Intended Audience :: Developers',
        'Natural Language :: English',
        'License :: OSI Approved :: Apache Software License',
        'Programming Language :: Python',
        'Programming Language :: Python :: 3.5',
        'Programming Language :: Python :: Implementation :: CPython',
    ),
)
