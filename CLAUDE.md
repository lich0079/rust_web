# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run Commands

```bash
cargo build --release          # Build release binary
cargo run                      # Run in development mode
./start.sh                     # Build release and run with log output to rust_web.log
cargo clippy                   # Lint / code analysis
cargo test                     # Run tests (no active test suite currently)
```

## Architecture

This is a Rust cryptocurrency monitoring application that tracks BTC/ETH technical indicators via Binance API and sends alerts to Lark (Feishu) bot webhooks.

### Core Flow

`main.rs` boots an **actix-web** HTTP server (port 8080) with Prometheus metrics, and spawns a **scheduler** that runs periodic analysis jobs on tokio:

- **1h/8h/24h jobs** (`schedule/mod.rs`) — fetch kline data from Binance, compute MACD crossovers, check price trends, and send alerts
- Scheduler uses `thread::sleep` in a tokio task with manual interval tracking (not cron-based)

### Module Map

- **`app/web`** — Actix-web server with `/`, `/hello/{name}`, `/metrics` endpoints
- **`app/exchange/binance`** — Binance REST API client (`get_klines_v3`, `get_ticker`, `get_symbols`)
- **`app/trading/crossover`** — MACD calculation (via `ta` crate), crossover detection, kline trend analysis, High-9 counting. Uses a static `TREND_MAP` to track previous trend state
- **`app/trading/find`** — Additional trading analysis utilities
- **`app/arh999`** — Fetches BTC ahr999 index from flink1.com API, alerts when index < 0.8
- **`app/lark/lark_client`** — Lark bot webhook sender. Routes messages to different webhooks by interval (1w/1d/4h/default)
- **`app/config`** — Loads `config.toml` into a global `Lazy<RwLock<Config>>` singleton
- **`app/utils/metrics`** — Prometheus metrics integration

### Key Patterns

- Global config via `once_cell::Lazy` + `RwLock` — access with `config::config()`
- Logging via `log` + `log4rs` (configured in `log4rs.yaml`)
- All external API calls use `reqwest` async client
- Decimal prices from Binance use `rust_decimal` with `serde` string deserialization
- Kline struct is a tuple struct matching Binance's array response format (fields accessed by index: `.2` = high, `.3` = low, `.4` = close)

### Configuration

`config.toml` contains Lark webhook IDs and API keys. The `settings.lark_bot_webhook_*` fields map to different alert channels by timeframe.
