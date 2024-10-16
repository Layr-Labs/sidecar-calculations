#!/usr/bin/env bash

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -a | tr '[:upper:]' '[:lower:]')

command_exists() {
    command -v "$@" > /dev/null 2>&1
}

apt_update_and_install() {
    if command_exists sudo; then
        sudo apt-get update
        sudo apt-get install "$@"
    else
        apt-get update
        apt-get install "$@"
    fi
}

if [[ "$OS" == "linux" ]]; then
    apt-get update
    apt-get install -y \
        curl \
        make \
        git \
        build-essential \
        sqlite3 \
        libsqlite3-0 \
        libsqlite3-dev \
        rustup

    rustup toolchain install stable
elif [[ "$OS" == "darwin" ]]; then
    hasBrew=$(which brew)
    if [[ -z $hasBrew ]]; then
        echo "Please install Homebrew to continue: https://brew.sh"
        exit 1
    fi
    # Dont automatically update brew packages and ruin everyone's day
    export HOMEBREW_NO_AUTO_UPDATE=1
    brew install sqlite

    hasGcc=$(which gcc)
    if [[ -z $hasGcc ]]; then
        echo "Please install Xcode to continue"
        exit 1
    fi

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    . "$HOME/.cargo/env"
else
    echo "Unsupported OS: $OS"
    exit 1
fi

which cargo
