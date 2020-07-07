#!/bin/sh
set -euv

export RUST_BACKTRACE=1
export QUICKCHECK_GENERATOR_SIZE=500

# Run tests
cargo test
oj-verify run ./examples/*.rs

# Generate a documentation
cargo +stable doc
cat <<EOF >./target/doc/index.html
<!DOCTYPE html>
<meta charset="utf-8">
<meta http-equiv="refresh" content="0; url=spella/index.html">
<title></title>
EOF
