#!/bin/bash

# Path to the chroot directory
CHROOT_PATH="/arm64-chroot"

umount -l "$CHROOT_PATH/dev/pts"
umount -l "$CHROOT_PATH/dev"
umount -l "$CHROOT_PATH/sys"
umount -l "$CHROOT_PATH/proc"
