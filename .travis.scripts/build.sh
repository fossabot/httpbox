#!/usr/bin/env bash

cross build --target ${TARGET}
cross build --target ${TARGET} --release
