FROM docker.io/library/rust:slim-trixie AS builder
WORKDIR /builder
RUN apt update && apt install build-essential -y
COPY . .
RUN cargo build --all -r

FROM docker.io/library/debian:trixie-slim AS migration
WORKDIR /app
COPY --from=builder /builder/target/release/migration /app/
CMD [ "./migration" ]

FROM docker.io/library/debian:trixie-slim AS backend
WORKDIR /app
COPY --from=builder /builder/target/release/kslink-backend /app/
CMD [ "./kslink-backend" ]
