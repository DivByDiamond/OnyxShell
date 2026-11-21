# OnyxShell - user shell `osh` for OnyxOS (RISC-V 64, freestanding).
#
# Thin delegation, no duplicated logic. The real build steps live in
# build.sh, the host tests are plain rustc test crates (see below).
#
#   make build    compile onyx-osh and convert to build/osh.onx
#   make test     run the host unit tests (stdio, quoting)
#
# Host tests are compiled with plain rustc, not `cargo test`: the
# package builds for riscv64gc-unknown-none-elf by default
# (.cargo/config.toml), so `cargo test` drags the freestanding bin into
# a host build and fails on RISC-V inline asm. CI
# (.github/workflows/ci.yml) additionally checks the syscall ABI
# snapshot against OnyxKernel, formatting and clippy on both targets.

TEST_TMP ?= /tmp/onyx-shell-host-tests

build:
	bash build.sh

test:
	rustc --edition=2024 -D warnings --test tests/stdio.rs -o $(TEST_TMP)-stdio
	rustc --edition=2024 -D warnings --test tests/quoting/main.rs -o $(TEST_TMP)-quoting
	$(TEST_TMP)-stdio
	$(TEST_TMP)-quoting

.PHONY: build test
