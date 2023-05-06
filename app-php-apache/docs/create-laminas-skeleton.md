# Laminas Skeleton

## Introduction 

This is a step by step guide to configure a Laminas skeleton application using a docker container. Check the [documentation](../README.md) for more information about the project.

## Pre-requisites

Before proceeding with the installation, please ensure that you have met the pre-requisites listed in the [documentation](../README.md#pre-requisites).

## Steps

### 1. Start Docker Composer

First of all, create a folder to store the application files. You might check where the volume is saved on your `docker-composer.yml` file. In this case, the volume is saved on the `app` folder.

```shell
mkdir app
```

Run docker-compose in detach mode

```shell
docker compose up -d
```

> **Note**: If you don't create the folder, Docker will create it for you with it's own permissions so you might have problems with the followings steps.

### 2. Access to Container

For the next steps, you need to access to the container. You can do it with the following command:

```shell
docker exec -it -u user:user app-php bash
```

This command will open a Bash terminal inside the container with the user `user` and the group `user`, which is the owner of the volume.

> **Note**: Make sure you have created an .env file with your user and group information. For more information visit the [documentation](create-php-apache-docker.md).

### 3. Create composer.phar

Inside the container, use the following commands to create the `composer.phar`

```shell
# Copy composer-setup.php
php -r "copy('https://getcomposer.org/installer', 'composer-setup.php');"

# Verify composer-setup.php and if it is ok, paste it
php -r "if (hash_file('sha384', 'composer-setup.php') === '55ce33d7678c5a611085589f1f3ddf8b3c52d662cd01d4ba75c0ee0459970c2200a51f492d557530c71c15d8dba01eae') { echo 'Installer verified'; } else { echo 'Installer corrupt'; unlink('composer-setup.php'); } echo PHP_EOL;"

# Create composer.phar
php composer-setup.php

# Remove composer-setup.php
php -r "unlink('composer-setup.php');"
```

### 4. Init Laminas Skeleton Application

Install the Laminas Skeleton Application in a new folder

```shell
php composer.phar create-project -n -sdev laminas/laminas-mvc-skeleton app
```

### 5. Move Files To Current Folder

To make it easier to work with the files, move all the files to the current folder and remove the app folder.

```shell
mv -n app/{.,}* . && rmdir app
```

## Conclusion

In this guide, you have learned how to configure a Laminas skeleton application using a Docker container.

## References

- [Laminas Skeleton Repository](https://github.com/laminas/laminas-mvc-skeleton#installation-using-composer)
- [Download Composer](https://getcomposer.org/download/)
- [Change composer version](https://stackoverflow.com/a/64598028)
