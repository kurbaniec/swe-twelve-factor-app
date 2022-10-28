<div align="center">
  <p>Kacper Urbaniec | SWE | 28.10.2022</p>
  <p><ins>Assignment 2: twelve-factor-app</ins></p>
  <h1><code>dogorcat-service</code></h1>
</div>

<h4 align="center">Microservice for dog/cat image classification.</h4>

## 🗒️About



## 🐋 Docker Image

### Use Prebuilt Image (Recommended)

```bash
docker pull ghcr.io/kurbaniec/dogorcat-service:main
```

### Build Image

Building locally can take a while (around 15 minutes).

```
docker build -t dogorcat-service .
```

## 🚀 Run

### Basic Usage

1. Create docker network

   ```bash
   docker network create dogorcat-net
   ```

2. Create & run database container

   ```bash
   docker run -itd -p 5432:5432 --name dogorcat-db -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=datasets --network=dogorcat-net postgres
   ```

   > This creates a postgres database that is used by default by the dogorcat service. The default URL of the database is `postgres://postgres:postgres@dogorcat-db/datasets`.

3. Create & run dogorcat service 

   ```bash
   docker run -it -p 8000:8000 --name dogorcat -e RUN_MIGRATION=true --network=dogorcat-net ghcr.io/kurbaniec/dogorcat-service:main
   ```

   > The environment variable `RUN_MIGRATION=true` is needed to perform a basic data migration that creates the required tables in the database. If the database does not use the settings mentioned in step 2, the environment variable `DATABASE_URL=postgres://[user[:password]@][netloc][:port][/dbname]` must also be set. 
   >
   > The web framework used by the dogorcat service can also be set via environment flags, as described in the [documentation](https://rocket.rs/v0.5-rc/guide/configuration/). If nothing is set, the container uses appropriate configuration values.

4. Test functionality via script

   ```bash
   chmod +x ./scripts/dogorcat.service.curl.sh
   ./scripts/dogorcat.service.curl.sh
   ```

   > The script requires [`curl`](https://curl.se/) and [`jq`](https://stedolan.github.io/jq/)

   The script tests most of the functions of the microservice. It loads a Tensorflow model for image classification, classifies cat images, tests the Tensorflow model swapping, classifies dog images and finally deletes all stored Tensorflow models from the database.

### Load Balancing

It is assumed that the section "Basic Usage" has already been carried out and a database container with a performed data migration is already running.



### ✨ Features

# swe-twelve-factor-app

## Features

POST /dogorcat/

* Multipart Image
* { isDog: true / false, isCat: true / false, emoji: 🐶 / 🐱}

GET /datasets
* Return all meta info of all models

GET /dataset/<id>
* Download model

POST /dataset
* Multipart FIle
* Return meta info new model

POST /dataset/latest/<id>

* Set latest datasets
* Must exist beforehand

PUT /dataset/<id>
* Set model to use in AI model
* Must exist beforehand

PUT /dataset/latest

* Set mode to latest AI model
* Must exist beforehand

DELETE /dataset/<id>

* Deletes specific dataset

DELETE /datasets

* Deletes all datasets

## Start

Docker

```
docker run -itd -p 5432:5432 --name dogorcat-db -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=datasets --network=dogorcat-net postgres
```

```
docker start app-db
```

```
psql -h localhost -U postgres
```

TODO create database on start

```
create database "app-db";
```

Application

env variables

* `DATABASE_URL=postgres://postgres:postgres@localhost/app-db`
* `ROCKET_LIMITS={form=100000000,forms=100000000,data-form=100000000,file=100000000}`

## Container TODO



```
docker network create dogorcat-net
```

postgres comes here

```
docker build -t dogorcat-service .
```

```
docker run -it -p 8000:8000 --name dogorcat -e RUN_MIGRATION=true  --network=dogorcat-net dogorcat-service
```





```
sudo apt-get install libpq-dev
cargo install diesel_cli --no-default-features --features postgres
```

```
export DATABASE_URL=postgres://postgres:postgres@localhost/app-db
```

https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-cpu-linux-x86_64-2.9.1.tar.gz

## TODO

* CI

  * Builder Docker Image with Diesel CLI
  * Run Diesel CLI on startup

  * Add exemplary publish config

* Describe how publish would work, add local script with env variables

* HA Proxy Docker Configuration
  * Provide scripts

* Tensorflow Image Detection
  * Woof!
* Dev Prod Parity
  * Describe usage of WSL2 => native Linux!
  * Also Docker abstracts Linux!
* Admin Process
  * Diesel CLI

Build Model

* https://colab.research.google.com/drive/1VQAiIAn81PlAPC6v1b-KxYM5LAB-wxto?usp=sharing



TensorFlow Sources

* https://aralroca.com/blog/cat-dog-classifier
* https://github.com/aralroca/cat-dog-detection-tfjs/tree/master/public/model



Pictures

* Private Archive
* https://pixabay.com/photos/cat-kitten-pets-animals-housecat-2934720/
* https://pixabay.com/photos/cat-domestic-animal-puss-shorthair-3113513/
* https://pixabay.com/photos/corgi-dog-pet-canine-rain-animal-4415649/
* https://pixabay.com/photos/cocker-spaniel-puppy-pet-canine-2785074/
* https://pixabay.com/photos/cat-pet-feline-animal-fur-sleep-4189697/



#### Fix Debugging With Tensorflow in CLion + Rust Plugin

Related Issue: https://github.com/intellij-rust/intellij-rust/issues/8711

Run following command in folder

```
find . -name libtensorflow_framework.so.2
```

Add environment variable with key `LD_LIBRARY_PATH` and value `<path-to-folder>/<find-result>`.

This could look like `LD_LIBRARY_PATH=/home/user/work/twelve-factor-app/target/debug/build/tensorflow-sys-07e5405e44850cf8/out/libtensorflow-cpu-linux-x86_64-2.9.1/lib`.

