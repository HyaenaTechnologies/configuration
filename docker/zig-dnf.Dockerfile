FROM amd64/fedora:latest

WORKDIR /zig-application

COPY ./ ./

RUN dnf -y upgrade \ 
dnf -y install zig \ 
zig test \ 
zig build \ 
