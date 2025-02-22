FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

ARG RUST_VERSION=1.80.0

FROM rust:1.80-slim

# Set Rust version as build argument
ARG RUST_VERSION=1.80.0

# Install development dependencies and lld linker
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install clang lld \
    && apt-get autoremove -y && apt-get clean -y \
    && rustup install ${RUST_VERSION} \
    && rustup default ${RUST_VERSION} \
    && rustup component add rustfmt clippy
   

# Set working directory
WORKDIR /workspaces/Running_App_Rust

# Copy project files
COPY . .

# Build the project
RUN cargo build

# Default command
CMD ["cargo", "run"]