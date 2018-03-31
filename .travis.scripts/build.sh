#!/usr/bin/env bash

${RUST_BUILD_TOOL} build --target ${RUST_TARGET} --release

ls -lh target/
ls -lh target/${RUST_TARGET}/release
file target/${RUST_TARGET}/release/httpbox*
