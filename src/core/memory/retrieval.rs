//! Retrieval Engine: 6 种智能检索方式

use crate::core::memory::MemoryNode;
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;

/// 检索引擎
pub struct RetrievalEngine;

impl RetrievalEngine {
    /// 方式1: 实体搜索
    pub fn search_entities(nodes: &[MemoryNode], query: &str, limit: usize) -> Vec<MemoryNode> {
        let query_lower = query.to_lowercase();
        let mut results: Vec<_> = nodes
            .iter()
            .filter(|node| node.content.to_lowercase().contains(&query_lower))
            .cloned()
            .collect();
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.into_iter().take(limit).collect()
    }

    /// 方式2: 主题查询
    pub fn query_topic(nodes: &[MemoryNode], topic: &str, days: u32) -> Vec<MemoryNode> {
        let now = Utc::now();
        let cutoff = now - Duration::days(days as i64);
        nodes
            .iter()
            .filter(|node| node.created_at >= cutoff && node.content.to_lowercase().contains(&topic.to_lowercase()))
            .cloned()
            .collect()
    }

    /// 方式3: 时间范围查询
    pub fn query_time_range(nodes: &[MemoryNode], start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<MemoryNode> {
        nodes
            .iter()
            .filter(|node| node.created_at >= start && node.created_at <= end)
            .cloned()
            .collect()
    }

    /// 方式4: 来源查询
    pub fn query_source(nodes: &[MemoryNode], source: &str, limit: usize) -> Vec<MemoryNode> {
        let source_lower = source.to_lowercase();
        let mut results: Vec<_> = nodes
            .iter()
            .filter(|node| node.source_ref.to_lowercase() == source_lower)
            .cloned()
            .collect();
        results.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        results.into_iter().take(limit).collect()
    }

    /// 方式5: 语义搜索
    pub fn semantic_search(nodes: &[MemoryNode], query: &str, limit: usize) -> Vec<MemoryNode> {
        let query_tokens = Self::tokenize(query);
        let mut scored: Vec<_> = nodes
            .iter()
            .map(|node| {
                let node_tokens = Self::tokenize(&node.content);
                let similarity = Self::calculate_similarity(&query_tokens, &node_tokens);
                (node.clone(), similarity)
            })
            .filter(|(_, score)| *score > 0.0)
            .collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().take(limit).map(|(node, _)| node).collect()
    }

    /// 方式6: 深度下钻
    pub fn drill_down(nodes: &[MemoryNode], parent_id: &str, depth: u32) -> Vec<MemoryNode> {
        if depth == 0 { return vec![]; }
        let mut results = vec![];
        let mut to_process = vec![parent_id.to_string()];
        let mut processed = std::collections::HashSet::new();
        for _ in 0..depth {
            let mut next_level = vec![];
            for current_id in to_process {
                if processed.contains(&current_id) { continue; }
                processed.insert(current_id.clone());
                for node in nodes {
                    if node.parent_id.as_ref() == Some(&current_id) {
                        results.push(node.clone());
                        next_level.push(node.node_id.clone());
                    }
                }
            }
            to_process = next_level;
            if to_process.is_empty() { break; }
        }
        results
    }

    fn tokenize(text: &str) -> Vec<String> {
        text.to_lowercase().split_whitespace().map(|s| s.to_string()).collect()
    }

    fn calculate_similarity(query_tokens: &[String], doc_tokens: &[String]) -> f32 {
        let mut query_counts: HashMap<String, f32> = HashMap::new();
        for token in query_tokens {
            *query_counts.entry(token.clone()).or_insert(0.0) += 1.0;
        }
        let mut doc_counts: HashMap<String, f32> = HashMap::new();
        for token in doc_tokens {
            *doc_counts.entry(token.clone()).or_insert(0.0) += 1.0;
        }
        let mut dot_product = 0.0;
        for (token, count) in &query_counts {
            if let Some(doc_count) = doc_counts.get(token) {
                dot_product += count * doc_count;
            }
        }
        let query_magnitude: f32 = query_counts.values().map(|c| c * c).sum::<f32>().sqrt();
        let doc_magnitude: f32 = doc_counts.values().map(|c| c * c).sum::<f32>().sqrt();
        if query_magnitude == 0.0 || doc_magnitude == 0.0 { return 0.0; }
        dot_product / (query_magnitude * doc_magnitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_search() {
        let nodes = vec![
            MemoryNode::new("Python programming".to_string(), "blog".to_string()),
            MemoryNode::new("Python learning".to_string(), "personal".to_string()),
        ];
        let results = RetrievalEngine::search_entities(&nodes, "Python", 10);
        assert_eq!(results.len(), 2);
    }
}
