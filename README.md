# 🌌 Oxta-Mem (Geodesic Memory)
**Industrial-Grade 4D Causal Memory for AI**

[![PyPI version](https://badge.fury.io/py/oxta-mem.svg)](https://pypi.org/project/oxta-mem/)
![Status](https://img.shields.io/badge/Status-Stable-green)
![Language](https://img.shields.io/badge/Rust-Inside-orange)
![Protocol](https://img.shields.io/badge/Protocol-RESP-red)

Oxta-Mem is a high-performance, persistent memory graph designed to give AI agents **"Time-Travel"** capabilities. Unlike Vector Databases that search by *similarity*, Oxta-Mem organizes information by **causality**, **time**, and **depth (4D)**.

Built with a high-performance Rust engine and exposed via an intuitive Python SDK.

---

## 🚀 Key Features

*   **Time Travel:** Retrieve the exact causal history of any variable or agent state.
*   **4D Causal Graph:** Nodes are linked in a Merkle-DAG structure, ensuring data integrity and lineage.
*   **Zero-Copy Persistence:** Uses `mmap` and `rkyv` for instant startup and RAM-speed access backed by disk.
*   **Vector Integration:** Hybrid search combining Causal retrieval with `usearch` vector similarity.
*   **Universal Compatibility:** Native Python SDK + Redis (RESP) protocol support.
*   **LangChain Ready:** Integrated retrievers for seamless use with LLM frameworks.

---

## ⚡ Performance (Verified)

*   **Native Write Throughput:** ~260,000+ ops/sec (1KB payloads)
*   **Native Read Latency:** ~1.4 - 3.0 µs (Sub-microsecond raw access)
*   **Python/Neural Throughput:** ~50,000 - 90,000 ops/sec
*   **Startup Time:** < 5ms (Zero-Copy/mmap)

---

## 🛠️ Installation

```bash
pip install oxta-mem
```

---

## 🐍 Quick Start (Python)

### 1. High-Level AI Memory (Neural Layer)
Perfect for AI agents tracking variables with historical context.

```python
import oxta_mem
import torch

# Initialize the 4D Memory Core
core = oxta_mem.GeodesicMemoryCore()

# Record a state (auto-links to previous states of 'agent_alpha')
state_vector = torch.randn(128)
core.write("agent_alpha", state_vector)

# Time Travel: Read the latest state
latest = core.read_latest("agent_alpha")
```

### 2. Industrial SDK (Flexible Persistence)
Use the `GeodesicClient` to manage complex data structures with Rust-speed.

```python
from oxta_mem import GeodesicClient

# Use 'native' driver for the embedded Rust engine
client = GeodesicClient(driver="native", db_path="geodesic.db", size_mb=500)

# Save any Python object (Dicts, Lists, NumPy)
client.save("system_config", {"mode": "causal", "version": 1.0})

# Recall causal history (last 5 steps)
history = client.recall_history("system_config", depth=5)
```

### 3. Native Rust Engine (Maximum Performance)
Direct access to the underlying Merkle-DAG engine.

```python
import oxta_mem

engine = oxta_mem.PyGeodesicEngine("memory.db", 1024) # 1GB arena
engine.write("key_0", b"binary_payload")
data = engine.read_latest("key_0")
```

---

## 🔗 LangChain Integration

Supercharge your RAG with causal context instead of just similarity snippets.

```python
from oxta_mem import GeodesicClient, GeodesicCausalRetriever

client = GeodesicClient(driver="native")
retriever = GeodesicCausalRetriever(client=client, depth=10)

# The LLM now receives the causal timeline of 'Market_Trend'
docs = retriever.get_relevant_documents("Market_Trend")
```

---

## 🏗️ Architecture

Oxta-Mem operates on a **Geodesic Mapping** principle:
1.  **Arena Allocation:** Pre-allocated memory files for zero fragmentation.
2.  **Merkle-Linage:** Every write generates a DAG edge to the previous state.
3.  **Hybrid Indexing:** Causal traversal + HNSW Vector Search.

---

## 📚 Documentation & Roadmap
*   [Scientific Proof (Big-O Analysis)](docs/SCIENTIFIC_PROOF.md)
*   [Engineering Roadmap](ROADMAP.md)
*   [TLA+ Verification](docs/tla/GeodesicMemory.tla)

---

## 📄 License
MIT License. Built for the era of Autonomus Agents.
