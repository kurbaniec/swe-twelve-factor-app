FROM rust:1.64 as builder
WORKDIR /usr/src/dogorcat-app
COPY . .
# RUN cargo install diesel_cli --no-default-features --features postgres && cargo install --path .
RUN cargo install --version 2.0.1 diesel_cli --no-default-features --features postgres && cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=builder /usr/local/cargo/bin/twelve-factor-app /usr/local/bin/twelve-factor-app
COPY --from=builder /usr/src/dogorcat-app/migrations /usr/src/dogorcat-app/migrations
ADD https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-cpu-linux-x86_64-2.9.1.tar.gz /usr/src/dogorcat-app/tf/
WORKDIR /usr/src/dogorcat-app/
RUN tar -xzf tf/libtensorflow-cpu-linux-x86_64-2.9.1.tar.gz --directory tf/ && rm tf/libtensorflow-cpu-linux-x86_64-2.9.1.tar.gz
#CMD ["export ROCKET_LIMITS='{form=100000000,forms=100000000,data-form=100000000,file=100000000}' && export DATABASE_URL='postgres://postgres:postgres@localhost/app-db' && twelve-factor-app"]
CMD ["/bin/bash", "-c", "export LD_LIBRARY_PATH=/usr/src/dogorcat-app/tf/lib;export ROCKET_LIMITS='{form=100000000,forms=100000000,data-form=100000000,file=100000000}';export DATABASE_URL='postgres://postgres:postgres@localhost/app-db';diesel migration run;twelve-factor-app"]