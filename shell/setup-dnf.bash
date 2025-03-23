#!/bin/bash

# Nvidia RPM Fusion
sudo dnf -y install akmod-nvidia xorg-x11-drv-nvidia xorg-x11-drv-nvidia-cuda
# DNF Virtual Machine/Virtual Private Server Setup
sudo dnf -y upgrade
# Install Utilities
sudo dnf -y install dnf-utils dnf-plugins-core curl git zsh openssl openssl-devel ufw iptables firewalld nftables
# Install Tools
curl -s https://packagecloud.io/install/repositories/github/git-lfs/script.rpm.sh | sudo bash
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
rustup target add aarch64-unknown-linux-gnu
rustup target add aarch64-pc-windows-msvc
rustup target add x86_64-pc-windows-msvc
sudo dnf -y install git-lfs gcc gdb llvm clang lldb make cmake ninja
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo dnf -y install ed sed nano vim podman
sudo dnf-3 config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo
sudo dnf -y install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
sudo dnf-3 config-manager --add-repo https://brave-browser-rpm-release.s3.brave.com/brave-browser.repo
sudo dnf -y install brave-browser
