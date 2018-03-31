#!/usr/bin/env bash

echo "OS name: ${TRAVIS_OS_NAME}"

if [ "${$TRAVIS_OS_NAME}" = "osx" ]; then
    echo "Updating rustup"
    rustup self update

    echo "Setting up build target: ${TARGET}"
    rustup target add ${TARGET}
fi

echo "Installing cross"
cargo install cross
