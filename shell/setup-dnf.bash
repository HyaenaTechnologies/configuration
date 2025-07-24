#!/bin/env bash

# Initial System Update
sudo dnf -y upgrade
# Install Utilities
sudo dnf -y install ssh dnf-utils dnf-plugins-core curl openssl openssl-devel ufw iptables firewalld nftables
curl -s https://packagecloud.io/install/repositories/github/git-lfs/script.rpm.sh | sudo bash
sudo dnf -y install autoconf automake gcc gdb llvm clang lldb lld make cmake ninja zig zsh tree git git-lfs rsync librsync
sudo dnf -y install ed sed nano podman micro neovim helix openssh ffmpeg imagemagick
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add aarch64-apple-darwin x86_64-apple-darwin aarch64-apple-ios x86_64-apple-ios
rustup target add aarch64-unknown-linux-gnu aarch64-linux-android x86_64-linux-android
rustup target add aarch64-pc-windows-msvc x86_64-pc-windows-msvc
# Install Go Language
wget https://go.dev/dl/go1.24.4.linux-amd64.tar.gz --verbose && tar --extract --file ./*.gz --verbose
sudo install ./go /usr/local/bin/
# Install Docker
sudo dnf-3 config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo
sudo dnf -y install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin cri-o cri-tools1.33
# Install Postgres, Maria and Sqlite
sudo dnf install -y https://download.postgresql.org/pub/repos/yum/reporpms/F-41-x86_64/pgdg-fedora-repo-latest.noarch.rpm
sudo dnf install -y postgresql17-server sqlite mariadb mariadb-server
# Install Compression Tools and Libraries
sudo dnf -y install libzip liblzma-dev gzip xz zip 7z
# Install Boost, Abseil, LibCamera and International Components for Unicode
sudo dnf -y install boost abseil-cpp icu libcamera
# Install Vulkan
sudo dnf -y install vulkan-headers vulkan-loader vulkan-tools spirv-tools spirv-headers-devel
# Install WebM and WebP Image Format Libraries
sudo dnf -y install libwebp libwebp-tools 
# Install LibUSB
sudo dnf -y install libusb-dev libhidapi-dev
# Install Mesa 3D Rendering Library
sudo dnf -y install mesa-dri-drivers mesa-filesystem mesa-va-drivers mesa-vdpau-drivers mesa-vulkan-drivers
# Install Kubectl
wget https://dl.k8s.io/release/v1.33.0/bin/linux/amd64/kubectl --verbose
sudo install ./kubectl /usr/local/bin/
# Install Kops
curl -Lo kops https://github.com/kubernetes/kops/releases/download/$(curl -s https://api.github.com/repos/kubernetes/kops/releases/latest | grep tag_name | cut -d '"' -f 4)/kops-linux-amd64
chmod +x kops
sudo install ./kops /usr/local/bin/
# Install Minikube
wget https://storage.googleapis.com/minikube/releases/latest/minikube-latest.x86_64.rpm --verbose
sudo dnf -y install ./minikube-latest.x86_64.rpm
rm ./minikube-latest.x86_64.rpm
# Install Kompose
wget https://github.com/kubernetes/kompose/releases/download/v1.36.0/kompose-linux-amd64 --verbose
mv ./kompose-linux-amd64 ./kompose
sudo install ./kompose /usr/local/bin/
# Install Skaffold
wget https://storage.googleapis.com/skaffold/releases/latest/skaffold-linux-amd64 --verbose
mv ./skaffold-linux-amd64 ./skaffold
sudo install ./skaffold /usr/local/bin/
# Install Brave Browser
sudo dnf-3 config-manager --add-repo https://brave-browser-rpm-release.s3.brave.com/brave-browser.repo
sudo dnf -y install brave-browser
# Install Vivaldi Browser
wget https://downloads.vivaldi.com/stable/vivaldi-stable-7.4.3684.46-1.x86_64.rpm --verbose
sudo dnf -y install ./vivaldi-stable-7.4.3684.46-1.x86_64.rpm
rm ./vivaldi-stable-7.4.3684.46-1.x86_64.rpm
# Install Snapd
sudo dnf install snapd
sudo ln -s /var/lib/snapd/snap /snap
