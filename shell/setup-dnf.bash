#!/bin/env bash

# Nvidia RPM Fusion
sudo dnf -y install akmod-nvidia xorg-x11-drv-nvidia xorg-x11-drv-nvidia-cuda
# DNF Virtual Machine/Virtual Private Server Setup
sudo dnf -y upgrade
# Install Utilities
sudo dnf -y install ssh dnf-utils dnf-plugins-core curl openssl openssl-devel ufw iptables firewalld nftables
# Install Tools
curl -s https://packagecloud.io/install/repositories/github/git-lfs/script.rpm.sh | sudo bash
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
rustup target add aarch64-unknown-linux-gnu
rustup target add aarch64-pc-windows-msvc
rustup target add x86_64-pc-windows-msvc
sudo dnf -y install git-lfs gcc gdb llvm clang lldb make cmake ninja zsh tree git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo dnf -y install ed sed nano kakoune podman micro vim neovim helix openssh
sudo dnf-3 config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo
sudo dnf -y install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
sudo dnf-3 config-manager --add-repo https://brave-browser-rpm-release.s3.brave.com/brave-browser.repo
sudo dnf -y install brave-browser
# Install Kops
curl -Lo kops https://github.com/kubernetes/kops/releases/download/$(curl -s https://api.github.com/repos/kubernetes/kops/releases/latest | grep tag_name | cut -d '"' -f 4)/kops-linux-amd64
chmod +x kops
sudo mv kops /usr/local/bin/kops
# Install Minikube
curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube-latest.x86_64.rpm
sudo rpm -Uvh minikube-latest.x86_64.rpm
# Install Skaffold
curl -Lo skaffold https://storage.googleapis.com/skaffold/releases/latest/skaffold-linux-amd64 && \
sudo install skaffold /usr/local/bin/
# Install Snapd
sudo dnf install snapd
sudo ln -s /var/lib/snapd/snap /snap
