//! Memory Node: 记忆树中的单个节点
//!
//! 每个 MemoryNode 代表一个记忆单位，支持：
//! - 分层结构（父子关系）
//! - Token 计数和压缩
//! - 元数据存储
//! - 相关性评分

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 记忆节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryNode {
    /// 唯一标识
    pub node_id: String,
    /// 父节点 ID
    pub parent_id: Option<String>,
    /// 子节点列表
    pub child_ids: Vec<String>,
    /// 节点内容
    pub content: String,
    /// Token 计数
    pub token_count: usize,
    /// 相关性评分 (0.0 - 1.0)
    pub score: f32,
    /// 来源引用
    pub source_ref: String,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 元数据
    pub metadata: HashMap<String, String>,
    /// 是否重要
    pub is_important: bool,
    /// 访问计数
    pub access_count: u32,
    /// 上次访问
    pub last_accessed: Option<DateTime<Utc>>,
}

impl MemoryNode {
    /// 创建新节点
    pub fn new(content: String, source_ref: String) -> Self {
        let now = Utc::now();
        let token_count = Self::estimate_tokens(&content);

        Self {
            node_id: uuid::Uuid::new_v4().to_string(),
            parent_id: None,
            child_ids: Vec::new(),
            content,
            token_count,
            score: 1.0,
            source_ref,
            created_at: now,
            updated_at: now,
            metadata: HashMap::new(),
            is_important: false,
            access_count: 0,
            last_accessed: None,
        }
    }

    /// 估算 token 数量
    pub fn estimate_tokens(text: &str) -> usize {
        (text.len() + 3) / 4
    }

    /// 添加元数据
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// 标记为重要
    pub fn mark_important(&mut self) {
        self.is_important = true;
    }

    /// 记录访问
    pub fn record_access(&mut self) {
        self.access_count += 1;
        self.last_accessed = Some(Utc::now());
    }

    /// 计算相关性评分
    pub fn calculate_relevance_score(&self) -> f32 {
        let mut score = self.score;
        if self.is_important {
            score *= 1.5;
        }
        score *= 1.0 + (self.access_count as f32 * 0.1);
        let age_days = (Utc::now() - self.created_at).num_days();
        if age_days < 7 {
            score *= 1.2;
        }
        score.min(1.0)
    }

    /// 获取摘要
    pub fn get_summary(&self, max_chars: usize) -> String {
        if self.content.len() <= max_chars {
            self.content.clone()
        } else {
            format!("{}...", &self.content[..max_chars])
        }
    }

    /// 序列化为 JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// 从 JSON 反序列化
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_memory_node() {
        let node = MemoryNode::new(
            "Test memory content".to_string(),
            "test_source".to_string(),
        );
        assert!(!node.node_id.is_empty());
        assert_eq!(node.content, "Test memory content");
        assert_eq!(node.source_ref, "test_source");
        assert!(node.token_count > 0);
    }

    #[test]
    fn test_token_estimation() {
        let token_count = MemoryNode::estimate_tokens("Hello world");
        assert!(token_count > 0);
    }

    #[test]
    fn test_metadata() {
        let mut node = MemoryNode::new("Test".to_string(), "test".to_string());
        node.add_metadata("key1".to_string(), "value1".to_string());
        assert_eq!(node.metadata.get("key1"), Some(&"value1".to_string()));
    }

    #[test]
    fn test_importance_marking() {
        let mut node = MemoryNode::new("Test".to_string(), "test".to_string());
        assert!(!node.is_important);
        node.mark_important();
        assert!(node.is_important);
    }

    #[test]
    fn test_access_tracking() {
        let mut node = MemoryNode::new("Test".to_string(), "test".to_string());
        assert_eq!(node.access_count, 0);
        node.record_access();
        assert_eq!(node.access_count, 1);
        assert!(node.last_accessed.is_some());
    }

    #[test]
    fn test_relevance_score() {
        let mut node = MemoryNode::new("Test".to_string(), "test".to_string());
        node.mark_important();
        node.record_access();
        let score = node.calculate_relevance_score();
        assert!(score > 1.0);
    }

    #[test]
    fn test_summary() {
        let content = "This is a long memory content".to_string();
        let node = MemoryNode::new(content, "test".to_string());
        let summary = node.get_summary(20);
        assert!(summary.contains("..."));
    }

    #[test]
    fn test_json_serialization() {
        let node = MemoryNode::new("Test content".to_string(), "test".to_string());
        let json = node.to_json();
        let deserialized = MemoryNode::from_json(&json).unwrap();
        assert_eq!(node.node_id, deserialized.node_id);
    }
}
