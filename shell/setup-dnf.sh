#!/bin/bash

sudo dnf -y upgrade && sudo dnf -y install dnf-plugins-core curl git iptables firewalld nftables

sudo echo 'export PATH="$PATH:/usr/bin"' >> ~/.bashrc && sudo echo 'export PATH="$PATH:/usr/bin"' >> /etc/skel/.bashrc

sudo echo 'export PATH="$PATH:/usr/local/bin"' >> ~/.bashrc && sudo echo 'export PATH="$PATH:/usr/local/bin"' >> /etc/skel/.bashrc

sudo echo 'export PATH="$PATH:/usr/include"' >> ~/.bashrc && sudo echo 'export PATH="$PATH:/usr/include"' >> /etc/skel/.bashrc

sudo echo 'export PATH="$PATH:/usr/local/include"' >> ~/.bashrc && sudo echo 'export PATH="$PATH:/usr/local/include"' >> /etc/skel/.bashrc

sudo echo '. "$HOME/.cargo/env"' >> ~/.bashrc && sudo echo '. "$HOME/.cargo/env"' >> /etc/skel/.bashrc

sudo echo 'export PATH="$PATH:/usr/bin/go/bin"' >> ~/.bashrc && sudo echo 'export PATH="$PATH:/usr/bin/go/bin"' >> /etc/skel/.bashrc

sudo echo 'export PATH="$PATH:/usr/bin/vulkan/x86_64/bin"' >> ~/.bashrc && sudo 'export PATH="$PATH:/usr/bin/vulkan/x86_64/bin"' >> /etc/skel/.bashrc

sudo dnf -y install zig gcc gdb llvm clang lldb make cmake ninja

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

wget https://go.dev/dl/go1.23.4.linux-amd64.tar.gz && tar --extract --file ./*.gz --verbose

sudo mv ./go1.23.4.linux-amd64/go /usr/bin

wget https://sdk.lunarg.com/sdk/download/1.3.296.0/linux/vulkansdk-linux-x86_64-1.3.296.0.tar.xz && tar --extract --file ./*.xz --verbose

sudo cp -r ./1.3.296.0/x86_64/include /usr/local/include && sudo mv /usr/local/include/include /usr/local/include/VulkanSDK

mv ./1.3.296.0 ./vulkan && sudo mv ./vulkan /usr/bin

sudo dnf -y install ed sed nano vim neovim helix podman

sudo dnf-3 config-manager --add-repo https://download.docker.com/linux/fedora/docker-ce.repo

sudo dnf -y install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

curl -LO https://github.com/kubernetes/minikube/releases/latest/download/minikube-latest.x86_64.rpm

sudo dnf -y install minikube-latest.x86_64.rpm

curl -L https://github.com/kubernetes/kompose/releases/download/v1.35.0/kompose-linux-amd64 -o kompose

curl -Lo kops https://github.com/kubernetes/kops/releases/download/$(curl -s https://api.github.com/repos/kubernetes/kops/releases/latest | grep tag_name | cut -d '"' -f 4)/kops-linux-amd64

chmod +x ./kompose ./kops && sudo mv ./kompose ./kops /usr/local/bin

curl -Lo skaffold https://storage.googleapis.com/skaffold/releases/latest/skaffold-linux-amd64 && \
sudo install skaffold /usr/local/bin/
