build:
  @OPENSSL_LIB_DIR=/usr/lib/openssl-1.0/ OPENSSL_INCLUDE_DIR=/usr/include/openssl-1.0/ cargo build

test:
  @OPENSSL_LIB_DIR=/usr/lib/openssl-1.0/ OPENSSL_INCLUDE_DIR=/usr/include/openssl-1.0/ cargo test --lib

bench day:
  @OPENSSL_LIB_DIR=/usr/lib/openssl-1.0/ OPENSSL_INCLUDE_DIR=/usr/include/openssl-1.0/ cargo bench --features bench --lib "day{{day}}::"

build22:
  @OPENSSL_LIB_DIR=/usr/lib/openssl-1.0/ OPENSSL_INCLUDE_DIR=/usr/include/openssl-1.0/ rustup run 1.22.1 cargo build

test22:
  @OPENSSL_LIB_DIR=/usr/lib/openssl-1.0/ OPENSSL_INCLUDE_DIR=/usr/include/openssl-1.0/ rustup run 1.22.1 cargo test

fetch day:
  @target/fetchdata {{day}}

run day part:
  @just build
  @target/debug/aoc {{day}} {{part}}

version:
  cargo version
