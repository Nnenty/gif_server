# I took this dockerfile config from https://github.com/Desiders/get_anime_bot_rs

FROM debian:bullseye-slim AS base
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get purge -y --auto-remove -o APT::AutoRemove::RecommendsImportant=false

FROM rust:1.80.1-slim-bullseye AS build
RUN apt-get update \
    && apt-get install -y --no-install-recommends libssl-dev \
    && apt-get install -y --no-install-recommends pkg-config \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get purge -y --auto-remove -o APT::AutoRemove::RecommendsImportant=false
WORKDIR /usr/src/app
RUN USER=root cargo init
COPY ./Cargo.toml .
RUN cargo build --release
COPY ./src ./src
RUN touch src/main.rs && cargo build --release

FROM base AS final
WORKDIR /app
COPY ./public/ ./public/
COPY ./config.toml ./config.toml
COPY --from=build /usr/src/app/target/release/gif_server .
VOLUME /configs
ENV RUST_BACKTRACE=full
ENTRYPOINT ["/app/gif_server"]