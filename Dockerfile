ARG RUST_VERSION=1.64
FROM rust:${RUST_VERSION}-slim-bullseye as build

# Build environment proxy
# ENV http_proxy http://203.202.141.90:3128

RUN apt update \
    && apt install --yes binutils build-essential pkg-config libssl-dev clang lld git protobuf-compiler \
    && rm -rf /var/lib/{apt,dpkg,cache,log}

RUN cargo build --release

FROM debian:bullseye-slim

ENV http_proxy http://203.202.141.90:3128

RUN apt update \
    && apt install --yes ca-certificates gettext-base libssl1.1 --no-install-recommends \
    && rm -rf /var/lib/{apt,dpkg,cache,log}

COPY --from=build "/target/release/piifilter" "/bin/piifilter"

EXPOSE 8000

CMD ["/bin/piifilter"]