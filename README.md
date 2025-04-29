# SysPerf: A Supercomputing Benchmark Framework
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

SysPerf is a passion project for benchmarking designed to provide comprehensive performance analysis for HPC environments. The current implementation focuses on storage performance testing via FIO, with a clear path toward complete cluster performance evaluation capabilities.

## Current Features

### Storage System Analysis
- **FIO Integration**
  - Block device performance testing
  - Configurable I/O patterns
  - Multi-threaded I/O testing
  - IOPS and throughput measurements
  - Latency analysis
  - Queue depth impact assessment

### On Deck Features
- **Results Processing**
  - Real-time metric collection
  - Embedded database storage
  - Basic visualization capabilities
  - JSON/CSV export options

## Quick Start

### Prerequisites
- Rust 1.70 or higher
- FIO 3.x or higher
- Linux kernel 4.x or higher

### Installation

```bash
# Clone the repository
git clone https://github.com/kennethdsheridan/sysperf-svr
cd sysperf-svr

# Build the project
cargo build --release

# Run tests
cargo test
```

### Basic Usage

```bash
# Run basic FIO test
./target/release/sysperf-svr benchmark --tool fio &
```

## Configuration

Example configuration for storage testing:

```yaml
# config.yaml
storage_benchmark:
  fio:
    block_sizes: 
      - "4k"
      - "8k"
      - "64k"
      - "1m"
    io_engine: "libaio"
    runtime: 60
    iodepth: [1, 8, 16, 32, 64]
    numjobs: [1, 4, 8]
    test_types:
      - "random-read"
      - "random-write"
      - "sequential-read"
      - "sequential-write"

database:
  embedded:
    path: "./data"
    retention_days: 30
  remote:
    enabled: false
    type: "influxdb"
    url: "http://metrics-db:8086"
```

## Architecture

SysPerf follows a **Hexagonal (Ports & Adapters)** design. At a glance, the project is organised like this:
```bash

sysperf-svr/
├── src/
│   ├── adapters/                 # ⬅️ OUTBOUND ADAPTERS (secondary)
│   │   ├── benchmark_adapter.rs
│   │   ├── database_adapter.rs
│   │   ├── log_adapter.rs
│   │   ├── metrics_adapter.rs
│   │   └── mod.rs
│   │
│   ├── application/              # ⬅️ APPLICATION SERVICES (use-cases)
│   │   ├── metrics/              #  ├─ orchestrates metric-collection use-cases
│   │   ├── storage/              #  ├─ orchestrates storage-benchmark flows
│   │   └── mod.rs
│   │
│   ├── cli/                      # ⬅️ PRIMARY ADAPTER – command-line interface
│   │   └── …                     #     (struct-opt/clap handlers live here)
│   │
│   ├── database/                 # ⬅️ DB bootstrap & migration helpers
│   │   └── …                     
│   │
│   ├── domain/                   # ⬅️ ENTERPRISE DOMAIN (pure business rules)
│   │   └── …                     
│   │
│   ├── ports/                    # ⬅️ PORTS (traits) – the “hexagon edges”
│   │   ├── benchmark_port.rs
│   │   ├── database_port.rs
│   │   ├── log_port.rs
│   │   ├── metrics_port.rs
│   │   ├── storage_port.rs
│   │   └── mod.rs
│   │
│   ├── wasm/                     # ⬅️ PRIMARY ADAPTER – WebAssembly front-end
│   │   └── …                     
│   │
│   ├── lib.rs                    # Library entry (re-exports of core crates)
│   └── main.rs                   # Binary entry (bridges CLI ↔ application layer)
│
├── config/                       # YAML & TOML examples, baseline configs
└── tests/                        # Black-box & integration tests
```

### Architectural Approach

The project uses a Ports and Adapters (Hexagonal) architecture that provides:

- **Ports Layer**: Trait-based interfaces that abstract core functionality
  - Benchmarking ports
  - Metric collection ports
  - Database interaction ports
  - Node communication ports
  - Hardware management ports

- **Adapters Layer**: Concrete implementations that are hot-swappable
  - FIO benchmarking adapters
  - Hardware metric collectors
  - SurrealDB persistence adapter
  - MAAS integration adapters
  - Logging implementations

### Key Benefits

- Component independence and hot-swappability
- Simplified testing through trait mocking
- High availability through adapter redundancy
- Easy extension of functionality
- Clear separation of concerns

### Current Implementation Status

- **Stable**
  - Storage benchmarking ports and adapters (FIO)
  - Basic metric collection
  - Database integration
  
- **In Development**
  - Network testing ports
  - GPU benchmarking interfaces
  - Additional storage adapters

## Roadmap

### Network Performance 
- [ ] InfiniBand Testing Suite
  - RDMA performance analysis
  - Subnet Manager performance
  - QoS validation
  - Network congestion testing
  
- [ ] Network Tools Integration
  - iperf3 TCP/UDP testing
  - perftest RDMA metrics
  - OpenMPI benchmark suite
  - Custom network test framework

### GPU Cluster Testing 
- [ ] Containerized GPU Benchmarks
  - HPL-AI implementation
  - NVIDIA NCCL tests
  - GPU-Direct RDMA
  - Multi-node GPU communication
  
- [ ] ML/AI Performance Suite
  - Distributed training metrics
  - Model inference benchmarks
  - Memory bandwidth testing
  - Scaling efficiency analysis

### Advanced Storage
- [ ] Parallel Filesystem Testing
  - Lustre performance suite
  - GPFS throughput analysis
  - BeeGFS benchmarking
  - Metadata performance testing
  
- [ ] Extended I/O Testing
  - IOR integration
  - mdtest implementation
  - Custom I/O patterns
  - Multi-client testing

### System Integration 
- [ ] Job Scheduler Integration
  - SLURM support
  - LSF compatibility
  - PBS integration
  - Resource allocation analysis

### Advanced Analytics 
- [ ] Machine Learning Integration
  - Performance prediction
  - Anomaly detection
  - Resource optimization
  - Trend analysis

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The HPC community
- The Rust community

## Citation

```bibtex
@software{sysperf2024,
  author = {Kenny (Knight) Sheridan},
  title = {SysPerf: A Supercomputing Benchmark Framework},
  year = {2024},
  publisher = {GitHub},
  url = {https://github.com/kennethdsheridan/sysperf-svr}
}
```
