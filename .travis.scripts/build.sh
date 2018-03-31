#!/usr/bin/env bash

if [ "${TRAVIS_OS_NAME}" = "osx" ]; then
    echo "Using 'cargo' to build on '${TRAVIS_OS_NAME}'"
    BUILD_TOOL="cargo"
else
    echo "Using 'cross' to build on '${TRAVIS_OS_NAME}' (see https://github.com/japaric/cross)"
    BUILD_TOOL="cross"
fi


${BUILD_TOOL} build --target ${TARGET} --release

tree target/
file target/release/httpbox
