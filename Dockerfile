FROM rust:1.64 as builder
# Build project dependencies & install migration tool
# Doing this first allows time-saving caching (see: https://stackoverflow.com/a/58474618/12347616)
WORKDIR /usr/src/dogorcat-app
COPY dummy.rs .
COPY Cargo.toml .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
RUN cargo install --version 2.0.1 diesel_cli --no-default-features --features postgres
# Download and unpack tensorflow library
ADD https://storage.googleapis.com/tensorflow/libtensorflow/libtensorflow-cpu-linux-x86_64-2.9.1.tar.gz /usr/src/dogorcat-app/tf/
RUN tar -xzf tf/libtensorflow-cpu-linux-x86_64-2.9.1.tar.gz --directory tf/ && rm tf/libtensorflow-cpu-linux-x86_64-2.9.1.tar.gz
# Copy remaining files and build & install service
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
WORKDIR /usr/src/dogorcat-app
# Install database dependencies
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
# Copy needed files from previous build stage
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=builder /usr/local/cargo/bin/twelve-factor-app /usr/local/bin/twelve-factor-app
COPY --from=builder /usr/src/dogorcat-app/tf/ /usr/src/dogorcat-app/tf/
COPY --from=builder /usr/src/dogorcat-app/migrations /usr/src/dogorcat-app/migrations
COPY --from=builder /usr/src/dogorcat-app/entrypoint.sh /usr/src/dogorcat-app/entrypoint.sh
# Make entrypoint script executable
RUN chmod +x ./entrypoint.sh
# Configure and start service
CMD ./entrypoint.sh