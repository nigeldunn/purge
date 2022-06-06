FROM rust:bullseye as build
WORKDIR /opt/purge

COPY Cargo.toml .
COPY Cargo.lock .
COPY src src/
COPY ../assets assets/

RUN cargo build --release --target=x86_64-unknown-linux-musl


FROM alpine:latest
COPY --from=build /opt/purge/target/release/purge /usr/bin/purge
ENTRYPOINT ["/usr/bin/purge"]