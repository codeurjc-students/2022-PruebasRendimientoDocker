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

## First Steps

After cloning the repository, you need to install the dependencies. For this you have two options:

### Install Dependencies With PHP on Your System

```shell
php composer.phar install
```

### Install Dependencies With PHP on Docker

1. Start docker-compose

    ```shell
    docker compose up -d
    ```

2. Access the container

    ```shell
    docker exec -it -u user:user app-php bash
    ```

3. Install dependencies

    ```shell
    php composer.phar install
    ```

## Development

### Run The Application

For this use docker-compose:

```shell
docker compose up -d
```

After this, you can access the application on `http://localhost:8080`

### Install or Update Dependendencies

```shell
# install dependencies based in composer.lock
php composer.phar install

# Or

# update dependencies based in composer.json
php composer.phar update  
``` 

### Make Changes to the Dockerfile

If you make any changes to the Dockerfile, you need to rebuild the image. To do it and start the containers, use the following command:

```shell
docker compose up -d --build
```

### Access the Container

To access the container, use the following command:

```shell
docker exec -it -u user:user app-php bash
```

This command will start an interactive shell session (bash) in the container named app-php with the user user:user. The -u option specifies the user and group IDs that the container process should run as.

### Laminas Development Mode

This functionality comes with [`laminas-development-mode`](https://github.com/laminas/laminas-development-mode)

```shell
php composer.phar development-enable  # enable development mode
php composer.phar development-disable # disable development mode
php composer.phar development-status  # whether or not development mode is enabled
```

You may provide development-only modules and bootstrap-level configuration in `config/development.config.php.dist`, and development-only application configuration in `config/autoload/development.local.php.dist`. Enabling development mode will copy these files to versions removing the `.dist` suffix, while disabling development mode will remove those copies.

After making changes to one of the above-mentioned `.dist` configuration files you will either need to disable then enable development mode for the changes to take effect, or manually make matching updates to the `.dist`-less copies of those files.

## References

- [laminas-skeleton repository](https://github.com/laminas/laminas-mvc-skeleton#readme)
- [docker exec documentation](https://docs.docker.com/engine/reference/commandline/exec/)