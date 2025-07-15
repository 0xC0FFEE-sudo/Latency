# Latency-X: High-Frequency Trading Bot

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

## 🚀 Overview

**Latency-X** is a high-performance, open-source High-Frequency Trading (HFT) system built in Rust. It features real-time market data processing, multiple trading strategies, cross-exchange arbitrage, and a professional web dashboard for monitoring and control.

### ✨ Key Features

- **🔥 Ultra-Low Latency**: Sub-millisecond order execution with optimized data structures
- **📊 Multiple Strategies**: Market Making, Cross-Exchange Arbitrage, MEV opportunities
- **🌐 Multi-Exchange Support**: Binance, Kraken, and extensible connector architecture
- **📈 Real-time Dashboard**: React-based web interface with live updates
- **💾 Data Persistence**: SQLite database for trades, orders, and historical data
- **⚡ High Performance**: Multi-threaded architecture with CPU core affinity
- **📝 Comprehensive Logging**: Structured logging with real-time WebSocket streaming
- **🎯 Risk Management**: Position tracking and automated risk controls

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Market Data   │    │   Trading Core   │    │   Dashboard     │
│   Connectors    │───▶│   & Strategies   │───▶│   & Monitoring  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
│                      │                      │
├─ Binance            ├─ Market Maker        ├─ React Frontend
├─ Kraken             ├─ Arbitrage           ├─ WebSocket Updates
├─ Mock Data          ├─ MEV Strategy        ├─ Trade History
└─ Extensible         └─ Risk Management     └─ Live Logs
```

## 🛠️ Technology Stack

- **Backend**: Rust with Tokio async runtime
- **Frontend**: React + TypeScript + Tailwind CSS
- **Database**: SQLite with SQLx
- **Real-time**: WebSocket communication
- **Monitoring**: Prometheus metrics, structured logging
- **Build**: Cargo for Rust, Vite for frontend

## 🚀 Quick Start

### Prerequisites

- **Rust** (1.70+): [Install Rust](https://rustup.rs/)
- **Node.js** (18+): [Install Node.js](https://nodejs.org/)
- **Git**: [Install Git](https://git-scm.com/)

### 1. Clone and Setup

```bash
git clone https://github.com/yourusername/latency-x.git
cd latency-x

# Setup configuration
cp latency-x-core/Config.toml.example latency-x-core/Config.toml
```

### 2. Configure API Keys

Edit `latency-x-core/Config.toml` and replace placeholder values:

```toml
[binance]
api_key = "YOUR_BINANCE_API_KEY"
api_secret = "YOUR_BINANCE_API_SECRET"

[kraken]
api_key = "YOUR_KRAKEN_API_KEY"
api_secret = "YOUR_KRAKEN_API_SECRET"
```

### 3. Build and Run

```bash
# Build the backend
cd latency-x-core
cargo build --release

# Build the frontend
cd ../latency-x-dashboard
npm install
npm run build

# Run the trading system
cd ../latency-x-core
./target/release/latency-x-core --strategy arbitrage
```

### 4. Access Dashboard

Open your browser and navigate to:
- **Dashboard**: http://localhost:3000
- **Metrics**: http://localhost:9000/metrics

## 📊 Trading Strategies

### Market Maker
Provides liquidity by placing buy and sell orders around the current market price.

```bash
./target/release/latency-x-core --strategy market-maker
```

### Cross-Exchange Arbitrage
Identifies price differences between exchanges and executes profitable trades.

```bash
./target/release/latency-x-core --strategy arbitrage
```

### MEV (Maximal Extractable Value)
Captures value from transaction ordering and blockchain state changes.

```bash
./target/release/latency-x-core --strategy mev
```

## 🖥️ Dashboard Features

- **📈 Live Trading Data**: Real-time trade execution and P&L
- **📊 Performance Metrics**: Latency measurements and success rates
- **📝 Live Logs**: Structured logging with filtering and search
- **⚙️ Settings**: Theme switching and configuration management
- **📋 Trade History**: Comprehensive trade and order history

## 🔧 Configuration

### Environment Variables

For production deployments, use environment variables for sensitive data:

```bash
export BINANCE_API_KEY="your_actual_key"
export BINANCE_API_SECRET="your_actual_secret"
export KRAKEN_API_KEY="your_actual_key"
export KRAKEN_API_SECRET="your_actual_secret"
```

### Database Configuration

```toml
database_url = "sqlite:latency_x.db"
```

### Strategy Parameters

```toml
[mev_strategy]
asset_a = "ETH"
asset_b = "BTC"
asset_c = "USDT"
trade_amount_b = 0.01
min_profit_threshold = 0.001
```

## 🧪 Testing

```bash
# Run all tests
cd latency-x-core
cargo test

# Run with integration tests
cargo test -- --include-ignored

# Frontend tests
cd latency-x-dashboard
npm test
```

## 📦 Docker Deployment

```bash
# Build Docker image
docker build -t latency-x .

# Run with environment variables
docker run -d \
  -p 3000:3000 \
  -p 9000:9000 \
  -e BINANCE_API_KEY="your_key" \
  -e BINANCE_API_SECRET="your_secret" \
  latency-x --strategy arbitrage
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This software is for educational and research purposes only. Trading cryptocurrencies and other financial instruments involves substantial risk and may not be suitable for all investors. Past performance is not indicative of future results. Use at your own risk.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for performance and safety
- UI components from [shadcn/ui](https://ui.shadcn.com/)
- Real-time charts with [Recharts](https://recharts.org/)
- WebSocket communication with [Tokio](https://tokio.rs/)

---

**⭐ Star this repository if you find it useful!** 
