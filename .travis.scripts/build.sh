#!/usr/bin/env bash

# Error on unset variables "-u" and exit on error statement "-e"
set -ue

SCRIPT_ABS_PATH="$(cd "$(dirname "$BASH_SOURCE")" && pwd)/$(basename "$BASH_SOURCE")"
SCRIPT_DIR_ABS_PATH="$(dirname ${SCRIPT_ABS_PATH})"

source ${SCRIPT_DIR_ABS_PATH}/logging.sh

info "Building release for '${RUST_TARGET}' using '${RUST_BUILD_TOOL}'"
${RUST_BUILD_TOOL} build --target ${RUST_TARGET} --release

if [ "none" != "${RELEASE_PKG}" ]; then
    info "Packaging artifact '${ARTIFACT}' into '${RELEASE_PKG}'"
    tar -C target/${RUST_TARGET}/release/ -czvf ${RELEASE_PKG} ${ARTIFACT}

    info "Final package content"
    tar -ztvf ${RELEASE_PKG}
else
    warn "No release package for this build"
fi
