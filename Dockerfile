# syntax = docker/dockerfile:1.5

# Build image
FROM rust:bullseye as build

ARG SUBNET_NODE_VERSION="No Version Info"
ARG GIT_BRANCH='No Branch Info'
ARG GIT_COMMIT='No Commit Info'

WORKDIR /src

COPY --link . .

RUN \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target,sharing=private \
       mkdir -p /out /contracts \
    && cd testnet/stacks-node \
    && cargo build --features monitoring_prom,slog_json --release \
    && cp /src/target/release/subnet-node /out

# Run image
FROM debian:bullseye-backports

COPY --from=build /out/ /bin/

CMD ["subnet-node", "start"]
