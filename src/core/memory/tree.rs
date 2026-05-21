//! Memory Tree: 核心树结构实现

use crate::core::memory::{MemoryNode, MemoryStore};
use crate::error::Result;
use chrono::{DateTime, Duration, Utc};
use std::path::Path;
use std::sync::{Arc, RwLock};

/// Memory Tree
pub struct MemoryTree {
    store: Arc<RwLock<MemoryStore>>,
    cache: Arc<RwLock<std::collections::HashMap<String, MemoryNode>>>,
}

impl MemoryTree {
    /// 创建新的 Memory Tree
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let store = MemoryStore::new(path)?;
        Ok(MemoryTree {
            store: Arc::new(RwLock::new(store)),
            cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
        })
    }

    /// 插入新的记忆
    pub fn insert(&self, content: &str, source: &str) -> Result<String> {
        let node = MemoryNode::new(content.to_string(), source.to_string());
        let node_id = node.node_id.clone();
        let store = self.store.write().unwrap();
        store.insert(&node)?;
        let mut cache = self.cache.write().unwrap();
        cache.insert(node_id.clone(), node);
        Ok(node_id)
    }

    /// 获取单个记忆
    pub fn get(&self, node_id: &str) -> Result<MemoryNode> {
        let cache = self.cache.read().unwrap();
        if let Some(node) = cache.get(node_id) {
            let mut node_clone = node.clone();
            drop(cache);
            node_clone.record_access();
            return Ok(node_clone);
        }
        drop(cache);
        let store = self.store.read().unwrap();
        let node = store
            .get(node_id)?
            .ok_or(crate::error::NexusError::NodeNotFound(node_id.to_string()))?
        Ok(node)
    }

    /// 按来源查询
    pub fn query_source(&self, source: &str, limit: usize) -> Result<Vec<MemoryNode>> {
        let store = self.store.read().unwrap();
        store.query_by_source(source, limit)
    }

    /// 时间范围查询
    pub fn query_time_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<MemoryNode>> {
        let store = self.store.read().unwrap();
        store.query_by_time_range(start, end)
    }

    /// 生成上下文
    pub fn build_context(&self, task: &str, token_budget: usize) -> Result<String> {
        let mut context = String::new();
        context.push_str(&format!("Task: {}\n\n", task));
        Ok(context)
    }

    /// 生成摘要
    pub fn summarize_days(&self, days: u32, token_budget: usize) -> Result<String> {
        let now = Utc::now();
        let start = now - Duration::days(days as i64);
        let _nodes = self.query_time_range(start, now)?;
        Ok(String::new())
    }

    /// 获取统计
    pub fn get_stats(&self) -> Result<(u64, u64)> {
        let store = self.store.read().unwrap();
        store.get_stats()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup() -> (MemoryTree, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let tree = MemoryTree::new(&db_path).unwrap();
        (tree, temp_dir)
    }

    #[test]
    fn test_insert_get() {
        let (tree, _) = setup();
        let id = tree.insert("Test", "test").unwrap();
        let node = tree.get(&id).unwrap();
        assert_eq!(node.content, "Test");
    }

    #[test]
    fn test_query_source() {
        let (tree, _) = setup();
        tree.insert("Memory 1", "gmail").unwrap();
        tree.insert("Memory 2", "gmail").unwrap();
        let results = tree.query_source("gmail", 10).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_stats() {
        let (tree, _) = setup();
        tree.insert("Mem1", "test").unwrap();
        tree.insert("Mem2", "test").unwrap();
        let (count, _) = tree.get_stats().unwrap();
        assert_eq!(count, 2);
    }
}
