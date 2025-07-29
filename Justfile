default:
  @just --choose

build:
    just release-backend

build-container:
    docker compose build

release-backend:
    cargo build -r -p kslink-backend

watch-backend:
    docker compose up postgres redis -d
    watchexec -e rs -r cargo run -p kslink-backend

clean:
    cargo clean
