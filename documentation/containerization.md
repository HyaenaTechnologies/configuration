# Linux Containerization


## Load Environment Variables

```shell
# Load from a File
eval ./*.env

||

source ./*.env

||

# Load from Environment

env
export
printenv
set
```
## Containerization

```shell
# Create an Isolated NameSpace
sudo unshare --pid --fork --mount-proc --mount /bin/bash

# List NameSpaces and Grep Process Identifications
lsns | grep pid

# Create Root File System
mkdir -p ~/container/rootfs

# Install a Bootstrap Linux Image
sudo apt update && sudo apt install debootstrap -y
sudo debootstrap --variant=minbase stable ~/container/rootfs http://deb.debian.org/debian

# Mount Required System Directories
sudo mkdir -p ~/container/rootfs/proc
sudo mkdir -p ~/container/rootfs/sys
sudo mkdir -p ~/container/rootfs/dev
sudo mkdir -p ~/container/rootfs/dev/pts

sudo mount -t proc proc ~/container/rootfs/proc
sudo mount --rbind /sys ~/container/rootfs/sys
sudo mount --rbind /dev ~/container/rootfs/dev
sudo mount --rbind /dev/pts ~/container/rootfs/dev/pts

# Verify Mounts
mount | grep ~/container/rootfs

# Enter the Chroot Environment
sudo chroot ~/container/rootfs /bin/bash
```

