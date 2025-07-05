# Latency-X: Helius-Accelerated DeFi HFT Trading Bot (Student Edition)

## Objective

Latency-X is an open-source, plug-and-play High-Frequency Trading (HFT) bot designed to achieve low-microsecond order execution and sub-second on-chain settlement via Qubic. It runs on affordable hardware and hosted RPC endpoints, making it accessible for students and individual developers.

## Features

- **Cross-Exchange/DEX Trading**: Supports Binance, Kraken, and memecoin venues like pump.fun.
- **Ultra-Low-Latency Strategies**:
    - Market-Making
    - Arbitrage
    - MEV (Miner Extractable Value)
- **High-Performance Networking**: Utilizes kernel-bypass networking (DPDK/XDP) for market data and order submission to minimize latency.
- **Qubic Integration**: Leverages Qubic for fast state-channel settlement and messaging.
- **Real-time Tooling**: Includes risk management, structured logging, and metrics for monitoring.
- **Backtesting Framework**: Test your strategies against historical data.

## Technology Stack

- **Core Engine**: Rust for its performance, safety, and modern asynchronous ecosystem.
- **Networking**:
    - `tokio` for asynchronous I/O.
    - WebSocket clients for market data streams.
    - Kernel-bypass is a goal, but current implementation uses standard networking.
- **Configuration**: `serde` and `config` for loading settings from `Config.toml` and environment variables.
- **Observability**:
    - `tracing` for structured logging.
    - `metrics` and `metrics-exporter-prometheus` for exposing a `/metrics` endpoint.
- **Deployment**: `Dockerfile` for building a minimal Debian-based image with all the application binaries.

## Getting Started

### Prerequisites

- Rust toolchain (latest stable)
- Docker (for containerized deployment)

### 1. Configuration

1.  Copy `latency-x-core/Config.toml.example` to `latency-x-core/Config.toml`.
2.  Edit `latency-x-core/Config.toml` to set up your strategies and API keys.
3.  For secrets like API keys, you can set them as environment variables. The application will load any value prefixed with `$` from the environment.

    For example, in `Config.toml`:
    ```toml
    api_key = "$BINANCE_API_KEY"
    ```
    The application will look for the `BINANCE_API_KEY` environment variable.

### 2. Building and Running

You can build and run the individual binaries or use the provided Dockerfile.

#### Build
```bash
cd latency-x-core
cargo build --release
```

This will produce three binaries in `target/release/`:
- `latency-x-core`: The main trading application.
- `pump-trader`: A specialized trader for the pump.fun platform.
- `backtester`: The backtesting engine.

#### Running the Core Application
The main application can run different strategies based on a command-line argument.

```bash
# Run the MarketMaker strategy
./target/release/latency-x-core market_maker

# Run the Arbitrage strategy
./target/release/latency-x-core arbitrage
```

#### Running the Pump.fun Trader
```bash
./target/release/pump-trader
```

#### Running the Backtester
The backtester uses historical tick data from `latency-x-core/data/ticks.csv` to simulate a strategy.

```bash
./target/release/backtester
```

### 3. Running Tests

Run all tests, including integration tests (which may require network access and are ignored by default):
```bash
cd latency-x-core
cargo test -- --include-ignored
```

Run only unit tests:
```bash
cd latency-x-core
cargo test
```

## Docker Deployment

A multi-stage `Dockerfile` is provided to build a small, optimized container image.

1.  **Build the image:**
    ```bash
    docker build -t latency-x .
    ```

2.  **Run the container:**
    You need to provide the configuration file and any necessary environment variables.
    ```bash
    docker run -it --rm \
      -v $(pwd)/latency-x-core/Config.toml:/app/Config.toml \
      -e BINANCE_API_KEY="your_key" \
      -e BINANCE_API_SECRET="your_secret" \
      latency-x market_maker
    ```
    This command runs the `market_maker` strategy inside the container. 
