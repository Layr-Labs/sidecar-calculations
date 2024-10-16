#!/bin/bash

set -x

# Path to the chroot directory
CHROOT_PATH="/arm64-chroot"

# Check if the script is run with sudo
if [ "$EUID" -ne 0 ]; then
    echo "Please run this script with sudo"
    exit 1
fi

# Ensure the chroot directory exists
if [ ! -d "$CHROOT_PATH" ]; then
    echo "Chroot directory does not exist. Please set it up first."
    exit 1
fi

# Mount necessary filesystems
mount -t proc /proc "$CHROOT_PATH/proc/"
mount --rbind /sys "$CHROOT_PATH/sys/"
mount --rbind /dev "$CHROOT_PATH/dev/"
mount --make-rslave "$CHROOT_PATH/dev/"
mount --rbind /dev/pts "$CHROOT_PATH/dev/pts/"
mount --make-rslave "$CHROOT_PATH/dev/pts/"

# Copy resolv.conf for network access
cp /etc/resolv.conf "$CHROOT_PATH/etc/resolv.conf"

# Complete the install
sudo chroot arm64-chroot /usr/bin/qemu-aarch64-static /bin/bash -c '/debootstrap/debootstrap --second-stage'
