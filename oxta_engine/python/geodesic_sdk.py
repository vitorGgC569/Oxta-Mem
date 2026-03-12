import pickle
import numpy as np
import io

try:
    import torch
    HAS_TORCH = True
except ImportError:
    HAS_TORCH = False

class GeodesicClient:
    """
    High-level Python SDK for Geodesic Memory Engine.
    Handles serialization of complex objects (NumPy, PyTorch) automatically.
    """
    def __init__(self, driver="redis", **kwargs):
        """
        driver: 'redis' (connect via network) or 'native' (direct rust binding)
        kwargs: arguments for the driver (host, port, db_path, etc)
        """
        self.driver_type = driver

        if driver == "redis":
            import redis
            host = kwargs.get("host", "localhost")
            port = kwargs.get("port", 6379)
            self.conn = redis.Redis(host=host, port=port, decode_responses=False) # Binary mode
        elif driver == "native":
            # Assumes geodesic_engine is built and installed in python path
            from geodesic_engine import PyGeodesicEngine
            path = kwargs.get("db_path", "geodesic.db")
            size = kwargs.get("size_mb", 100)
            self.conn = PyGeodesicEngine(path, size)
        else:
            raise ValueError("Unknown driver. Use 'redis' or 'native'")

    def _serialize(self, data):
        """Converts Tensors/Arrays/Objects to bytes."""
        if HAS_TORCH and isinstance(data, torch.Tensor):
            buff = io.BytesIO()
            torch.save(data, buff)
            return b"TORCH:" + buff.getvalue()

        if isinstance(data, np.ndarray):
            buff = io.BytesIO()
            np.save(buff, data)
            return b"NUMPY:" + buff.getvalue()

        # Fallback to Pickle for generic objects
        return b"PICKL:" + pickle.dumps(data)

    def _deserialize(self, data: bytes):
        """Restores bytes to objects."""
        if data.startswith(b"TORCH:"):
            buff = io.BytesIO(data[6:])
            return torch.load(buff)

        if data.startswith(b"NUMPY:"):
            buff = io.BytesIO(data[6:])
            return np.load(buff)

        if data.startswith(b"PICKL:"):
            return pickle.loads(data[6:])

        return data # Return as raw bytes if unknown prefix

    def save(self, key: str, value):
        """Saves any object (Tensor, Array, Dict) to the timeline."""
        payload = self._serialize(value)

        if self.driver_type == "redis":
            return self.conn.set(key, payload)
        else:
            return self.conn.write(key, payload)

    def load_latest(self, key: str):
        """Retrieves the latest state of the object."""
        if self.driver_type == "redis":
            data = self.conn.get(key)
        else:
            data = self.conn.read_latest(key)

        if data is None: return None
        return self._deserialize(data)

    def recall_history(self, key: str, depth: int):
        """Returns a list of states (time travel)."""
        # Note: Redis protocol implementation of 'RECALL' needed in server.rs if using redis driver.
        # For now, native driver supports it directly.
        if self.driver_type == "redis":
            # TODO: Implement RECALL command in server.rs and redis client extension
            raise NotImplementedError("History recall via Redis protocol not yet implemented in SDK")
        else:
            raw_list = self.conn.recall(key, depth)
            return [self._deserialize(x) for x in raw_list]
