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

- **_Container Orchestration Engines:_** [Docker Compose][Moby Compose], [Docker Swarm][Moby Swarm], [Minikube][K8S Kube], [KOps][K8S Ops]

- **_Container Tools:_** [Kompose][K8S Kompose], [Kubect]l[K8S Control], [Skaffold] [K8S Skaffold]

- **_Editors:_** [Helix][Helix Editor], [NeoVim][NeoVim Editor], [Vim][Vim Editor], [Visual Studio Code][VSCode]

- **_Integrated Development Environments:_** [Arduino IDE][Arduino]

- **_Programming Languages:_** [Go][Go Language], [Rust][Rust Language], [Zig][Zig Language]

- **_Operating Systems:_** Fedora -> CentOS -> Red Hat Enterprise Linux || Ubuntu <- Debian

- **_Version Control Systems:_** [Git][Git Repositories]

## Databases

- **_Authorization_**: [SpiceDB][Spice Database]

- **_Graph_**: [Surreal][Surreal Database]

- **_Object Storage_**: [Minio][Minio Database], [JuiceFS][JuiceFS Database]

- **_Time Series_**: [InfluxDB][Influx Database], [Greptime][Greptime Database], [Loki][Loki Database], [Prometheus][Prometheus Database], [Mimir][Mimir Database], [Tempo][Tempo Database], [Victoria Metrics][Victoria Metrics Database]


## Build Update Releaser

```shell
git clone

GOOS=linux GOARCH=amd64 go build -o ./binary/upr ./source/main.go
```

## Install Update Releaser

```shell
echo 'export PATH="$PATH:/usr/local/bin/upr"' >> ~/.bashrc && sudo echo 'export PATH="$PATH:/usr/local/bin/upr"' >> /etc/skel/.bashrc
```