## Pre-requisites

1. Install Docker (Tested on version 20.10.23)
2. Install Docker Compose (Tested on version 2.16.0)
3. Create a docker `Volume` for grafana called `grafana-storage`

```bash
docker volume create grafana-storage
```

## How to run

### Run the docker-compose file

```bash
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

## References

### Integration of cAdvisor with Prometheus

- [Configuration used for cAdvisor in docker-compose](https://github.com/google/cadvisor#quick-start-running-cadvisor-in-a-docker-container)
- [How to monitor docker containers with Prometheus](https://prometheus.io/docs/guides/cadvisor/#monitoring-docker-container-metrics-using-cadvisor)
- [cAdvisor metrics for Prometheus](https://github.com/google/cadvisor/blob/master/docs/storage/prometheus.md)

### Grafana

- [Provisioning Data Sources](https://grafana.com/docs/grafana/latest/administration/provisioning/#data-sources)
- [Create provisioning for Prometheus](https://grafana.com/docs/grafana/latest/datasources/prometheus/#provision-the-data-source)
- [Create variables in grafana](https://grafana.com/docs/grafana/latest/dashboards/variables/add-template-variables/)
- [Create Prometheus template variables](https://grafana.com/docs/grafana/latest/datasources/prometheus/template-variables/)