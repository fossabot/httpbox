#!/usr/bin/env bash

if [ "${RUST_BUILD_TOOL}" = "cargo" ]; then
    echo "Updating rustup"
    rustup self update

    echo "Setting up build target for '${RUST_TARGET}' if necessary"
    rustup target add ${RUST_TARGET} || true
elif [ "${RUST_BUILD_TOOL}" = "cross" ]; then
    echo "Installing cross (if absent) (see https://github.com/japaric/cross)"
    cargo install --list | grep cross || cargo install cross
else
    echo "Invalid value for RUST_BUILD_TOOL='${RUST_BUILD_TOOL}': aborting!"
    exit 1
fi
