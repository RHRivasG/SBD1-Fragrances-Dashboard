FROM rustlang/rust:nightly-slim
WORKDIR /back/
COPY . .
RUN cargo install --path .
CMD ["evaluation-api"]
