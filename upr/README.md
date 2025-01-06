# Update Releaser

Update Releaser is a System Update Tool

## Features

- APT
- DNF
- DNF Release
- Snap
- Ubuntu Release

## Build

```shell
git clone

GOOS=linux GOARCH=amd64 go build -o ./binary/upr ./upr/source/main.go
```

## Install

```shell
echo 'export PATH="$PATH:/usr/local/bin/upr"' >> ~/.bashrc && sudo echo 'export PATH="$PATH:/usr/local/bin/upr"' >> /etc/skel/.bashrc
```
