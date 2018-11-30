build:
  @OPENSSL_LIB_DIR=/usr/lib/openssl-1.0/ OPENSSL_INCLUDE_DIR=/usr/include/openssl-1.0/ rustup run 1.22.1 cargo build
