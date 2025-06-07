//! 网络拓扑结构模拟模块

use std::collections::HashMap;

/// 表示网络节点的结构体
#[derive(Debug, Clone)]
pub struct NetworkNode {
    pub id: u32,
    pub name: String,
    pub connections: Vec<u32>,
}

/// 表示网络拓扑的结构体
#[derive(Debug, Clone)]
pub struct NetworkTopology {
    pub nodes: HashMap<u32, NetworkNode>,
}

impl NetworkTopology {
    /// 创建新的网络拓扑
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// 添加网络节点
    pub fn add_node(&mut self, node: NetworkNode) {
        self.nodes.insert(node.id, node);
    }

    /// 删除网络节点
    pub fn remove_node(&mut self, node_id: u32) {
        // 删除节点
        self.nodes.remove(&node_id);

        // 删除其他节点中对该节点的引用
        for node in self.nodes.values_mut() {
            node.connections.retain(|&id| id != node_id);
        }
    }

    /// 添加节点连接
    pub fn add_connection(&mut self, from: u32, to: u32) {
        if let Some(node) = self.nodes.get_mut(&from) {
            if !node.connections.contains(&to) {
                node.connections.push(to);
            }
        }
    }

    /// 删除节点连接
    pub fn remove_connection(&mut self, from: u32, to: u32) {
        if let Some(node) = self.nodes.get_mut(&from) {
            node.connections.retain(|&id| id != to);
        }
    }

    /// 计算最短路径
    pub fn shortest_path(&self, start: u32, end: u32) -> Option<Vec<u32>> {
        use std::collections::{HashMap, VecDeque};

        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, vec![start]));

        while let Some((current, path)) = queue.pop_front() {
            if current == end {
                return Some(path);
            }

            if let Some(node) = self.nodes.get(&current) {
                for &neighbor in &node.connections {
                    if !visited.contains_key(&neighbor) {
                        visited.insert(neighbor, true);
                        let mut new_path = path.clone();
                        new_path.push(neighbor);
                        queue.push_back((neighbor, new_path));
                    }
                }
            }
        }
        None
    }

    /// 检查网络连通性
    pub fn is_connected(&self) -> bool {
        if self.nodes.is_empty() {
            return true;
        }

        let mut visited = HashMap::new();
        let mut stack = vec![];

        if let Some(first_node) = self.nodes.keys().next() {
            stack.push(*first_node);
        }

        while let Some(node_id) = stack.pop() {
            if visited.contains_key(&node_id) {
                continue;
            }
            visited.insert(node_id, true);

            if let Some(node) = self.nodes.get(&node_id) {
                for &neighbor in &node.connections {
                    if !visited.contains_key(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }

        visited.len() == self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_topology() {
        let mut topology = NetworkTopology::new();
        let node = NetworkNode {
            id: 1,
            name: "Node1".to_string(),
            connections: vec![2, 3],
        };
        topology.add_node(node);
        assert_eq!(topology.nodes.len(), 1);
    }

    #[tokio::test]
    async fn test_analyze_topology() {
        // 创建测试拓扑结构
        let topology = Topology::new();
        topology.add_node("node1").await;
        topology.add_node("node2").await;
        topology.add_edge("node1", "node2").await;

        // 分析拓扑结构
        let analysis = topology.analyze().await;

        // 验证分析结果
        assert_eq!(analysis.node_count, 2);
        assert_eq!(analysis.edge_count, 1);
        assert!(analysis.is_connected);
    }

    pub fn criterion_benchmark(c: &mut Criterion) {
        c.bench_function("analyze_topology", |b| {
            b.to_async(tokio::runtime::Runtime::new().unwrap())
                .iter(|| async {
                    let topology = Topology::new();
                    topology.add_node("node1").await;
                    topology.add_node("node2").await;
                    topology.add_edge("node1", "node2").await;
                    topology.analyze().await
                });
        });
    }

    criterion_group!(benches, criterion_benchmark);
    criterion_main!(benches);
}