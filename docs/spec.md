# spec.md

## Functional Specifications

### 1. Market Data Ingestion
- **DPDK/XDP-based NIC polling** for WebSocket/TCP and raw UDP feeds from Binance, Kraken Pro, Uniswap v3, dYdX, Bullx, pump.fun, Axiom, Photon  
- **Message normalization** into common tick format  
- **Time stamping** via CPU TSC and NIC hardware timestamps (where available)

### 2. Strategy Engine
- **Market-Making**: configurable spread, inventory skew, dynamic quoting on CEX & DEX  
- **Arbitrage**: cross-exchange (e.g., Binance ↔ Kraken) and cross-chain via Qubic messaging  
- **MEV**: sandwich/back-run bots leveraging mempool sniffer and on-chain data  
- **Plugin architecture** for new strategies in Rust or C++

### 3. Order Execution Gateway
- **Kernel-bypass** for order submission using DPDK or AF_XDP sockets  
- **Dedicated execution threads** pinned to isolated CPU cores  
- **Exchange adapters** in Rust/C++ calling REST or raw WebSocket APIs with minimal overhead  
- **Rate-limiting and retry logic** per exchange

### 4. Qubic Integration
- **QubiPy (Python) or Rust SDK** for sending settlement messages to Nitro state channels  
- **Asynchronous channel updates**: batch signed messages to Qubic RPC endpoint  
- **On-chain dispute fallback** via Solidity smart contracts  

### 5. Risk, Logging & Compliance
- **In-memory P&L and position ledger** written to Redis for real-time risk checks  
- **Kill-switch** to halt trading on threshold breaches  
- **Audit logs** asynchronously persisted to NVMe PostgreSQL  
- **Basic KYC/AML hooks** (logging only; no full KYC integration)
