FROM rust:1.79 AS builder
WORKDIR /app
RUN apt update && apt install mold clang -y
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/pizza_dough_calculator pizza_dough_calculator

ENTRYPOINT ["./pizza_dough_calculator", "--serve", "--host=0.0.0.0"]