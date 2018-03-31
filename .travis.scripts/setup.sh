#!/usr/bin/env bash

if [ "${TRAVIS_OS_NAME}" = "osx" ]; then
    echo "Updating rustup"
    rustup self update

    echo "Setting up build target: ${TARGET}"
    rustup target add ${TARGET}
else
    echo "Installing cross"
    cargo install cross
fi
