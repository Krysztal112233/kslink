FROM docker.io/library/rust:slim-trixie AS builder
RUN cargo install cargo-pgo
WORKDIR /builder
RUN apt update && apt install llvm-bolt -y
COPY . .
RUN cargo build --all -r && cargo pgo build -- --all && cargo pgo bolt build

