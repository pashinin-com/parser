.. rparser documentation master file, created by
   sphinx-quickstart on Wed Dec 14 20:46:13 2016.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

Rust notes and help
===================

Install Rust and update
-----------------------

For Linux & Mac:

.. code-block:: bash

   curl -sSf https://static.rust-lang.org/rustup.sh | sh


See version of Rust:

.. code-block:: bash

   rustc -V
   rustc 1.12.0 (3191fbae9 2016-09-23)


Cargo
-----

Build lib (only Rust code):

.. code-block:: bash

   # Standard (static) linking
   # cargo build --verbose
   cargo build

   # dynamic linking
   cargo rustc --release -- -C prefer-dynamic


To run Rust tests:

.. code-block:: bash

   cargo test


Cargo.toml

# dylib, rlib, staticlbig


Apply certain traits:

.. code-block:: rust

   #[derive(Debug)]
   #[derive(PartialEq, PartialOrd)]


Libraries used
--------------

* https://github.com/dgrunwald/rust-cpython
* `Nom <https://github.com/Geal/nom>` - Rust parsing library.
  `Docs <http://rust.unhandledexpression.com/nom/>`



.. code-block:: bash

   # generates documentation in target/doc folder
   cargo doc




Cow - copy on write



Lib notes
---------

The main file is lib.rs where we define `PyInit_` function.

## Build

After `python setup.py build_rust` ready pkg is in build/lib.linux-x86_64-2.7/
After `python3 setup.py build_rust`  ready pkg is in build/lib/
ln -sf /var/www/parser/build/lib/rparser /usr/local/lib/python3.5/rparser
