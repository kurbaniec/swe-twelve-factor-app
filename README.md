# swe-twelve-factor-app

## Features

POST /isdog/
* Multipart Image
* { isDog: true / false, emoji: 🐶 / ❌🐶 }

POST /iscat/

* Multipart Image
* { isDog: true / false, emoji: 🐱 / ❌🐱 }

POST /isdogorcat/

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
docker run -itd -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -v /app-db/postgresql/data:/var/lib/u-service/postgresql/data --name app-db postgres
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
sudo apt-get install libpq-dev
cargo install diesel_cli --no-default-features --features postgres
```

```
export DATABASE_URL=postgres://postgres:postgres@localhost/app-db
```



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



#### Fix Debugging With Tensorflow in CLion + Rust Plugin

Related Issue: https://github.com/intellij-rust/intellij-rust/issues/8711

Run following command in folder

```
find . -name libtensorflow_framework.so.2
```

Add environment variable with key `LD_LIBRARY_PATH` and value `<path-to-folder>/<find-result>`.

This could look like `LD_LIBRARY_PATH=/home/user/work/twelve-factor-app/target/debug/build/tensorflow-sys-07e5405e44850cf8/out/libtensorflow-cpu-linux-x86_64-2.9.1/lib`.

