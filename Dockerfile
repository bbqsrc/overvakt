FROM --platform=$BUILDPLATFORM rust:1-alpine AS build

RUN apk upgrade --update-cache --available && \
    apk add g++ && \
    rm -rf /var/cache/apk/*

WORKDIR /app
COPY . /app

RUN cargo clean && cargo build --release

FROM scratch

WORKDIR /usr/src/overvakt

COPY ./res/assets/ ./res/assets/
COPY --from=build /app/target/release/overvakt /usr/local/bin/overvakt

CMD [ "overvakt", "-c", "/etc/overvakt.toml" ]

EXPOSE 8080
