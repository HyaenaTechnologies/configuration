#!/bin/bash

# Ubuntu Driver Installer
sudo ubuntu-drivers install

# APT Virtual Machine/Virtual Private Server Setup
sudo apt update
sudo apt -y full-upgrade
# Install Utilities
sudo apt -y install apt-transport-https ca-certificates curl git zsh libssl libssl-dev ufw iptables firewalld nftables
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
curl -s https://packagecloud.io/install/repositories/github/git-lfs/script.deb.sh | sudo bash
sudo apt -y install git-lfs gcc gdb llvm clang lldb make cmake ninja
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
wget https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz  tar --extract --file ./*.xz --verbose
mv ./zig-linux-x86_64-0.13.0 ./zig  sudo mv ./zig /usr/bin
wget https://github.com/zigtools/zls/releases/download/0.13.0/zls-x86_64-linux.tar.xz  tar --extract --file ./*.xz --verbose
sudo mv ./zls /usr/bin/zig
sudo add-apt-repository ppa:maveonair/helix-editor  sudo apt update
sudo apt -y install ed sed nano vim neovim helix podman
# Add Docker's official GPG key:
sudo apt -y update
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc

# Add the repository to Apt sources:
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "${UBUNTU_CODENAME:-$VERSION_CODENAME}") stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt -y update
curl -LO https://github.com/kubernetes/minikube/releases/latest/download/minikube-linux-amd64
sudo install minikube-linux-amd64 /usr/local/bin/minikube && rm minikube-linux-amd64
curl -L https://github.com/kubernetes/kompose/releases/download/v1.35.0/kompose-linux-amd64 -o kompose
curl -Lo kops https://github.com/kubernetes/kops/releases/download/$(curl -s https://api.github.com/repos/kubernetes/kops/releases/latest | grep tag_name | cut -d '"' -f 4)/kops-linux-amd64
chmod +x kops
sudo mv kops /usr/local/bin/kops
curl -Lo skaffold https://storage.googleapis.com/skaffold/releases/latest/skaffold-linux-amd64 && \
sudo install skaffold /usr/local/bin/
