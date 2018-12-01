build:
  @OPENSSL_LIB_DIR=/usr/lib/openssl-1.0/ OPENSSL_INCLUDE_DIR=/usr/include/openssl-1.0/ cargo build

test:
  @OPENSSL_LIB_DIR=/usr/lib/openssl-1.0/ OPENSSL_INCLUDE_DIR=/usr/include/openssl-1.0/ cargo test --lib

build22:
  @rustup run 1.22.1 just build

test22:
  @rustup run 1.22.1 just test

fetch day:
  @target/fetchdata {{day}}

run day part:
  @just build
  @target/debug/aoc {{day}} {{part}}

version:
  cargo version
