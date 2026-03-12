use rkyv::{Archive, Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use memmap2::MmapMut;
use std::path::Path;
use usearch::{Index, IndexOptions, MetricKind, ScalarKind};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
// rkyv 0.8 compatibility
pub struct Node {
    pub value: Vec<u8>,
    pub prev: Option<u64>, // Offset in the file
    pub timestamp: u64,
}

pub struct GeodesicEngine {
    pub heads: HashMap<String, u64>, // Maps Variable ID -> Offset in File
    #[allow(dead_code)] // File needs to be kept alive for mmap
    file: File,
    mmap: MmapMut,
    current_offset: u64,
    // Hybrid Search: In-Memory Vector Index
    // In a real system, this would be persisted to disk too.
    vector_index: Option<Index>,
}

impl GeodesicEngine {
    pub fn new<P: AsRef<Path>>(path: P, size_mb: u64) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false) // Do not truncate existing files
            .open(&path)?;

        let size_bytes = size_mb * 1024 * 1024;
        file.set_len(size_bytes)?;

        let mmap = unsafe { MmapMut::map_mut(&file)? };

        // Initialize Vector Index (Simple configuration)
        let options = IndexOptions {
            dimensions: 128, // Default dimension, should be configurable
            metric: MetricKind::Cos,
            quantization: ScalarKind::F32,
            connectivity: 16,
            expansion_add: 128,
            expansion_search: 64,
            multi: false,
        };

        let index = Index::new(&options).ok(); // Ignore init failure for now

        Ok(Self {
            heads: HashMap::new(),
            file,
            mmap,
            current_offset: 0,
            vector_index: index,
        })
    }

    pub fn write(&mut self, token_id: &str, value: Vec<u8>) -> Result<u64, String> {
        // Hybrid Search Check: If value looks like a vector (float32 bytes), index it.
        // For prototype, we don't parse the bytes here, we assume separate method or manual handling.
        // But let's assume if the user provides a "vector" argument (API change needed), we index it.
        // For this function, we stick to the core log.

        self.write_internal(token_id, value)
    }

    // New method for Hybrid Write
    pub fn write_with_vector(&mut self, token_id: &str, value: Vec<u8>, vector: Vec<f32>) -> Result<u64, String> {
        let addr = self.write_internal(token_id, value)?;

        if let Some(idx) = &mut self.vector_index {
            // Use the address as the Key in the vector index
            // Note: usearch keys are u64, perfect for our address/offset
            idx.add(addr, &vector).map_err(|e| format!("Vector index error: {}", e))?;
        }

        Ok(addr)
    }

    fn write_internal(&mut self, token_id: &str, value: Vec<u8>) -> Result<u64, String> {
        let prev_ptr = self.heads.get(token_id).copied();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let node = Node {
            value,
            prev: prev_ptr,
            timestamp,
        };

        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&node)
            .map_err(|e| e.to_string())?;

        let len = bytes.len() as u64;

        if self.current_offset + len + 8 > self.mmap.len() as u64 {
             return Err("Arena Full".to_string());
        }

        let len_bytes = len.to_le_bytes();
        self.mmap[self.current_offset as usize..self.current_offset as usize + 8].copy_from_slice(&len_bytes);

        let data_start = self.current_offset + 8;
        self.mmap[data_start as usize..data_start as usize + len as usize].copy_from_slice(&bytes);

        let node_addr = self.current_offset;
        self.current_offset += 8 + len;

        self.heads.insert(token_id.to_string(), node_addr);

        Ok(node_addr)
    }

    pub fn read_latest(&self, token_id: &str) -> Option<Node> {
        let addr = self.heads.get(token_id)?;
        self.read_node_at(*addr)
    }

    pub fn read_node_at(&self, addr: u64) -> Option<Node> {
        if addr >= self.current_offset { return None; }

        let len_bytes = &self.mmap[addr as usize..addr as usize + 8];
        let len = u64::from_le_bytes(len_bytes.try_into().unwrap());

        let data_start = addr + 8;
        let data_end = data_start + len;

        let bytes = &self.mmap[data_start as usize..data_end as usize];

        rkyv::from_bytes::<Node, rkyv::rancor::Error>(bytes).ok()
    }

    pub fn recall(&self, token_id: &str, depth: usize) -> Vec<Node> {
        let mut result = Vec::new();
        let mut curr_addr = self.heads.get(token_id).copied();

        for _ in 0..depth {
            if let Some(addr) = curr_addr {
                if let Some(node) = self.read_node_at(addr) {
                    curr_addr = node.prev;
                    result.push(node);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        result
    }

    pub fn search_similar(&self, vector: Vec<f32>, k: usize) -> Vec<Node> {
        let mut results = Vec::new();

        if let Some(idx) = &self.vector_index {
            // USearch `search` returns `Result<Matches, Error>`.
            // `Matches` stores keys and distances in internal vectors.
            if let Ok(matches) = idx.search(&vector, k) {
                // We need to iterate over the keys. Matches struct exposes .keys field.
                for key in matches.keys {
                    // key is the address in our store (u64)
                    if let Some(node) = self.read_node_at(key) {
                        results.push(node);
                    }
                }
            }
        }

        results
    }
}
