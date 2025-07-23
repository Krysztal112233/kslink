#!/usr/bin/bash

sea-orm-cli generate entity -o crates/entity/src/model --with-serde both --database-url=postgres://postgres:postgres@localhost/postgres
