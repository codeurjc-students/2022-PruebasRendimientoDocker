# Pull Docker Metrics Into Prometheus

## Introduction

## Configure Docker

First, you need to configure Docker to expose metrics. You can do this by adding the following JSON configuration to the `/etc/docker/daemon.json` file:

```json
{
  "metrics-addr": "localhost:9323"
}
```

This configuration tells Docker to expose metrics at `localhost:9323`.

## Create Docker Compose File For Prometheus

```yaml
services:
  prometheus:
    image: prom/prometheus
    container_name: prometheus
    restart: always
    ports:
      - '9090:9090'
    extra_hosts:
      - "host.docker.internal:host-gateway"
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
    volumes:
      - './prometheus.yml:/etc/prometheus/prometheus.yml'
```

## Add Prometheus Targets

Finally, add the Docker metrics endpoint as a target in Prometheus. You can do this by adding the following configuration to prometheus.yml:

```yaml
scrape_configs:
  - job_name: 'docker'
    static_configs:
      - targets: ['host.docker.internal:9323']
```

This configuration tells Prometheus to scrape metrics from the Docker daemon's metrics endpoint at `host.docker.internal:9323`.

## WSL Workaround

If you're using WSL, the `host.docker.internal` alias doesn't work to access the host network from within the Prometheus container. Instead, you need to run the Prometheus container in the host network and scrape the metrics from the WSL IP address.

To execute prometheus in the host network, you can add the following configuration to the `docker-compose.yml` file:

```yaml
services:
  prometheus:
    ...
    network_mode: host
    ...
```

To get the WSL IP address, you can use the following command in the WSL terminal:

```shell
ip addr show eth0 | grep -oP '(?<=inet\s)\d+(\.\d+){3}'
```

Once you have the WSL IP address, you can add it as target in Prometheus by modifyng the `prometheus.yml` configuration file:

```yaml
scrape_configs:
  - job_name: 'docker'
    static_configs:
      - targets: ['wsl-ip:9323']
```

Replace `wsl-ip` with the IP address obtained from the previous command.

## References

- [Docker Documentation](https://docs.docker.com/config/daemon/prometheus/)