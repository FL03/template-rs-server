FROM scratch AS workspace

# Create a new workdir
WORKDIR /src
# Copy the current directory to the workdir
ADD . .

FROM rust:latest AS builder-base

ARG TOOLCHAIN=stable

RUN apt-get update -y && \
    apt-get upgrade -y

RUN rustup default ${TOOLCHAIN} && \
    rustup update

FROM builder-base AS builder

# Create a new workdir
WORKDIR /workspace
# Copy the current directory to the workdir
ADD . .
# Build the project
RUN cargo build --all-features -r -v

FROM debian:stable-slim AS runner-base

# Update the package list and upgrade the packages
RUN apt-get update -y && \
    apt-get upgrade -y

FROM runner-base AS runner

# Set the environment variables
ENV PZZLD_MODE=release \
    PZZLD_PORT=8080 \
    RUST_LOG=trace
# Expose the port
EXPOSE ${PZZLD_PORT}
# Create a new workdir
WORKDIR /etc/pzzld
# Copy the binary from the builder
COPY --from=builder /workspace/target/release/pzzld /bin/pzzld
# Copy the configuration file
COPY --from=workspace /src/.config/docker.config.toml /etc/pzzld/Puzzled.toml
# Set the permissions
RUN chmod +x /bin/pzzld
# Set the entrypoint
ENTRYPOINT [ "pzzld" ]
