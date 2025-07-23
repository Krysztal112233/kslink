FROM docker.io/library/rust:slim-trixie AS builder
RUN cargo install cargo-pgo
WORKDIR /builder
RUN apt update && apt install build-essential llvm-bolt -y
COPY . .
RUN cargo build --all -r && cargo pgo build -- --all

FROM docker.io/library/debian:trixie-slim AS migration
WORKDIR /app
COPY --from=builder /builder/target/release/migration /app/
CMD [ "migration" ]

FROM docker.io/library/debian:trixie-slim AS backend-pgo
WORKDIR /app
COPY --from=builder /builder/target/x86_64-unknown-linux-gnu/release/kslink-backend /app/
CMD [ "kslink-backend" ]

FROM docker.io/library/debian:trixie-slim AS backend
WORKDIR /app
COPY --from=builder /builder/target/release/kslink-backend /app/
CMD [ "kslink-backend" ]
