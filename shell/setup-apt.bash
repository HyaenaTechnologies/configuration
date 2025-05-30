#!/bin/env bash

# Ubuntu Driver Installer
sudo ubuntu-drivers install
# Initial System Update
sudo apt update && sudo apt -y full-upgrade
# Install Utilities
sudo apt -y install curl ssh apt-transport-https ca-certificates pkg-config linux-tools-generic fdutils libssl-dev ufw iptables firewalld nftables
# Install Tools
curl -s https://packagecloud.io/install/repositories/github/git-lfs/script.deb.sh | sudo bash
sudo apt -y install git-lfs gcc gdb llvm clang lldb make cmake ninja-build tree git zsh rsync librsync-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
rustup target add aarch64-unknown-linux-gnu
rustup target add aarch64-pc-windows-msvc
rustup target add x86_64-pc-windows-msvc
sudo add-apt-repository ppa:maveonair/helix-editor
sudo apt update
sudo apt -y install ed sed nano micro neovim helix podman openssh ffmpeg imagemagick
# Add Docker's official GPG key:
sudo apt -y update
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc
# Add Docker's official GPG key:
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc
# Add the repository to Apt sources:
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "${UBUNTU_CODENAME:-$VERSION_CODENAME}") stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt update

sudo apt -y install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
# Install Vulkan
sudo apt -y install libvulkan-dev libvulkan-memory-allocator-dev libvulkan-volk-dev libvulkan1 spirv-cross vulkan-memory-allocator-doc vulkan-tools vulkan-utility-libraries-dev vulkan-validationlayers libshaderc-dev glslang-dev glslang-tools
# Install WebM and WebP Image Format Libraries
sudo apt -y install libwebm-dev libwebm-tools libwebm1 libwebp-dev libwebp7 libwebpdecoder3 libwebpdemux2 libwebpmux3
# Install LibUSB
sudo apt -y install libusb-dev libhidapi-dev
# Install Filament 3D Rendering Library
sudo apt -y install libfilament-dev libfilament-tools
# Install Mesa
sudo apy -y install mesa-vulkan-drivers mesa-vdpau-drivers mesa-va-drivers mesa-utils-bin mesa-utils mesa-libgallium mesa-common-dev mesa-drm-shim mesa-opencl-icd
# Install International Components for Unicode
sudo apt -y install libicu-dev icu-devtools icu-doc
# Install Brave Browser
sudo curl -fsSLo /usr/share/keyrings/brave-browser-archive-keyring.gpg https://brave-browser-apt-release.s3.brave.com/brave-browser-archive-keyring.gpg
echo "deb [signed-by=/usr/share/keyrings/brave-browser-archive-keyring.gpg] https://brave-browser-apt-release.s3.brave.com/ stable main"|sudo tee /etc/apt/sources.list.d/brave-browser-release.list
sudo apt update && sudo apt -y install brave-browser
# Install Kops
curl -Lo kops https://github.com/kubernetes/kops/releases/download/$(curl -s https://api.github.com/repos/kubernetes/kops/releases/latest | grep tag_name | cut -d '"' -f 4)/kops-linux-amd64
chmod +x kops
sudo mv kops /usr/local/bin/kops
# Install Minikube
curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube_latest_amd64.deb
sudo dpkg -i minikube_latest_amd64.deb
# Install Skaffold
curl -Lo skaffold https://storage.googleapis.com/skaffold/releases/latest/skaffold-linux-amd64 && \
sudo install skaffold /usr/local/bin/
# Install Snaps
sudo snap install snapcraft --classic
sudo snap install snap-store snapd-desktop-integration desktop-security-center firmware-updater prompting-client krita 0ad warzone2100

