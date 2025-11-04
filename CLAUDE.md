# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based cryptocurrency trading analysis and monitoring web service. The application uses Actix-web framework with Prometheus metrics, performs technical analysis on cryptocurrency data from Binance, and sends notifications via Lark (飞书) webhooks.

## Development Commands

### Build and Run
- **Development build**: `cargo build`
- **Release build**: `cargo build --release`
- **Run locally**: `cargo run`
- **Run in background**: `nohup ./target/release/rust_web >> ./rust_web.log &`

### Code Quality
- **Linting**: `cargo clippy` (already configured in dependencies)
- **Testing**: Use standard `cargo test` for unit tests
- **Check formatting**: `cargo fmt --check`

### Monitoring
- **Web server**: Runs on `127.0.0.1:8080/`
- **Metrics endpoint**: `http://127.0.0.1:8080/metrics`
- **Application logs**: Check `./rust_web.log` for background runs
- **System logs**: `log/system.log`
- **Rolling logs**: `log/roll.log`

## Architecture

### Core Components

1. **Main Application** (`src/main.rs`)
   - Initializes logging with `log4rs.yaml`
   - Sets up signal handlers for graceful shutdown
   - Starts the scheduler and web server

2. **App Structure** (`src/app/mod.rs`)
   - `web`: Actix-web HTTP server and API endpoints
   - `schedule`: Background task scheduler for periodic analysis
   - `exchange`: Binance API client for market data
   - `trading`: Technical analysis and MACD crossover detection
   - `lark`: Notification system via Lark webhooks
   - `config`: Configuration management from `config.toml`
   - `utils`: Prometheus metrics integration

3. **Configuration** (`config.toml`)
   - Server IP and port settings
   - API keys and webhook URLs for different time intervals (1w, 1d, 4h)
   - Use `config::config()` to access configuration globally

4. **Scheduled Tasks** (`src/app/schedule/mod.rs`)
   - **1h job**: Analyzes 4h intervals for MACD crossovers
   - **8h job**: Analyzes 1d intervals for MACD crossovers
   - **24h job**: Analyzes 1w intervals and ARH999 index
   - Monitor thread runs continuously for system health

### Technical Analysis Features

- **MACD Crossover Detection**: Identifies bullish/bearish signals using MACD lines (12, 26, 9 parameters)
- **Trend Analysis**: Detects consecutive rising/falling patterns in k-lines and MACD
- **High/Low 9 Trend**: Custom indicator for identifying market extremes
- **Chart Generation**: Creates PNG charts for MACD analysis (using plotters crate)

### Data Flow

1. Scheduler triggers periodic analysis jobs
2. Binance client fetches k-line data via `get_klines_v3()`
3. Technical analysis calculates MACD indicators
4. Crossover and trend detection algorithms analyze patterns
5. Lark notifications sent based on configured webhook intervals
6. Prometheus metrics track application performance

## Key Implementation Details

- **Concurrent HashMap**: Uses `dashmap` for thread-safe trend state tracking
- **Async Runtime**: Built on `tokio` with async/await patterns
- **Error Handling**: Uses `anyhow` for error propagation throughout the application
- **Decimal Arithmetic**: Uses `rust_decimal` for precise financial calculations
- **Time Handling**: Uses `chrono` for date/time operations and chart naming

## Testing Strategy

The project includes benchmarking code in `lib.rs` for performance testing of different data structures (HashMap vs Arc<Mutex<>> vs Rc<Mutex<>>). When adding new features, ensure proper error handling and consider the async nature of the application.