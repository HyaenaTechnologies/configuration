[Arduino]: https://arduino.cc/en/software
[C Language]: https://learn.microsoft.com/en-us/cpp/c-language
[C++ Language]: https://learn.microsoft.com/en-us/cpp/cpp/
[Git Repositories]: https://git-scm.com
[Go Language]: https://go.dev/
[Helix Editor]: https://helix-editor.com/
[NeoVim Editor]: https://neovim.io/
[Rust Language]: https://www.rust-lang.org/
[Vim Editor]: https://www.vim.org/
[VSCode]: https://code.visualstudio.com/
[Zig Language]: https://ziglang.org/

# Configuration

Development Environment Configuration

## Development Environment

- **_Container Engines:_** Docker, Podman

- **_Container Management:_** Docker Desktop, Kompose, Podman Desktop, Skaffold

- **_Container Orchestration Engines:_** Docker Swarm - Swarm Kit, Kubernetes = Minikube, KOps

- **_Editors:_** [Helix][Helix Editor], [NeoVim][NeoVim Editor], [Vim][Vim Editor], [Visual Studio Code][VSCode]

- **_Integrated Development Environments:_** [Arduino IDE][Arduino]

- **_Languages:_** [C][C Language], [C++][C++ Language], [Go][Go Language], [Rust][Rust Language], [Zig][Zig Language]

- **_Operating Systems:_** Fedora - CentOS, Red Hat Enterprise Linux, Ubuntu - Debian

- **_Platforms:_** Desktop, Web, Embedded

- **_Version Control:_** [Git][Git Repositories]

## Build Update Releaser

```shell
git clone

GOOS=linux GOARCH=amd64 go build -o ./binary/upr ./source/main.go
```

## Install Update Releaser

```shell
echo 'export PATH="$PATH:/usr/local/bin/upr"' >> ~/.bashrc && sudo echo 'export PATH="$PATH:/usr/local/bin/upr"' >> /etc/skel/.bashrc
```