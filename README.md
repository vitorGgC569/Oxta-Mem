# Oxta Mem
**Industrial-Grade 4D Causal Memory for AI**

![Status](https://img.shields.io/badge/Status-Stable-green)
![Language](https://img.shields.io/badge/Rust-Inside-orange)
![Protocol](https://img.shields.io/badge/Protocol-RESP-red)

The Geodesic Memory Engine is a high-performance, persistent memory graph designed to give AI models "Time-Travel" capabilities without retraining. Unlike Vector Databases that search by *similarity*, Geodesic Engine searches by *causality* and *time*.

## 🚀 Key Features

*   **Time Travel:** Retrieve the exact state of any variable at any point in history.
*   **4D Graph Structure:** Nodes are linked causally (Merkle-DAG), not just stored as a list.
*   **Zero-Copy Persistence:** Uses `mmap` and `rkyv` for instant startup and RAM-speed access backed by disk.
*   **Universal Compatibility:** Speaks the **Redis (RESP)** protocol. Works with any Redis client.
*   **Python Bindings:** Plug-and-play integration with PyTorch, TensorFlow, and LangChain via `pyo3`.

## ⚡ Performance

Benchmarks running on standard hardware:

*   **Write Throughput:** > 300,000 writes/sec (Append-Only Log)
*   **Read Latency (P99):** ~3 microseconds
*   **Startup Time:** < 10ms (Zero-Copy)

> "The engine navigates millions of causal steps effectively instantly due to Arena Allocation and CPU Prefetching."

## 🛠️ Quick Start (Docker)

The easiest way to run the engine is via Docker.

### Prerequisites
*   Docker
*   Docker Compose (Optional)

### Running with Docker CLI

1.  **Build the image:**
    ```bash
    docker build -t geodesic-engine .
    ```

2.  **Run the container:**
    ```bash
    docker run -d -p 6379:6379 -v $(pwd)/data:/data --name geodesic geodesic-engine
    ```

3.  **Test connection (using redis-cli):**
    ```bash
    redis-cli -p 6379 PING
    # Output: PONG

    redis-cli -p 6379 SET user:1 "Alice"
    # Output: OK

    redis-cli -p 6379 GET user:1
    # Output: "Alice"
    ```

### Running with Docker Compose

1.  **Start the service:**
    ```bash
    docker-compose up -d
    ```

2.  **Stop the service:**
    ```bash
    docker-compose down
    ```

The database file will be persisted in the `./data` directory on your host machine.

---

## 🏗️ Manual Build (Rust)

If you prefer running "bare metal" or want to develop the core:

### Prerequisites
*   Rust (latest stable)
*   Cargo

### Installation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-repo/geodesic-engine.git
    cd geodesic-engine
    ```

2.  **Build and Run:**
    ```bash
    cd geodesic_engine
    cargo run --release -- --port 6379 --db-path ./my_memory.db --size-mb 1024
    ```

### Running Benchmarks
To reproduce the performance metrics:

```bash
cd geodesic_engine
cargo run --release --bin benchmark
```

---

## 🐍 Python Integration (AI/ML)

You can use the engine directly from Python.

### Option 1: Via Redis Client (Recommended for Production)
```python
import redis

# Connect to Geodesic Engine
r = redis.Redis(host='localhost', port=6379, decode_responses=True)

# Write Event
r.set("sensor_X", "High Pressure")

# Read Current State
print(r.get("sensor_X"))
```

### Option 2: Native Bindings (High Performance / Embedded)
To build the Python extension:
```bash
cd geodesic_engine
maturin develop --release
```

Usage:
```python
from geodesic_engine import PyGeodesicEngine

engine = PyGeodesicEngine("local_store.db", 100)
engine.write("var_a", b"some_data")
print(engine.read_latest("var_a"))
```

## 📚 Documentation
*   [Scientific Proof (Big-O Analysis)](docs/SCIENTIFIC_PROOF.md)
*   [Engineering Roadmap](ROADMAP.md)
*   [TLA+ Verification](docs/tla/GeodesicMemory.tla)
