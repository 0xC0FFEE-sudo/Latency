
## Project Details

**Name:** Qubic-Accelerated DeFi HFT Trading Bot (Student Edition)  
**Objective:** Build an open-source, plug-and-play HFT trading bot that achieves low-microsecond order execution and sub-second on-chain settlement via Qubic, running on affordable hardware and hosted RPC endpoints.

**Scope:**
- CEX & DEX trading on Binance, Kraken Pro, Coinbase, plus memecoin venues (Bullx, pump.fun, Axiom, Photon)  
- Ultra-low-latency market-making, arbitrage, and MEV strategies  
- Kernel-bypass networking (DPDK/XDP) instead of FPGA/ASIC  
- Qubic state-channel settlement and messaging  
- Real-time risk management, logging, and compliance

**Success Metrics:**
- End-to-end order path latency <5 μs  
- On-chain settlement <1 s via Qubic RPC  
- Daily volume capability ≥ $10 M (proof-of-concept)  
- Open-source code that runs on a student’s local or cloud VM

**Deliverables:**
1. Source code repository with setup scripts  
2. Documentation and sample configs for local/server/cloud deployment  
3. Benchmark reports of latency and throughput  
4. Tutorials for extending to new exchanges or strategies
