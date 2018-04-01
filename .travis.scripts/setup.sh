#!/usr/bin/env bash

# Error on unset variables "-u" and exit on error statement "-e"
set -ue

SCRIPT_ABS_PATH="$(cd "$(dirname "$BASH_SOURCE")" && pwd)/$(basename "$BASH_SOURCE")"
SCRIPT_DIR_ABS_PATH="$(dirname ${SCRIPT_ABS_PATH})"

source ${SCRIPT_DIR_ABS_PATH}/logging.sh

if [ "${RUST_BUILD_TOOL}" = "cargo" ]; then
    info "Updating rustup"
    rustup self update

    info "Setting up build target for '${RUST_TARGET}' if necessary"
    rustup target add ${RUST_TARGET} || true
elif [ "${RUST_BUILD_TOOL}" = "cross" ]; then
    info "Installing cross (if absent) (see https://github.com/japaric/cross)"
    cargo install --list | grep cross || cargo install cross
else
    error "Invalid value for RUST_BUILD_TOOL='${RUST_BUILD_TOOL}': aborting!"
    exit 1
fi
