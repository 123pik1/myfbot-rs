# myfbot-rs

`myfbot-rs` is a Discord bot written in Rust. It provides Discord slash commands and supports local text generation through an Ollama API.

## Commands

- `/age [user]` - shows when a user's Discord account was created.
- `/ask <prompt>` - sends a prompt to the configured Ollama model.
- `!ping` - responds with `Pong!`.
- `myf` - responds with `myf myf`.

## Configuration

Set these environment variables before starting the bot:

| Variable | Required | Default |
| --- | --- | --- |
| `DISCORD_TOKEN` | Yes | - |
| `DISCORD_ALLOWED_USER` | No | `0` |
| `OLLAMA_MODEL` | No | `Qwen2.5` |
| `OLLAMA_HOST` | No | `http://127.0.0.1:11111` |

`DISCORD_ALLOWED_USER` is reserved for application-level user checks and should contain a Discord user ID.

## Running

With Rust and Cargo:

```sh
cargo run --release
```

Ollama must be running and the selected model must be available locally.

With Nix:

```sh
nix develop
cargo run --release
```

Build a standalone package with:

```sh
nix build
./result/bin/myfbot-rs
```

## Development

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```
