FROM rust:latest as yew-builder
RUN rustup target add wasm32-unknown-unknown
WORKDIR /usr/src
RUN cargo install trunk
RUN USER=root cargo new daiklave-yew
WORKDIR /usr/src/daiklave-yew/
COPY ./daiklave-yew/Cargo.toml .
RUN cargo build --release
RUN rm ./src/*.rs
COPY ./daiklave-yew/Trunk.toml .
COPY ./daiklave-yew/src ./src
COPY ./daiklave-yew/index.html .
COPY ./daiklave-yew/tailwind.config.js .
COPY ./daiklave-yew/tailwind.css .
RUN rm ./target/release/deps/daiklave_yew*
RUN trunk build --release --dist dist

FROM rust:latest as axum-builder
WORKDIR /usr/src
RUN USER=root cargo new daiklave-axum
WORKDIR /usr/src/daiklave-axum/
COPY ./daiklave-axum/Cargo.toml .
RUN cargo build --release
RUN rm ./src/*.rs
COPY ./daiklave-axum/src ./src
RUN rm ./target/release/deps/daiklave_axum*
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*
EXPOSE 3000
ENV TZ=Etc/UTC \
    APP_USER=appuser
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p /usr/src/app
COPY --from=axum-builder /usr/src/daiklave-axum/target/release/daiklave-axum /usr/src/app/daiklave
COPY --from=yew-builder usr/src/daiklave-yew/dist /usr/src/app/assets
USER $APP_USER
WORKDIR /usr/src/app
CMD ["./daiklave"]