#!/usr/bin/env bash

OS=$(uname -o | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m | tr '[:upper:]' '[:lower:]')

filename="sidecar-calculations-${OS}-${ARCH}.tar.gz"

echo $filename

tar cvf $filename sqlite-extension/build/lib/libcalculations.*
