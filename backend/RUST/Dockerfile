FROM rustlang/rust:nightly-slim as build
WORKDIR /backend/eval-api

RUN USER=root cargo new --bin evaluation-api

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release

RUN rm src/*.rs

COPY ./src ./src

COPY ./.env ./.env

RUN rm ./target/release/deps/evaluation-api*
RUN cargo build --release

FROM rustlang/rust:nightly-slim
COPY --from=build /evaluation-api/target/release/evaluation-api .

CMD ["./evaluation-api"]
