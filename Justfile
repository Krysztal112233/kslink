default:
  @just --choose

build:
    just release-backend
    just release-frontend
    just release-migration

build-container:
    docker compose build

release-backend:
    cargo build -r -p kslink-backend

release-frontend:
    dx build -p kslink-frontend --platform web -r

release-migration:
    cargo build -r -p migration

watch-backend:
    docker compose up postgres redis -d
    watchexec -e rs -r cargo run -p kslink-backend

watch-frontend:
    dx serve -p kslink-frontend --platform web

clean:
    cargo clean
