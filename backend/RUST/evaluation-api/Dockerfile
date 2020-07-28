FROM rustlang/rust:nightly as build
WORKDIR /backend/evaluation-api

RUN USER=root cargo init --bin

RUN USER=root apt-get update;USER=root apt-get install libpq-dev -y

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release

RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/evaluation_api*
RUN cargo build --release

FROM rustlang/rust:nightly-slim

RUN USER=root apt-get update;USER=root apt-get install libpq-dev -y

COPY --from=build /backend/evaluation-api/target/release/evaluation-api .

COPY ./.env-docker ./.env

COPY ./wait-for.sh ./wait-for.sh

CMD ["./evaluation-api"]
