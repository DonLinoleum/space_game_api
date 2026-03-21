
FROM rust:1.94 as builder

WORKDIR /app
COPY Cargo.toml ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --release

COPY src ./src
COPY migrations ./migrations

RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/space_game_backend .
COPY --from=builder /app/migrations ./migrations

ENTRYPOINT ["./space_game_backend"]