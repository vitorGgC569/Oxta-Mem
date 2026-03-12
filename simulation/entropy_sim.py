import random
import time
import statistics

# ==============================================================================
# SIMULATION: 10 Years of Entropy
# ==============================================================================

class Node:
    def __init__(self, val, prev_addr, timestamp):
        self.val = val
        self.prev = prev_addr
        self.timestamp = timestamp

class SimulationEngine:
    def __init__(self):
        self.store = [] # Arena Allocation simulation (List = Contiguous RAM)
        self.heads = {}

    def write(self, token_id, value):
        prev_ptr = self.heads.get(token_id, -1)
        new_addr = len(self.store)

        node = Node(value, prev_ptr, new_addr)
        self.store.append(node)
        self.heads[token_id] = new_addr
        return new_addr

    def traverse_back(self, token_id, steps):
        """Simulates walking back 'steps' into the past."""
        ptr = self.heads.get(token_id, -1)
        jumps = 0

        start_time = time.perf_counter_ns()

        while ptr != -1 and jumps < steps:
            node = self.store[ptr]
            ptr = node.prev
            jumps += 1

        end_time = time.perf_counter_ns()
        return (end_time - start_time) / 1000.0 # Microseconds

def run_simulation():
    print("--- STARTING ENTROPY SIMULATION: 10 VIRTUAL YEARS ---")

    sim = SimulationEngine()

    # Parameters
    # 10 Years * 365 Days * 24 Hours * 60 Minutes (approx 5M writes)
    TOTAL_WRITES = 1_000_000 # Reduced to 1M for quick CI execution
    VARIABLES = ["Var_A", "Var_B", "Var_C", "Var_D", "Var_E"]

    print(f"Simulating {TOTAL_WRITES} writes across {len(VARIABLES)} variables...")

    # 1. Fill Memory (Fragmented writes)
    start_gen = time.time()
    for i in range(TOTAL_WRITES):
        var = random.choice(VARIABLES)
        val = random.random()
        sim.write(var, val)

        if i % 200_000 == 0:
            print(f"  Generated {i} records...")

    end_gen = time.time()
    print(f"Generation Complete. Time: {end_gen - start_gen:.2f}s")
    print(f"Total Nodes: {len(sim.store)}")

    # 2. Measure Latency for Deep Recall
    print("\n--- MEASURING LATENCY (Deep Causal Traversal) ---")
    depths = [10, 100, 1000, 10000, 100000]

    results = []

    for depth in depths:
        latencies = []
        # Run 100 trials
        for _ in range(100):
            var = random.choice(VARIABLES)
            lat = sim.traverse_back(var, depth)
            latencies.append(lat)

        avg_lat = statistics.mean(latencies)
        p99_lat = sorted(latencies)[int(len(latencies)*0.99)]

        print(f"Depth: {depth:6d} steps | Avg Latency: {avg_lat:8.3f} us | P99: {p99_lat:8.3f} us")
        results.append((depth, avg_lat))

    # 3. Save Results
    with open("docs/SIMULATION_RESULTS.txt", "w") as f:
        f.write("DEPTH,AVG_LATENCY_US\n")
        for d, l in results:
            f.write(f"{d},{l}\n")

    print("\nSimulation Complete. Results saved to docs/SIMULATION_RESULTS.txt")

if __name__ == "__main__":
    run_simulation()
