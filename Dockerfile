#########
# Build #
#########
FROM rust:1.56 as build
RUN USER=root cargo new --bin redis-over-http
WORKDIR /redis-over-http
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src
RUN rm ./target/release/deps/redis-over-http*
RUN cargo build --release

########
# Prod #
########
FROM rust:1.56-slim-buster
COPY --from=build /redis-over-http/target/release/redis-over-http .
CMD ["./redis-over-http"]