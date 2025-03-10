#!/bin/bash

# Nvidia RPM Fusion
sudo dnf -y install akmod-nvidia xorg-x11-drv-nvidia xorg-x11-drv-nvidia-cuda

# DNF Virtual Machine/Virtual Private Server Setup
sudo dnf -y upgrade
# Install Utilities
sudo dnf -y install dnf-utils dnf-plugins-core curl git zsh openssl openssl-devel ufw iptables firewalld nftables
# Export PATH Environment Variables to .bashrc
sudo echo 'export PATH="$PATH:/usr/bin"' >> ~/.bashrc
sudo echo 'export PATH="$PATH:/usr/local/bin"' >> ~/.bashrc
sudo echo 'export PATH="$PATH:/usr/include"' >> ~/.bashrc
sudo echo 'export PATH="$PATH:/usr/local/include"' >> ~/.bashrc
sudo echo 'export PATH="$PATH:/usr/lib"' >> ~/.bashrc
sudo echo 'export PATH="$PATH:/usr/local/lib"' >> ~/.bashrc
# Install Tools
sudo echo '. "$HOME/.cargo/env"' >> ~/.bashrc
sudo echo 'export PATH="$PATH:/usr/bin/zig"' >> ~/.bashrc
curl -s https://packagecloud.io/install/repositories/github/git-lfs/script.rpm.sh | sudo bash
sudo dnf -y install zig git-lfs gcc gdb llvm clang lldb make cmake ninja
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo dnf -y install ed sed nano vim neovim helix podman
sudo dnf-3 config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo
sudo dnf -y install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
curl -LO https://github.com/kubernetes/minikube/releases/latest/download/minikube-linux-amd64
sudo install minikube-linux-amd64 /usr/local/bin/minikube && rm minikube-linux-amd64
curl -L https://github.com/kubernetes/kompose/releases/download/v1.35.0/kompose-linux-amd64 -o kompose
curl -Lo kops https://github.com/kubernetes/kops/releases/download/$(curl -s https://api.github.com/repos/kubernetes/kops/releases/latest | grep tag_name | cut -d '"' -f 4)/kops-linux-amd64
chmod +x kops
sudo mv kops /usr/local/bin/kops
curl -Lo skaffold https://storage.googleapis.com/skaffold/releases/latest/skaffold-linux-amd64 && \
sudo install skaffold /usr/local/bin/
