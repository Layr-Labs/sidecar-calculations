#!/usr/bin/env bash

OS=$(uname -o | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m | tr '[:upper:]' '[:lower:]')

if [[ -f .release_version ]];
then
    VERSION=$(cat .release_version)
fi

if [[ -z $VERSION ]]; then
    echo "No version found, using commit hash"
    VERSION=$(git rev-parse --short HEAD)
fi

# if OS is gnu/linux, strip off the gnu part
if [[ $OS == "gnu/linux" ]]; then
    OS="linux"
fi

filename="sidecar-calculations-${VERSION}-${OS}-${ARCH}.tar.gz"

find sqlite-extension/build/lib -name "libcalculations.*" | xargs tar cvf $filename
