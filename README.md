# Parser

[![Build Status](https://travis-ci.org/pashinin-com/parser.png?branch=master)](https://travis-ci.org/pashinin-com/parser)

## Install

```bash
curl -sSf https://static.rust-lang.org/rustup.sh | sh
sudo -H -u www-data mkdir -p /var/www/parser
sudo -H -u www-data git clone https://github.com/pashinin-com/parser.git /var/www/parser/initial
cd /var/www/parser/initial
sudo -H -u www-data make rd
ln -s /var/www/parser/initial/target/release/libparser.so /var/www/pashinin.com/initial/tmp/ve/lib/python3.5/
```

## Description

This is a Python 3.5 module (.so file). The main file is lib.rs where we
define `PyInit_` function.

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
