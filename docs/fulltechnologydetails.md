# fulltechnologydetails.md

## Detailed Technology Overview

### A. Hardware & Cloud Setup
- **Local Dev:** MacBook Pro (ARM or Intel) with Thunderbolt→10GbE NIC for initial coding/backtesting  
- **Affordable Server:** Mid-range desktop (e.g., Ryzen 7/i7, 32 GB RAM, NVMe SSD, Intel X540 10GbE NIC) (~$1 000 USD)  
- **Cloud Option:** AWS bare-metal or Oracle Cloud VM (Nitro) with 10–25 Gbps NIC, placement group, single AZ  
- **CPU Tuning:** `isolcpus`, `nohz_full`, fixed CPU frequency, disable Hyper-Threading

### B. Networking & OS
- **Kernel-Bypass:** DPDK or AF_XDP/XDP for market data and order submission  
- **NIC Tuning:** disable interrupt coalescing, set IRQ affinity, use hugepages  
- **Real-Time Kernel:** optional PREEMPT_RT patch to reduce jitter  
- **Time Sync:** PTP via software (e.g., Linuxptp); hardware sync not required

### C. Software Stack
- **Languages:**  
  - **Rust** for core engine and strategy modules (memory-safe, zero-cost abstractions)  
  - **C++17** for ultra-tuned order gateway (optional)  
  - **Go/Python** for auxiliary services (logging, monitoring)
- **Network Libraries:** DPDK, liburing (io_uring), socket2 (Rust), Boost.Asio (C++)  
- **Exchange Adapters:** OpenLimits (Rust), direct REST/WebSocket clients  
- **Qubic SDKs:** QubiPy (Python), `qubic-rs` (community Rust crate)
- **Inter-process Messaging:** lock-free queues (e.g., crossbeam, folly)  

### D. Data & Persistence
- **In-Memory:** Redis for hot state (positions, P&L)  
- **On-Disk:** PostgreSQL on NVMe for audit logs; optional SQLite for config  
- **Metrics:** Prometheus exporter, Grafana dashboards  

### E. Deployment & Orchestration
- **Containerization:** *Not* recommended for core; use raw VMs/Bare-metal  
- **Orchestration:** Systemd units or supervised processes for restart  
- **CI/CD:** *Omitted* (manual build & deploy scripts)

### F. Security & Compliance
- **Secrets:** local encrypted files, HashiCorp Vault (optional)  
- **Transport:** TLS/mTLS for all RPC and exchange endpoints  
- **Key Storage:** OS keyring or lightweight HSM emulator  
- **Audit:** immutable logs anchored via Qubic transactions

---

> _This stack balances affordability and performance, enabling students to achieve low-microsecond trading latencies without enterprise hardware._  
