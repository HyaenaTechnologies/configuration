#!/bin/env bash

# Initial System Update
sudo dnf -y upgrade
# Install Utilities
sudo dnf -y install ssh dnf-utils dnf-plugins-core curl openssl openssl-devel ufw iptables firewalld nftables
curl -s https://packagecloud.io/install/repositories/github/git-lfs/script.rpm.sh | sudo bash
sudo dnf -y install autoconf automake gcc gdb llvm clang lldb make cmake ninja zig zsh tree git git-lfs rsync librsync
sudo dnf -y install ed sed nano podman micro neovim helix openssh ffmpeg imagemagick
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
rustup target add aarch64-unknown-linux-gnu
rustup target add aarch64-pc-windows-msvc
rustup target add x86_64-pc-windows-msvc
# Install Go Language
wget https://go.dev/dl/go1.24.4.linux-amd64.tar.gz --verbose && tar --extract --file ./*.gz --verbose
sudo install ./go /usr/local/bin/
# Install Docker
sudo dnf-3 config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo
sudo dnf -y install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin cri-o cri-tools1.33
# Install Boost, Abseil, LibCamera and International Components for Unicode
sudo dnf -y install boost abseil-cpp icu libcamera
# Install Vulkan
sudo dnf -y install vulkan-headers vulkan-loader vulkan-tools spirv-tools spirv-headers-devel
# Install WebM and WebP Image Format Libraries
sudo dnf -y install libwebp libwebp-tools 
# Install LibUSB
sudo dnf -y install libusb-dev libhidapi-dev
# Install Mesa
sudo dnf -y install mesa-dri-drivers mesa-filesystem mesa-va-drivers mesa-vdpau-drivers mesa-vulkan-drivers
# Install Kubectl
curl -LO https://dl.k8s.io/release/v1.33.0/bin/linux/amd64/kubectl
sudo install ./kubectl /usr/local/bin/
# Install Kops
curl -Lo kops https://github.com/kubernetes/kops/releases/download/$(curl -s https://api.github.com/repos/kubernetes/kops/releases/latest | grep tag_name | cut -d '"' -f 4)/kops-linux-amd64
chmod +x kops
sudo install ./kops /usr/local/bin/
# Install Minikube
curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube-latest.x86_64.rpm
sudo rpm -Uvh minikube-latest.x86_64.rpm
rm ./minikube-latest.x86_64.rpm
# Install Kompose
curl -L https://github.com/kubernetes/kompose/releases/download/v1.36.0/kompose-linux-amd64 -o kompose
sudo install ./kompose /usr/local/bin/
# Install Skaffold
curl -Lo skaffold https://storage.googleapis.com/skaffold/releases/latest/skaffold-linux-amd64
sudo install ./skaffold /usr/local/bin/
# Install Brave Browser
sudo dnf-3 config-manager --add-repo https://brave-browser-rpm-release.s3.brave.com/brave-browser.repo
sudo dnf -y install brave-browser
# Install Snapd
sudo dnf install snapd
sudo ln -s /var/lib/snapd/snap /snap
