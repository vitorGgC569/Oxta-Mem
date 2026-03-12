pub mod engine;
pub mod server;
pub mod sharding;

use pyo3::prelude::*;
use crate::engine::GeodesicEngine;

#[pymodule]
fn geodesic_engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyGeodesicEngine>()?;
    Ok(())
}

#[pyclass]
struct PyGeodesicEngine {
    inner: GeodesicEngine,
}

#[pymethods]
impl PyGeodesicEngine {
    #[new]
    fn new(path: String, size_mb: u64) -> PyResult<Self> {
        let engine = GeodesicEngine::new(path, size_mb).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        Ok(PyGeodesicEngine { inner: engine })
    }

    fn write(&mut self, token_id: String, value: Vec<u8>) -> PyResult<u64> {
        self.inner.write(&token_id, value).map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)
    }

    // New Binding for Hybrid Search
    fn write_with_vector(&mut self, token_id: String, value: Vec<u8>, vector: Vec<f32>) -> PyResult<u64> {
         self.inner.write_with_vector(&token_id, value, vector).map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)
    }

    fn read_latest(&self, token_id: String) -> PyResult<Option<Vec<u8>>> {
        if let Some(node) = self.inner.read_latest(&token_id) {
             Ok(Some(node.value))
        } else {
             Ok(None)
        }
    }

    fn recall(&self, token_id: String, depth: usize) -> PyResult<Vec<Vec<u8>>> {
        let nodes = self.inner.recall(&token_id, depth);
        Ok(nodes.into_iter().map(|n| n.value).collect())
    }

    // New Binding for Similarity Search
    fn search_similar(&self, vector: Vec<f32>, k: usize) -> PyResult<Vec<Vec<u8>>> {
        let nodes = self.inner.search_similar(vector, k);
        Ok(nodes.into_iter().map(|n| n.value).collect())
    }
}
