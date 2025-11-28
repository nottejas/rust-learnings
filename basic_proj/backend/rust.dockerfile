FROM rust:1.82 as builder

WORKDIR /app

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

COPY . .

RUN cargo build --release


# production
FROM debian:bookworm-slim

WORKDIR /usr/local/bin

# Copy the correct binary
COPY --from=builder /app/target/release/backend .

# Install Postgres TLS dependency required by postgres crate
RUN apt update && apt install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

CMD ["./backend"]

