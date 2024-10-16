#!/usr/bin/env bash

OS=$(uname -o | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m | tr '[:upper:]' '[:lower:]')
VERSION=$(cat .release_version)

if [[ -z $VERSION ]]; then
    echo "No version found, using commit hash"
    VERSION=$(git rev-parse --short HEAD)
    exit 1
fi

filename="sidecar-calculations-${VERSION}-${OS}-${ARCH}.tar.gz"

echo $filename

tar cvf $filename sqlite-extension/build/lib/libcalculations.*
