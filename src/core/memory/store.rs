//! Memory Store: SQLite 存储层实现

use crate::core::memory::MemoryNode;
use crate::error::{NexusError, Result};
use chrono::{DateTime, Utc};
use rusqlite::{
    params, Connection, OptionalExtension, Row,
};
use std::path::Path;

/// SQLite 存储层
pub struct MemoryStore {
    conn: Connection,
}

impl MemoryStore {
    /// 创建新的存储实例
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)
            .map_err(|e| NexusError::DatabaseError(e))?;
        let store = MemoryStore { conn };
        store.init_schema()?;
        Ok(store)
    }

    /// 初始化数据库架构
    fn init_schema(&self) -> Result<()> {
        self.conn
            .execute_batch(
                r#"
CREATE TABLE IF NOT EXISTS memory_nodes (
    node_id TEXT PRIMARY KEY,
    parent_id TEXT,
    content TEXT NOT NULL,
    token_count INTEGER NOT NULL,
    score REAL NOT NULL,
    source_ref TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    metadata TEXT,
    is_important BOOLEAN DEFAULT 0,
    access_count INTEGER DEFAULT 0,
    last_accessed TEXT,
    FOREIGN KEY (parent_id) REFERENCES memory_nodes(node_id)
);
CREATE INDEX IF NOT EXISTS idx_parent ON memory_nodes(parent_id);
CREATE INDEX IF NOT EXISTS idx_source ON memory_nodes(source_ref);
CREATE INDEX IF NOT EXISTS idx_created ON memory_nodes(created_at);
CREATE INDEX IF NOT EXISTS idx_score ON memory_nodes(score);
"#,
            )
            .map_err(|e| NexusError::DatabaseError(e))?;
        Ok(())
    }

    /// 插入新节点
    pub fn insert(&self, node: &MemoryNode) -> Result<()> {
        let metadata_json = serde_json::to_string(&node.metadata)
            .map_err(|e| NexusError::SerializationError(e))?;

        self.conn
            .execute(
                r#"
INSERT INTO memory_nodes (
    node_id, parent_id, content, token_count, score, source_ref,
    created_at, updated_at, metadata, is_important, access_count, last_accessed
) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
"#,
                params![
                    &node.node_id,
                    &node.parent_id,
                    &node.content,
                    node.token_count as i64,
                    node.score,
                    &node.source_ref,
                    node.created_at.to_rfc3339(),
                    node.updated_at.to_rfc3339(),
                    &metadata_json,
                    node.is_important as i32,
                    node.access_count as i32,
                    node.last_accessed.map(|dt| dt.to_rfc3339()),
                ],
            )
            .map_err(|e| NexusError::DatabaseError(e))?;
        Ok(())
    }

    /// 获取单个节点
    pub fn get(&self, node_id: &str) -> Result<Option<MemoryNode>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT node_id, parent_id, content, token_count, score, source_ref, created_at, updated_at, metadata, is_important, access_count, last_accessed FROM memory_nodes WHERE node_id = ?1",
            )
            .map_err(|e| NexusError::DatabaseError(e))?;

        let node = stmt
            .query_row(params![node_id], |row| Self::parse_row(row))
            .optional()
            .map_err(|e| NexusError::DatabaseError(e))?

        Ok(node)
    }

    /// 按来源查询
    pub fn query_by_source(&self, source: &str, limit: usize) -> Result<Vec<MemoryNode>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT node_id, parent_id, content, token_count, score, source_ref, created_at, updated_at, metadata, is_important, access_count, last_accessed FROM memory_nodes WHERE source_ref = ?1 ORDER BY created_at DESC LIMIT ?2",
            )
            .map_err(|e| NexusError::DatabaseError(e))?

        let nodes: Vec<MemoryNode> = stmt
            .query_map(params![source, limit as i64], |row| Self::parse_row(row))
            .map_err(|e| NexusError::DatabaseError(e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| NexusError::DatabaseError(e))?

        Ok(nodes)
    }

    /// 按时间范围查询
    pub fn query_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<MemoryNode>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT node_id, parent_id, content, token_count, score, source_ref, created_at, updated_at, metadata, is_important, access_count, last_accessed FROM memory_nodes WHERE created_at BETWEEN ?1 AND ?2 ORDER BY created_at DESC",
            )
            .map_err(|e| NexusError::DatabaseError(e))?

        let nodes: Vec<MemoryNode> = stmt
            .query_map(params![start.to_rfc3339(), end.to_rfc3339()], |row| Self::parse_row(row))
            .map_err(|e| NexusError::DatabaseError(e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| NexusError::DatabaseError(e))?

        Ok(nodes)
    }

    /// 获取统计信息
    pub fn get_stats(&self) -> Result<(u64, u64)> {
        let total_nodes: u64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM memory_nodes", [], |row| row.get(0))
            .map_err(|e| NexusError::DatabaseError(e))?

        let total_tokens: u64 = self
            .conn
            .query_row("SELECT COALESCE(SUM(token_count), 0) FROM memory_nodes", [], |row| row.get(0))
            .map_err(|e| NexusError::DatabaseError(e))?

        Ok((total_nodes, total_tokens))
    }

    /// 解析行
    fn parse_row(row: &Row) -> rusqlite::Result<MemoryNode> {
        let metadata_json: String = row.get(8)?;
        let metadata = serde_json::from_str(&metadata_json).unwrap_or_default();
        let created_at_str: String = row.get(6)?;
        let updated_at_str: String = row.get(7)?;

        Ok(MemoryNode {
            node_id: row.get(0)?,
            parent_id: row.get(1)?,
            child_ids: Vec::new(),
            content: row.get(2)?,
            token_count: row.get::<_, i64>(3)? as usize,
            score: row.get(4)?,
            source_ref: row.get(5)?,
            created_at: created_at_str.parse().unwrap_or_else(|_| Utc::now()),
            updated_at: updated_at_str.parse().unwrap_or_else(|_| Utc::now()),
            metadata,
            is_important: row.get::<_, i32>(9)? != 0,
            access_count: row.get::<_, i32>(10)? as u32,
            last_accessed: row.get::<_, Option<String>>(11)?.and_then(|s| s.parse().ok()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_store_creation() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let store = MemoryStore::new(&db_path).unwrap();
        let (nodes, _) = store.get_stats().unwrap();
        assert_eq!(nodes, 0);
    }
}
