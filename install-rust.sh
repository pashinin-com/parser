#!/bin/bash
set -e -x
curl https://static.rust-lang.org/rustup.sh > /tmp/rustup.sh
chmod +x /tmp/rustup.sh
/tmp/rustup.sh -y --disable-sudo --channel=$1
