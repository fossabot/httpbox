#!/usr/bin/env bash

echo "Travis OS: ${TRAVIS_OS_NAME}"
if [ "${TRAVIS_OS_NAME}" = "osx" ]; then
    echo "Installing dependencies via 'homebrew'"
    brew update
    brew install tree

    echo "Updating rustup"
    rustup self update

    echo "Setting up build target: ${TARGET}"
    rustup target add ${TARGET}
else
    echo "Installing dependencies via 'APT'"
    apt-get -qq update -qq
    apt-get -qqy install tree

    echo "Installing cross (if absent) (see https://github.com/japaric/cross)"
    cargo install --list | grep cross || cargo install cross
fi
