#!/bin/sh

set -euxo pipefail

git_status_clean="$(set +e; git diff-index --quiet HEAD; echo $?)"

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

if [ "${git_status_clean}" == "0" ]; then
    git commit -pm "Version $1 release"
fi
