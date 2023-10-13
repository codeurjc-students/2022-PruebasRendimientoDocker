# 2022-Docker Performance Testing

## Table of Contents

- [2022-Docker Performance Testing](#2022-docker-performance-testing)
  - [Table of Contents](#table-of-contents)
    - [Related Documentation](#related-documentation)
  - [Repository Guides](#repository-guides)
    - [Pre-requisites](#pre-requisites)
      - [Prepare PHP application](#prepare-php-application)
  - [How to run](#how-to-run)
    - [Run the docker-compose file](#run-the-docker-compose-file)
    - [Link grafana with prometheus](#link-grafana-with-prometheus)
      - [Using the UI](#using-the-ui)
      - [Using the provisioning file](#using-the-provisioning-file)
  - [Run artillery tests](#run-artillery-tests)
    - [Running the test with the Artillery CLI](#running-the-test-with-the-artillery-cli)
    - [Running the tests with Docker](#running-the-tests-with-docker)
    - [Available tests](#available-tests)
  - [References](#references)
    - [Related repositories](#related-repositories)
    - [cAdvisor](#cadvisor)
      - [cAdvisor fixes](#cadvisor-fixes)
    - [Integration of cAdvisor with Prometheus](#integration-of-cadvisor-with-prometheus)
    - [Grafana](#grafana)
    - [Artillery](#artillery)
    - [Pushing Artillery metrics to Prometheus](#pushing-artillery-metrics-to-prometheus)


### Related Documentation 

- [Metrics for Apache applications running in Docker](./docs/metrics_for_apache_applications_running_in_docker.md)
- [How to Provision Dashboard in Grafana](./docs/how_to_provision_dashboards.md)
- [Creating Docker Container Resource Consumption Dashboard](./docs/create_docker_container_resource_consumption_dashboard.md)
- [Managing Dashboards through UI vs as Code](./docs/dashboard_ui_vs_dashboard_as_code.md)
- [Pull Docker metrics into Prometheus](./docs/pull_docker_metrics_into_prometheus.md)
- [Pull Apache metrics into Prometheus](#)
- [Push Artillery metrics into Prometheus](./docs/push_artillery_metrics_into_prometheus.md)
- [Testing with Artillery](./docs/testing_with_artillery.md)

## Repository Guides

### Pre-requisites

1. Install Docker (Tested on version 20.10.23)
2. Install Docker Compose (Tested on version 2.16.0)
3. Create a docker `Volume` for grafana called `grafana-storage`
 
    ```shell
    docker volume create grafana-storage
    ```

#### Prepare PHP application

Follow the steps below to perform the observability tests on the PHP application:

1. Create a docker `Volume` for PostgreSQL:

    ```shell
    docker volume create postgres-data
    ```

2. Create inside the current directory a directory an `.env` file with the following content:

    ```env
    UID=1000
    GID=1000
    POSTGRES_USER=postgres
    POSTGRES_PASSWORD=postgres
    POSTGRES_DB=postgres
    ```

    Change the values of UID and GID to your user and group IDs. To find out your user and group IDs, run the following commands:

    ```shell
    id -u # User ID
    id -g # Group ID
    ```

3. Run the docker-compose:

    ```shell
    docker compose -f docker-compose.yml up -d
    ```

4. Clone the repository [app-apache-php](https://github.com/MarioRP-01/app-apache-php)

    ```shell
    git clone git@github.com:MarioRP-01/app-apache-php.git # SSH
    git clone https://github.com/MarioRP-01/app-apache-php.git # HTTPS
    ```

5. Copy the .env file into the `app-apache-php` directory and change to the `app-apache-php` directory:

    ```shell
    cp .env app-apache-php/
    cd app-apache-php
    ```

6. Follow the instructions of the [set up resources guide](https://github.com/MarioRP-01/app-apache-php/blob/main/docs/set-up-data-storage.md) to dump the information in the volume.
  
    It's possible to use the `docker-compose.yml` containers instead of the ones in the guide. **Skip the steps 1 and 2** in the [Postgres](mariorp01/app-php-apache) section.

7. Finally, copy the `resources` directory into this directory and remove the `app-apache-php` directory`:

    ```shell
    cp -r app-apache-php/resources ./apache-resources
    rm -rf app-apache-php
    ```

## How to run

### Run the docker-compose file

```shell
docker compose up -d
```

### Link grafana with prometheus

For grafana to be able to query prometheus, you need to create a data source in grafana. There are two ways to do this:

#### Using the UI
1. Run the docker-compose file
2. Open the browser and go to `localhost:3000` and login with the default username and password `admin:admin`
3. Go to Configuration > Data sources > Add data source
4. Pick Prometheus as data type
5. Save the URL as `http://prometheus:9090` and save the data source

#### Using the provisioning file
Alternatively, you can create a provisioning file for grafana to automatically create the data source. See the directory `grafana/provisioning/datasources` for an example. For more information, see the references section.

## Run artillery tests

One of the goals of this repository is to be able to observe the changes in the metrics when the application is subjected to different loads. To do this, we use [Artillery](https://artillery.io/), a load testing tool that allows us to simulate different loads on the application.

The `artillery` directory contains all the Artillery tests and other artifacts used by the tests.

### Running the test with the Artillery CLI

1. Ensure that you have Artillery installed (last tested with version  2.0.0-31)

    ```shell
    npm install -g artillery@latest
    ```
2. To run a specific test, execute the following command:

    ```shell
    artillery run artillery/<test-script>
    ```
    Replace `<test-script>` with the name of the Artillery test script you want to run.

### Running the tests with Docker

- To run the tests using Docker, execute the following command:

    ```shell
    docker run --rm -it \
        --volume ${PWD}/artillery:/scripts \
        --network host \
        artilleryio/artillery:2.0.0-31 \
        run /scripts/<test-script>
    ```

    Replace `<test-script>` with the name of the Artillery test script you want to run.

- Alternatively, you can use the shell script `run-artillery` to run the tests:

    ```shell
    ./run-artillery <test-script>
    ```

    Replace `<test-script>` with the name of the Artillery test script you want to run.

### Available tests

- `apache-test-1.yml`: Go to the main page, then ask for pages until there are no more pages to ask for.

    ```shell
    # Artillery CLI
    artillery run artillery/apache-test-1.yml
    ```

    ```shell
    # Docker
    docker run --rm -it \
        --volume ${PWD}/artillery:/scripts \
        --network host \
        artilleryio/artillery:2.0.0-31 \
        run /scripts/apache-test-1.yml
    ```

    ```shell
    # Shell script
    ./run-artillery apache-test-1.yml
    ```


- `apache-test-2.yml`: Go to the main page and then navigate to a new product each two seconds twenty times.

    ```shell
    # Artillery CLI
    artillery run artillery/apache-test-2.yml
    ```

    ```shell
    # Docker
    docker run --rm -it \
        --volume ${PWD}/artillery:/scripts \
        --network host \
        artilleryio/artillery:2.0.0-31 \
        run /scripts/apache-test-2.yml
    ```

    ```shell
    # Shell script
    ./run-artillery apache-test-2.yml
    ```

- `apache-test-3.yml`: Search for a product and navigate to the first result.

    ```shell
    # Artillery CLI
    artillery run artillery/apache-test-3.yml
    ```

    ```shell
    # Docker
    docker run --rm -it \
        --volume ${PWD}/artillery:/scripts \
        --network host \
        artilleryio/artillery:2.0.0-31 \
        run /scripts/apache-test-3.yml
    ```

    ```shell
    # Shell script
    ./run-artillery apache-test-3.yml
    ```

## References

### Related repositories

- [app-apache-php](https://github.com/MarioRP-01/app-apache-php/tree/main)
- [clothing-dataset-small-manager](https://github.com/MarioRP-01/clothing-dataset-small-manager)

### cAdvisor

#### cAdvisor fixes

- [Failed to get system UUID](https://github.com/google/cadvisor/issues/2157)

### Integration of cAdvisor with Prometheus

- [Configuration used for cAdvisor in docker-compose](https://github.com/google/cadvisor#quick-start-running-cadvisor-in-a-docker-container)
- [How to monitor docker containers with Prometheus](https://prometheus.io/docs/guides/cadvisor/#monitoring-docker-container-metrics-using-cadvisor)
- [cAdvisor metrics for Prometheus](https://github.com/google/cadvisor/blob/master/docs/storage/prometheus.md)

### Grafana

- [Provisioning Data Sources](https://grafana.com/docs/grafana/latest/administration/provisioning/#data-sources)
- [Create provisioning for Prometheus](https://grafana.com/docs/grafana/latest/datasources/prometheus/#provision-the-data-source)
- [Create variables in grafana](https://grafana.com/docs/grafana/latest/dashboards/variables/add-template-variables/)
- [Create Prometheus template variables](https://grafana.com/docs/grafana/latest/datasources/prometheus/template-variables/)
- [How to Extract Label Values from Prometheus Metrics in Grafana](https://grafana.com/blog/2023/02/23/how-to-extract-label-values-from-prometheus-metrics-in-grafana/)

### Artillery
- [Install Artillery](https://www.artillery.io/docs/guides/getting-started/installing-artillery)
- [Run Artillery with Docker](https://www.artillery.io/docs/guides/guides/docker)

### Pushing Artillery metrics to Prometheus
- [Pushing metrics to Prometheus](https://prometheus.io/docs/instrumenting/pushing/#pushing-metrics)
- [Pushgateway Repository](https://github.com/prometheus/pushgateway)
- [Artillery Pushgateway Plugin](https://prometheus.io/docs/instrumenting/pushing/#pushing-metrics)