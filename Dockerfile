FROM rust:1.83.0-slim-bullseye AS build

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    curl \
    build-essential

WORKDIR /build

COPY Cargo.lock Cargo.toml ./
COPY src src
COPY entrypoint.sh entrypoint.sh
RUN cargo clean && cargo build --locked --release

FROM debian:bullseye-slim AS final
WORKDIR /app

RUN apt-get update && apt-get install -y \
    build-essential \
    autoconf \
    automake \
    libtool

COPY --from=build /build/target/release/seris /app
COPY --from=build /build/entrypoint.sh /app

USER root
RUN chmod +x entrypoint.sh
RUN addgroup --system --gid 1001 seris
RUN adduser --system --uid 1001 seris

USER seris

CMD ["./entrypoint.sh"]
