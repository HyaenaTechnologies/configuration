FROM amd64/fedora:latest

WORKDIR /zig_application

COPY ./ ./

RUN dnf -y upgrade
RUN dnf -y install zig
RUN zig build
RUN zig test

ENTRYPOINT ["/bin/linux/zig_application"]