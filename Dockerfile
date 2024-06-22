FROM rust:1.79
WORKDIR /app
COPY . .
RUN cargo build --release
ENTRYPOINT ["./target/release/pizza_dough_calculator", "--serve", "--host=0.0.0.0"]