<div align="center">
  <p>Kacper Urbaniec | SWE | 28.10.2022</p>
  <h1><ins>Assignment 2: twelve-factor-app</ins></h1>
</div>
## Implemented Factors

### Codebase

> One codebase tracked in revision control, many deploys

The codebase and all related assets such as test scripts or the Dockerfile for containerisation are all part of the same project, which is tracked in Git at https://github.com/kurbaniec/swe-twelve-factor-app.

In terms of deploys, on the one hand there is the local development environment that I set up when I developed and tested the application. There is also an environment that mimics a production environment in a sense. When pushing on the `main` branch, a new Docker image is created and published to the GitHub container registry at https://github.com/kurbaniec/swe-twelve-factor-app/pkgs/container/dogorcat-service. 

If it were a real production application, one could simply add further steps to the [CI/CD pipeline of the project](https://github.com/kurbaniec/swe-twelve-factor-app/blob/main/.github/workflows/main.yml), e.g. deploy the created Docker image to the cloud to make it available to users. Also adding more environments like Staging would make sense in real-word application development.

### Dependencies

> Explicitly declare and isolate dependencies

The project explicitly declares dependencies in two files:  [`Cargo.toml`](https://github.com/kurbaniec/swe-twelve-factor-app/blob/main/Cargo.toml) & [`Dockerfile`](https://github.com/kurbaniec/swe-twelve-factor-app/blob/main/Dockerfile).

 `Cargo.toml` is the Rust-equivalent to `NPM`'s `package.json`. It specifies all dependencies that the package manager `cargo` installs & manages for the Rust application.

Rust binaries rely on static linking meaning that the compiled binary includes all dependencies in the executable. However, some used libraries in the project break this convention as they rely on external dependencies that need to be additionally installed & linked during execution.

For one, there is Tensorflow itself, with the Rust library being only a wrapper around the C API thus requiring the prebuilt C API dynamically linked. The used ORM for the project Diesel also requires `libpq-dev`, the C API for PosgreSQL.

As we see `cargo` can manage most of the projects dependencies, explicitly declaring them, however this two outliers break the dependencies principle of the twelve-factor app. This is where the Dockerfile comes into place.

In the Dockerfile the external dependencies are explicitly declared, there is no need to configure anything, the built image takes care of everything. Also, with the usage of Docker images we can be sure that no implicit dependencies are leaked thus all our depencies are properly isolated.



## Acknowledgments

* https://12factor.net/
* https://www.redhat.com/architect/12-factor-app
* https://www.youtube.com/watch?v=REbM4BDeua0
