# SysPerf: A Supercomputing Benchmark Framework
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

SysPerf is a supercomputing benchmark framework designed to provide comprehensive performance analysis for HPC environments. Currently supporting storage performance testing via FIO, with a roadmap for complete cluster performance evaluation capabilities.

## On-deck Features

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

## Quick Start

### Prerequisites

- Rust 1.70+ (if building with Cargo)
- Linux system with kernel 4.x or higher
- `nix` (if building from flakes)

---

## Installation & Build Options

### Option 1: Build with Nix (Recommended)

SysPerf uses [Nix](https://nixos.org/) to produce statically linked binaries, `.deb` packages for Debian/Ubuntu, and `.tar.gz` archives.

#### 1. Install Nix

```bash
sh <(curl -L https://nixos.org/nix/install)
````

Enable flakes:

```bash
mkdir -p ~/.config/nix
echo 'experimental-features = nix-command flakes' >> ~/.config/nix/nix.conf
```

#### 2. Build the Static Binary

```bash
nix build . --extra-experimental-features 'nix-command flakes'
```

Output:

```
result/bin/sysperf-svr
```

#### 3. Build and Install a .deb Package (Debian/Ubuntu)

```bash
nix build .#deb --extra-experimental-features 'nix-command flakes'
cp result sysperf-svr_0.1.0_amd64.deb
sudo dpkg -i sysperf-svr_0.1.0_amd64.deb
```

Installs the binary to `/usr/bin/sysperf-svr`.

#### 4. Build a .tar.gz Archive (Portable)

```bash
nix build .#tarball --extra-experimental-features 'nix-command flakes'
```

Output:

```
result/sysperf-svr.tar.gz
```

#### 5. Optional: Development Shell

```bash
nix develop --extra-experimental-features 'nix-command flakes'
```

---

### Option 2: Build with Cargo (for development)

```bash
# Clone the repository
git clone https://github.com/kennethdsheridan/sysperf-svr
cd sysperf-svr

# Build
cargo build --release

# Run tests
cargo test
```

Output binary will be located at:

```
./target/release/sysperf-svr
```

---

## Basic Usage

```bash
# Run basic FIO test
sysperf-svr benchmark --tool fio --test-type random-read

# Run comprehensive storage benchmark
sysperf-svr storage-suite

# View results
sysperf-svr results --latest
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

---

## Architecture

The project follows a Ports and Adapters (Hexagonal) architecture:

```
sysperf-svr/
├── src/
│   ├── ports/
│   ├── adapters/
│   ├── cli/
│   └── main.rs
├── config/
└── tests/
```

### Architectural Approach

* **Ports Layer**: Trait-based interfaces for benchmarking, metrics, database, and system integration
* **Adapters Layer**: Modular, swappable implementations

Key benefits:

* Component independence
* Testability through interface abstraction
* Flexible extension through adapter plug-ins

---

## Current Status

**Stable:**

* Storage benchmarking (FIO)
* Basic metric collection
* Embedded database integration

**In Development:**

* Network and GPU benchmarking
* Job scheduler integration
* Result visualization

---

## License

This project is licensed under the MIT License – see the [LICENSE](LICENSE) file.

---

## Acknowledgments

* The HPC community
* The Rust community

---

## Citation

```bibtex
@software{sysperf2024,
  author = {Kenny (Knight) Sheridan},
  title = {SysPerf: Supercomputing Benchmark Framework},
  year = {2024},
  publisher = {GitHub},
  url = {https://github.com/kennethdsheridan/sysperf-svr}
}
```


