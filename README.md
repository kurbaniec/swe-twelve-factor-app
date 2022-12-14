<div align="center">
  <p>Kacper Urbaniec | SWE | 28.10.2022</p>
  <p><ins>Assignment 2: twelve-factor-app</ins></p>
  <h1><code>dogorcat-service</code></h1>
</div>

<h4 align="center">Microservice for dog/cat image classification.</h4>

## 🗒️About

The project depicts a microservice that is intended to reflect a [twelve-factor app](https://12factor.net/).

The idea behind the service itself, an image classifier that outputs whether an image contains a dog or a cat, was to learn a bit about Tensorflow and machine learning, and in particular how to implement it in the Rust programming language.

At the moment, machine learning support in Rust is very limited. It is not recommended to create models with Rust, but to use existing models in Rust-based web frameworks or other applications. This way, one can comfortably create models in more mature environments like Python with Keras and Tensorflow and use the exported models in performant Rust applications with the [Tensorflow Rust Library](https://github.com/tensorflow/rust).

The Tensorflow model, `models/dogorcat.zip`, used for image classification is based on [Lawrence Moroney's (Google) notebook](https://colab.research.google.com/github/lmoroney/dlaicourse/blob/master/Course%202%20-%20Part%202%20-%20Lesson%202%20-%20Notebook.ipynb) with [Kaggle's Cats Vs Dogs dataset](https://www.kaggle.com/c/dogs-vs-cats/data). If one wants to create the model oneself (and tweak it even further), I have created a [forked version of the original notebook](https://colab.research.google.com/drive/1VQAiIAn81PlAPC6v1b-KxYM5LAB-wxto?usp=sharing) that includes an additional "Save Model" section where the Tensorflow model can easily be exported and used in this project.

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

   > The script requires [`curl`](https://curl.se/) and [`jq`](https://stedolan.github.io/jq/).

   The script tests most of the functions of the microservice. It loads a Tensorflow model for image classification, classifies cat images, tests the Tensorflow model swapping, classifies dog images and finally deletes all stored Tensorflow models from the database.
   
   > Additionally, a postman collection can be found in the `scripts` folder to manually interact with microservice.

### Load Balancing

It is assumed that the section "Basic Usage" has already been carried out and a database container with all required tables created is already running.

1. Start one dogorcat serivce and load initial Tensorflow model

   ```bash
   docker run -it --name dogorcat1 --network=dogorcat-net -p 9000:8000 ghcr.io/kurbaniec/dogorcat-service:main
   ```

   ```bash
   curl -F data="@./models/dogorcat.zip;type=application/zip" -F latest='true' http://localhost:9000/dataset
   ```

2. Start two more dogorcat services

   ```bash
   docker run -it --name dogorcat2 --network=dogorcat-net ghcr.io/kurbaniec/dogorcat-service:main
   ```

   ```bash
   docker run -it --name dogorcat3 --network=dogorcat-net ghcr.io/kurbaniec/dogorcat-service:main
   ```

3. Start haproxy load balancer

   ```bash
   docker run -it --name dogorcat-haproxy --net dogorcat-net -v $(pwd):/usr/local/etc/haproxy:ro -p 8000:8000 -p 8404:8404 haproxytech/haproxy-alpine:2.4
   ```

4. Interact with the services

   Execute image classification like the following multiple times.

   ```bash
   curl -F image="@./images/dog1.jpg" http://localhost:8000/dogorcat
   ```

   Checkout out the printed logs or inspect the load balancer dashboard under http://localhost:8404/.

### Cleanup

```bash
docker stop dogorcat && docker rm dogorcat
docker stop dogorcat-db && docker rm dogorcat-db
docker stop dogorcat1 && docker rm dogorcat1
docker stop dogorcat2 && docker rm dogorcat2
docker stop dogorcat3 && docker rm dogorcat3
docker stop dogorcat-haproxy && docker rm dogorcat-haproxy
docker network rm dogorcat-net
docker image rm ghcr.io/kurbaniec/dogorcat-service:main
```

## ✨ Features

### Terms

The term "latest" in the context of this project means a Tensorflow model stored in the database that is loaded by default when the dogorcat service starts. There can only be one model where the parameter "latest" is set to true. 

The term "dataset" is used in this project as a synonym for Tensorflow models.

### API

`POST /dogorcat/`

* Checks if image contains a dog or a cat
* Input:
  * multipart/form-data
  * `image` key with image file as value
* Output: `{ isDog: true / false, isCat: true / false, emoji: 🐶 / 🐱}`

`GET /datasets`

* Returns meta info of all stored tensorflow models
* Output: `[{ "id": <id>, "latest": true / false, "created_on": "<timestamp>" }, ...]`

`GET /dataset/<id>`

* Downloads tensorflow model
* Input:
  * path parameter
  * `id` of the to model to download


`POST /dataset`

* Uploads a tensorflow model and optionally sets it as "latest"
* If the key `latest` is set to true, not only is the model saved as "latest" in the database, but also the currently loaded Tensorflow model in the service is replaced by the uploaded model
* Input:
  * multipart/form-data
  * `data` key with zipped tensorflow model as value
  * (Optional) `latest` key with possible values "true" / "false"
* Output: `{ "id": <id>, "latest": true / false, "created_on": "<timestamp>" }`

`POST /dataset/latest/<id>`

* Sets an existing tensorflow model as "latest" in the database
* Does not replace current Tensorflow model in the service
* Input:
  * path parameter
  * `id` of existing tensorflow model


`PUT /dataset/<id>`

* Replaces the currently loaded Tensorflow model in the service by one retrieved from the database
* Input:
  * path parameter
  * `id` of existing tensorflow model


`PUT /dataset/latest`

* Replaces the currently loaded Tensorflow model in the service by one marked as "latest" from the database

`DELETE /dataset/<id>`

* Deletes specified Tensorflow model from database
* Does not remove current Tensorflow model in the service
* Input:
  * path parameter
  * `id` of tensorflow model to remove

`DELETE /datasets`

* Deletes all Tensorflow models from database
* Does not remove current Tensorflow model in the service

## 🔍 Debug Project in CLion

Rust binaries are typically statically linked in one executable. However, as the Tensorflow Rust library relies on the external Tensorflow C API, it needs to be dynamically linked. This can make problems when trying to debug in the IDE (see related [issue](https://github.com/intellij-rust/intellij-rust/issues/8711)). 

To fix debugging run the following command in the project folder.

```
find . -name libtensorflow_framework.so.2
```

Next add an environment variable with the key `LD_LIBRARY_PATH` and value `<path-to-folder>/<find-result>`.

This could look like `LD_LIBRARY_PATH=/home/user/work/twelve-factor-app/target/debug/build/tensorflow-sys-07e5405e44850cf8/out/libtensorflow-cpu-linux-x86_64-2.9.1/lib`.

## Acknowledgments

* https://www.christianhaller.me/blog/projectblog/2020-06-02-TFCatsVsDogsI
* https://aralroca.com/blog/cat-dog-classifier
* https://github.com/aralroca/cat-dog-detection-tfjs/tree/master/public/model
* https://rocket.rs/
* https://diesel.rs/guides/
* https://github.com/tensorflow/rust
* https://www.haproxy.com/de/blog/how-to-run-haproxy-with-docker

## Pictures

* Private Archive
* https://pixabay.com/photos/cat-kitten-pets-animals-housecat-2934720/
* https://pixabay.com/photos/cat-domestic-animal-puss-shorthair-3113513/
* https://pixabay.com/photos/corgi-dog-pet-canine-rain-animal-4415649/
* https://pixabay.com/photos/cocker-spaniel-puppy-pet-canine-2785074/
* https://pixabay.com/photos/cat-pet-feline-animal-fur-sleep-4189697/

