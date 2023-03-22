### Run the docker-compose file
```bash
docker compose -f docker-compose.json up -d
```

### How to link graphana with prometheus
1. Run the docker-compose file
2. Open the browser and go to `localhost:3000` and login with the default username and password `admin:admin`
3. Go to Configuration > Data sources > Add data source
4. Pick Prometheus as data type
5. Save the URL as `http://prometheus:9090` and save the data source