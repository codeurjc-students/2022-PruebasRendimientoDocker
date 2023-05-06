# App Apache PHP

## Description

This project is a basic application with Apache and PHP made to test apache's metric exports methods.

## Pre-requisites

Have Docker and Docker Compose installed (Tested on version 20.10.23).

Create .env file with the following variables:

```shell
UID=...
GID=...
```

Add your user and group id to the `UID` and `GID` variables respectively (you can use `id -u` to get your user id and `id -g` to get your group id).

## Development

### Run Docker Compose

```shell
docker compose up -d
```

### Make Changes to the Dockerfile

If you make any changes to the Dockerfile, you need to rebuild the image. To do it and start the containers, use the following command:

```shell
docker composer up -d --build
```

### Access to Container

To access the container, use the following command:

```shell
docker exec -it -u user:user app-php bash
```

This command will start an interactive shell session (bash) in the container named app-php with the user user:user. The -u option specifies the user and group IDs that the container process should run as.

## References

- [docker exec documentation](https://docs.docker.com/engine/reference/commandline/exec/)