[APT Debian]: https://www.debian.org/
[APT Ubuntu]: https://ubuntu.com/
[Arduino]: https://arduino.cc/en/software
[Containerman]: https://podman.io/
[Git Repositories]: https://git-scm.com
[Go Language]: https://go.dev/
[Greptime Database]: https://greptime.com/
[Helix Editor]: https://helix-editor.com/
[Influx Database]: https://influxdata.com/
[JuiceFS Database]: https://juicefs.com/
[K8S Control]: https://kubernetes.io/
[K8S Kompose]: https://kompose.io/
[K8S Kube]: https://minikube.sigs.k8s.io/docs/
[K8S Ops]: https://kops.sigs.k8s.io/
[K8S Skaffold]: https://skaffold.dev/
[Loki Database]: https://grafana.com/oss/loki/
[Mimir Database]: https://grafana.com/oss/mimir/
[Minio Database]: https://min.io/
[Moby]: http://docker.com
[Moby Compose]: https://docs.docker.com/reference/cli/docker/compose/
[Moby Swarm]: https://docs.docker.com/reference/cli/docker/swarm/
[NeoVim Editor]: https://neovim.io/
[Prometheus Database]: https://prometheus.io/
[RPM CentOS]: http://centos.org/
[RPM Fedora]: https://fedoraproject.org/
[RPM RHEL]: http://developers.redhat.com/
[Rust Language]: https://rust-lang.org/
[Spice Database]: https://authzed.com/
[Surreal Database]: https://surrealdb.com/
[Tempo Database]: https://grafana.com/oss/tempo/
[Victoria Metrics Database]: https://victoriametrics.com/
[Vim Editor]: https://www.vim.org/
[VSCode]: https://code.visualstudio.com/
[Zig Language]: https://ziglang.org/

# Configuration

Development Environment Configuration

## Development Environment

- **_Container Engines:_** [Docker][Moby], [Podman][Containerman]

- **_Container Orchestration Engines:_** [Docker Compose][Moby Compose], [Docker Swarm][Moby Swarm], [KOps][K8S Ops], [Minikube][K8S Kube]

- **_Container Tools:_** [Kompose][K8S Kompose], [Kubectl][K8S Control], [Skaffold][K8S Skaffold]

- **_Editors:_** [Helix][Helix Editor], [NeoVim][NeoVim Editor], [Vim][Vim Editor], [Visual Studio Code][VSCode]

- **_Integrated Development Environments:_** [Arduino IDE][Arduino]

- **_Programming Languages:_** [Go][Go Language], [Rust][Rust Language], [Zig][Zig Language]

- **_Operating Systems:_** [Fedora][RPM Fedora] -> [CentOS][RPM CentOS] -> [Red Hat Enterprise Linux][RPM RHEL] || [Ubuntu][APT Ubuntu] <- [Debian][APT Debian]

- **_Version Control Systems:_** [Git][Git Repositories]

## Databases

- **_Authorization:_** [Spice][Spice Database]

- **_Graph:_** [Surreal][Surreal Database]

- **_Object Storage:_** [JuiceFS][JuiceFS Database], [Minio][Minio Database]

- **_Time Series:_** [Greptime][Greptime Database], [Influx][Influx Database], [Loki][Loki Database], [Mimir][Mimir Database], [Prometheus][Prometheus Database], [Tempo][Tempo Database], [Victoria Metrics][Victoria Metrics Database]


## Build Update Releaser

```shell
git clone

GOOS=linux GOARCH=amd64 go build -o ./binary/upr ./source/main.go
```

## Install Update Releaser

```shell
echo 'export PATH="$PATH:/usr/local/bin/upr"' >> ~/.bashrc && sudo echo 'export PATH="$PATH:/usr/local/bin/upr"' >> /etc/skel/.bashrc
```