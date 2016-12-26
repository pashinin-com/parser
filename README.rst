Parser
======

.. image:: https://travis-ci.org/pashinin-com/parser.png?branch=master
    :target: https://travis-ci.org/pashinin-com/rparser

This is a Python 3.5 module (.so file). The main file is lib.rs where we
define `PyInit_` function.

Install
-------

```bash
pip install rparser
```

## Description



## Libraries used

https://github.com/dgrunwald/rust-cpython
[Nom](https://github.com/Geal/nom) - Rust parsing library. [Docs](http://rust.unhandledexpression.com/nom/)

## Commands

```bash
# generates documentation in target/doc folder
cargo doc
```

## Rust

### Install and update

To install Rust (Linux, Mac) — run this command:

```bash
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

See version of Rust:

```bash
rustc -V
rustc 1.12.0 (3191fbae9 2016-09-23)
```


### Use Rust

Build project:

```bash
# Standard (static) linking
# cargo build --verbose
cargo build

# dynamic linking
cargo rustc --release -- -C prefer-dynamic
```

To run project tests:

```bash
cargo test
```

Cargo.toml

```bash
# dylib, rlib, staticlbig
```

Apply certain traits:

```
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
```

### Cow - copy on write

## Build

After `python setup.py build_rust` ready pkg is in build/lib.linux-x86_64-2.7/
After `python3 setup.py build_rust`  ready pkg is in build/lib/
ln -sf /var/www/parser/build/lib/rparser /usr/local/lib/python3.5/rparser
