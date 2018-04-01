#!/usr/bin/env bash

# Error on unset variables "-u" and exit on error statement "-e"
set -ue

SCRIPT_ABS_PATH="$(cd "$(dirname "$BASH_SOURCE")" && pwd)/$(basename "$BASH_SOURCE")"
SCRIPT_DIR_ABS_PATH="$(dirname ${SCRIPT_ABS_PATH})"

source ${SCRIPT_DIR_ABS_PATH}/logging.sh

info "Building release for '${RUST_TARGET}' using '${RUST_BUILD_TOOL}'"
${RUST_BUILD_TOOL} build --target ${RUST_TARGET} --release

info "Packaging artifact '${ARTIFACT}' into '${PKG}'"
tar -C target/${RUST_TARGET}/release/ -czvf ${PKG} ${ARTIFACT}

info "Final package content"
tar -ztvf ${PKG}
