FROM rust:alpine AS builder

# Adding necessary packages
RUN apk update --no-cache \
    && apk add --no-cache \
                pkgconfig \
                openssl   \ 
                openssl-dev \
                musl-dev

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup toolchain install stable-x86_64-unknown-linux-musl

WORKDIR /app
COPY Cargo.toml Cargo.lock ./

COPY ./src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM scratch

LABEL org.opencontainers.image.source="https://github.com/gldraphael/clippy"
LABEL org.opencontainers.image.description="A simple hello world application."

ENV \
    # Configure app to bind to port 80
    APP_PORT=80

EXPOSE 80

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/clippy /app/

# CMD ["sleep", "infinity"]
ENTRYPOINT ["./clippy"]
