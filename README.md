# SysPerf: Supercomputing Benchmark Framework

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

SysPerf is an evolving supercomputing benchmark framework designed to provide comprehensive performance analysis for HPC environments. Currently supporting storage performance testing via FIO, with an ambitious roadmap for complete cluster performance evaluation capabilities.

## Incoming Features

### Storage System Analysis
- **FIO Integration**
  - Block device performance testing
  - Configurable I/O patterns
  - Multi-threaded I/O testing
  - IOPS and throughput measurements
  - Latency analysis
  - Queue depth impact assessment
  
- **Results Processing**
  - Real-time metric collection
  - Embedded database storage
  - Basic visualization capabilities
  - JSON/CSV export options

## Roadmap 

### Phase 1: Network Performance (Q2 2024)
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

### Phase 2: GPU Cluster Testing (Q3 2024)
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

### Phase 3: Advanced Storage (Q4 2024)
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

### Phase 4: System Integration (Q1 2025)
- [ ] Job Scheduler Integration
  - SLURM support
  - LSF compatibility
  - PBS integration
  - Resource allocation analysis

### Phase 5: Advanced Analytics (Q2 2025)
- [ ] Machine Learning Integration
  - Performance prediction
  - Anomaly detection
  - Resource optimization
  - Trend analysis

### Phase 6: Quantum & Future Tech (Q3 2025)
- [ ] Quantum Computing Benchmarks
  - Gate-based system testing
  - Quantum simulator benchmarks
  - Hybrid algorithm performance
  - Error rate analysis

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
./target/release/sysperf-svr benchmark --tool fio --test-type random-read

# Run comprehensive storage benchmark
./target/release/sysperf-svr storage-suite

# View results
./target/release/sysperf-svr results --latest
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

The project follows a Ports and Adapters (Hexagonal) architecture pattern:

```
sysperf-svr/
├── src/
│   ├── ports/                  # Interface definitions
│   │   ├── application_port.rs # Core application interfaces
│   │   ├── database_port.rs    # Database abstraction
│   │   ├── discovery_port.rs   # Service discovery interfaces
│   │   ├── log_port.rs        # Logging interfaces
│   │   ├── node_metrics_port.rs # System metrics interfaces
│   │   └── maas_login_port.rs  # Authentication interfaces
│   │
│   ├── adapters/              # Concrete implementations
│   │   ├── application_adapter.rs # Core application logic
│   │   ├── log_adapter.rs     # Logging implementation
│   │   ├── maas_discovery_adapter.rs # Service discovery
│   │   ├── maas_login_adapter.rs # Authentication
│   │   ├── node_metrics_adapter.rs # System metrics
│   │   └── surrealdb_adapter.rs # Database implementation
│   │
│   ├── cli/                   # Command line interface
│   └── main.rs                # Application entry point
│
├── config/                    # Configuration files
└── tests/                    # Integration tests
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

### Development Focus Areas
1. Network testing implementation
2. GPU benchmark integration
3. Visualization improvements
4. Documentation
5. Test coverage

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The HPC community
- The Rust community

## Citation

```bibtex
@software{sysperf2024,
  author = {Kenny (Knight) Sheridan},
  title = {SysPerf: Supercomputing Benchmark Framework},
  year = {2024},
  publisher = {GitHub},
  url = {https://github.com/kenetthdsheridan/sysperf-svr}
}
```

## Project Status

- **Stable**: FIO-based storage testing
- **Alpha**: Basic metric collection and visualization
- **Planning**: All other features


