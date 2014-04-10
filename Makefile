SSL_LIB ?= ../rust-openssl/build
SSL_CFG ?= --cfg openssl
HTTP_LIB ?= ../rust-http/build
RUSTC ?= rustc
RUSTDOC ?= rustdoc
RUSTPKG ?= rustpkg
RUSTFLAGS ?= -O -L build -L $(SSL_LIB) -L $(HTTP_LIB) $(SSL_CFG)
RUST_REPOSITORY ?= ../rust
RUST_CTAGS ?= $(RUST_REPOSITORY)/src/etc/ctags.rust
VERSION=0.1-pre

libcouchdb_so=build/.libcouchdb.timestamp

couchdb: $(libcouchdb_so)

$(libcouchdb_so):
	mkdir -p build/
	$(RUSTC) $(RUSTFLAGS) src/lib.rs --out-dir=build
	@touch build/.libcouchdb.timestamp

test: $(libcouchdb_so)
	$(RUSTC) $(RUSTFLAGS) --test src/test.rs --out-dir=build
	build/test

clean:
	rm -r build

all: http examples docs

TAGS:
	ctags -f TAGS --options=$(RUST_CTAGS) -R src

.PHONY: all http examples docs clean check quickcheck
