FROM rustlang/rust:1.64-buster AS build

RUN apt-get update
RUN apt-get install -y musl-tools

RUN rustup --version
RUN rustup target add x86_64-unknown-linux-musl

RUN rustc --version && \
    rustup --version && \
    cargo --version

WORKDIR /app
COPY . /app
RUN cargo clean && cargo build --release --target x86_64-unknown-linux-musl
RUN strip ./target/x86_64-unknown-linux-musl/release/overvakt

FROM scratch

WORKDIR /usr/src/overvakt

COPY ./res/assets/ ./res/assets/
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/overvakt /usr/local/bin/overvakt

CMD [ "overvakt", "-c", "/etc/overvakt.toml" ]

EXPOSE 8080
