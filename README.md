# parser-rust

[![Build Status](https://travis-ci.org/pashinin-com/parser-rust.png?branch=master)](https://travis-ci.org/pashinin-com/parser-rust)

## Description

This is a Python 3.5 module (.so file). The main file is lib.rs where we
define `PyInit_` function.

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
