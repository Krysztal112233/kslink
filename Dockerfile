FROM docker.io/library/rust:slim-trixie AS builder
WORKDIR /builder
RUN apt update && apt install build-essential curl wget file libssl-dev pkg-config -y
COPY . .
RUN cargo fetch --locked
RUN cargo build --all -r --exclude kslink-frontend

FROM docker.io/library/debian:trixie-slim AS migration
WORKDIR /app
COPY --from=builder /builder/target/release/migration /app/
CMD [ "./migration" ]

FROM docker.io/library/debian:trixie-slim AS backend
WORKDIR /app
COPY --from=builder /builder/target/release/kslink-backend /app/
CMD [ "./kslink-backend" ]

FROM builder AS frontend-builder
ARG KSLINK_BASE_URL
ENV KSLINK_BASE_URL=${KSLINK_BASE_URL}
RUN (curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh  | bash) && cargo binstall dioxus-cli@0.7.0-alpha.3 --force
RUN dx build -r -p kslink-frontend

FROM docker.io/library/caddy:alpine AS frontend
WORKDIR /app
COPY --from=frontend-builder /builder/target/dx/kslink-frontend/release/web/public/ .
EXPOSE 9000
