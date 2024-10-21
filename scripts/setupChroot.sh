#!/bin/bash

set -x

# Path to the chroot directory
CHROOT_PATH="/arm64-chroot"

# Ensure the chroot directory exists
if [ ! -d "$CHROOT_PATH" ]; then
    echo "Chroot directory does not exist. Please set it up first."
    exit 1
fi

# Mount necessary filesystems
sudo mount -t proc /proc "$CHROOT_PATH/proc/"
sudo mount --rbind /sys "$CHROOT_PATH/sys/"
sudo mount --rbind /dev "$CHROOT_PATH/dev/"
sudo mount --make-rslave "$CHROOT_PATH/dev/"
sudo mount --rbind /dev/pts "$CHROOT_PATH/dev/pts/"
sudo mount --make-rslave "$CHROOT_PATH/dev/pts/"

# Copy resolv.conf for network access
sudo cp /etc/resolv.conf "$CHROOT_PATH/etc/resolv.conf"

# Complete the install
sudo chroot /arm64-chroot /usr/bin/qemu-aarch64-static /bin/bash -c '/debootstrap/debootstrap --second-stage'
