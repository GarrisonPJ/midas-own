//! 网络模块，包含网络延迟计算和拓扑结构模拟
//!
//! 该模块提供了网络延迟计算和拓扑结构模拟的核心功能。

pub mod latency;
pub mod topology;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_initialization() {
        // 测试模块初始化
        assert!(true);
    }
}