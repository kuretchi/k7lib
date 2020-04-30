#!/bin/sh
set -euv
export RUST_BACKTRACE=1
export QUICKCHECK_GENERATOR_SIZE=500

cargo test
cargo +stable doc
cat <<EOT >./target/doc/index.html
<!DOCTYPE html>
<meta charset="utf-8">
<meta http-equiv="refresh" content="0; url=spella/index.html">
<title></title>
EOT
oj-verify run ./examples/*.rs
