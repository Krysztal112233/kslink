<div align="center">

<h1> KSLink </h1>

✨Dead Simple Self-hosting Short Link Service Powered by Rust✨

</div>

## Deploy

### Docker Compose

Modify `KSLINK_BASE_URL` in `.env` file into you domain, and run `docker compose up`.

The frontend service will run at `:9000` and the backend will run at `:8000` by default

> **Note**
>
> You have to run `docker compose build` after you edit the `.env` file for rebuilding the image.

### Build

You have to install the latest Rust version. And also I suggest you install [`just`](https://github.com/casey/just) command for better build experience

```bash
just
```

Choose `build`, or directly run `just build`

## Configuration

Please see `kslink.toml` file for more detail

### Database

This project also support SQLite but I've nerver tested it.

Just replace `database.url` in the `kslink.toml` file into `sqlite://app.db?journal_mode=WAL&cache=shared`, in the docker the SQLite database file will be stored at `/app/app.db`

### Redis

This project also support Redis, but if Redis unavaliable it will fallback to in memory cache called `Moka`, so if you doesn't need Redis or wanna keep you deployment stack clean you can run it without Redis.
