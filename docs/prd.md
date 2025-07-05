# prd.md

## Product Requirements Document

### 1. User Personas
- **Quant Trader:** customizes strategy parameters and monitors latency/PNL.  
- **Developer:** runs and extends the open-source bot on local or cloud VMs.  
- **Compliance Auditor:** reviews audit logs and on-chain settlements.

### 2. Key Use Cases
- **UC1:** Launch market-making on Binance and Uniswap v3 with 0.03% spread.  
- **UC2:** Execute cross-exchange arbitrage between Kraken Pro and Binance.  
- **UC3:** Capture MEV opportunities on memecoin pools (pump.fun, Photon).  
- **UC4:** Settle trades via Qubic state channels with <1 s finality.

### 3. Performance & Constraints
- **Latency:** ≤5 μs order-entry to exchange receipt  
- **Reliability:** ≥99.9% service uptime (single-node)  
- **Hardware:** student-affordable (≤ $2 000) or cloud VM credits  
- **Security:** keys in local encrypted file or Vault; TLS for RPC

### 4. Milestones
| Phase           | Deliverable                                      | Timeline  |
|-----------------|--------------------------------------------------|-----------|
| Phase 1 (Dev)   | Prototype in Rust/C++ with DPDK market data feed | Month 1   |
| Phase 2 (Test)  | Add execution gateway & basic strategy           | Month 2   |
| Phase 3 (Qubic) | Integrate Qubic RPC for settlement               | Month 3   |
| Phase 4 (Scale) | Add memecoin adapters & MEV module               | Month 4   |
| Phase 5 (Demo)  | Publish benchmarks & open-source release         | Month 5   |
