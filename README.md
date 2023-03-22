## Pre-requisites

1. Install Docker (All test made with version 20.10.23)
2. Create a docker volume for grafana called grafana-storage

## How to run

### Run the docker-compose file

```bash
docker compose -f docker-compose.json up -d
```

### Link grafana with prometheus

1. Run the docker-compose file
2. Open the browser and go to `localhost:3000` and login with the default username and password `admin:admin`
3. Go to Configuration > Data sources > Add data source
4. Pick Prometheus as data type
5. Save the URL as `http://prometheus:9090` and save the data source