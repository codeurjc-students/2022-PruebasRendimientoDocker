# Push Artillery Metrics Into Prometheus

## Overview

The purpose of this guide is to provide instructions on how to **push artillery metrics into Prometheus** using **Docker**. Prometheus is a popular open-source monitoring and alerting solution widely used in the industry for monitoring and alerting of various systems, including web applications.

Artillery is a popular load testing tool used to test the performance and scalability of web applications. By pushing Artillery metrics into Prometheus, we can monitor and visualize the performance of our application under load tests.

## Pre-requisites

Before we can push Artillery metrics into Prometheus using Docker, we need to have Docker and Docker Compose installed on the machine.

Please note that for this guide, we will launch the Artillery test against Prometheus, so there is no need for a web application to be load tested.

## Steps

### 1: Create a Docker Compose File With Prometheus Running

To start, we will create a Docker Compose file that launches Prometheus. We will create a file called `docker-compose.yml` with the following contents:

```yaml
services:
  prometheus:
    image: prom/prometheus
    container_name: prometheus
    restart: always
    ports:
      - '9090:9090'
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
    volumes:
      - './prometheus.yml:/etc/prometheus/prometheus.yml'
```

> **Note**: The above Docker Compose file needs a file called `prometheus.yml` you can create an empty file with that name for now.

This will launch a Prometheus server and bind its web UI to port 9090 on the Docker host.

### 2: Create an Artillery Script

Next, we need to create an Artillery script that will simulate a load test. We will create, inside a directory called `scripts` a file called `test-prometheus.yml` with the following contents:

```yaml
config:
  target: 'http://localhost:9090'
  phases:
    - duration: 60
      arrivalRate: 10
scenarios:
- flow:
    - get:
        url: '/metrics'
```
> **Note**: If you are running your development server on WSL , you may need to access your application using the IP address `127.0.0.1` instead of `localhost`.

Now, to run it, use this command:

```shell
docker run --rm -it \
    --volume ${PWD}/scripts:/scripts \
    --network host \
    artilleryio/artillery:latest \
    run /scripts/test-prometheus.yml
```

If everything went well, you should be able to see an output like this:

```
--------------------------------------
Metrics for period to: 15:56:30(+0000) (width: 7.351s)
--------------------------------------

http.codes.200: ........................................ 77
http.request_rate: ..................................... 10/sec
http.requests: ......................................... 78
http.response_time:
  min: ................................................. 4
  max: ................................................. 11
  median: .............................................. 5
  p95: ................................................. 7.9
  p99: ................................................. 10.1
http.responses: ........................................ 77
vusers.created: ........................................ 78
vusers.created_by_name.0: .............................. 78
```

### Push Artillery Metrics Into Prometheus

One of the primary features of Prometheus is its pull-based model, where it scrapes metrics from targets at regular intervals. This model works well for targets that have long lifetimes, such as servers or containers, but is not ideal for short-lived tasks, such as those executed by Artillery.

To address this issue, a push-based approach can be used, where the metrics are pushed from the task to Prometheus. This approach involves the use of a pushgateway, which is a service that can be used to collect metrics and push them to Prometheus. The pushgateway acts as an intermediary between the task and Prometheus, and can be configured as a target for Prometheus to scrape.

### 3: Add Pushgateway to Docker Compose

Next, we need to modify our `docker-compose.yml` file to include the Pushgateway service. We will add the following service definition to the file:

```yaml
pushgateway:
  image: 'prom/pushgateway'
  container_name: pushgateway
  restart: always
  ports:
    - '9091:9091'
```

### 4: Add Pushgateway to Prometheus Targets

We need to configure Prometheus to scrape metrics from the Pushgateway server. We will add the following target definition to the prometheus.yml file:

```yaml
scrape_configs:
  - job_name: "pushgateway"
    honor_labels: true
    static_configs:
      - targets: ["pushgateway:9091"]
        labels:
          alias: 'pushgateway'
```

### 5: Add Pushgateway to Artillery Configuration

Finally, we need to modify our artillery.yaml file to include the prometheus exporter and tell Artillery to push metrics to the Pushgateway server. We will add the following configuration to the file:

```yaml
config:
  target: 'http://localhost:9090'
  phases:
    - duration: 60
      arrivalRate: 10
  plugins:
    publish-metrics:
      - type: prometheus
        pushgateway: "http://localhost:9091"
        tags:
          - "testId:prometheus_test"
          - "type:loadtest"
scenarios:
- flow:
    - get:
        url: '/metrics'
```

This tells Artillery to enable the prometheus plugin, which will push metrics to the Pushgateway server at `http://localhost:9091`.

## References

- [Prometheus Pushgateway Repository](https://github.com/prometheus/pushgateway)
- [When to use a Pushgateway](https://prometheus.io/docs/practices/pushing/#when-to-use-the-pushgateway)
- [Configure Artillery Script to export metrics to Pushgateway](https://www.artillery.io/docs/guides/plugins/plugin-publish-metrics#prometheus-pushgateway)
- [Create Artillery Scripts for HTTP Requests](https://www.artillery.io/docs/guides/guides/http-reference)