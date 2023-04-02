#!/bin/sh

set -euxo pipefail

find . \
    -not -path './target/*' \
    -type f \
    -name 'Cargo.toml' \
    -print \
    -a \
    -exec \
        sed -i -E "s/\".*?\"(  # Keep in sync)/\"$1\"\\1/g" '{}' \
    \;

cargo +stable update -v -w
