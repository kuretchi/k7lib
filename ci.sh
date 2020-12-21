#!/bin/sh
set -euv

export RUST_BACKTRACE=1
export QUICKCHECK_TESTS=2000
# Allow to generate all values of `u8`
# See https://github.com/BurntSushi/quickcheck/blob/0.9.2/src/arbitrary.rs#L760-L762
export QUICKCHECK_GENERATOR_SIZE=255

# Run tests
cargo test
oj-verify run ./examples/*.rs

# Generate a documentation
cargo +stable doc
cat <<EOF >./target/doc/index.html
<!DOCTYPE html>
<meta charset="utf-8">
<meta http-equiv="refresh" content="0; url=k7lib/index.html">
<title></title>
EOF
