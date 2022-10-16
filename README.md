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

GET /models
* Return all meta info of all models

GET /model/<id>
* Download model

POST /model
* Multipart FIle
* Return meta info new model

PUT /model/<id>
* Set model to use in AI model
* Must exist beforehand

PUT /model/active

* Set mode to latest AI model
* Must exist beforehand

## Start

Docker

```
docker run -itd -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -v /app-db/postgresql/data:/var/lib/u-service/postgresql/data --name app-db postgres
```

```
psql -h localhost -U postgres
```

TODO create database on start

```
create database "app-db";
```

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



TensorFlow Sources

* https://aralroca.com/blog/cat-dog-classifier
* https://github.com/aralroca/cat-dog-detection-tfjs/tree/master/public/model