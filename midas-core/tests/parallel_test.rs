// 并行处理模块单元测试

use midas_core::parallel::{parallel_process, low_latency_parallel_process, TradingData};
use std::sync::Arc;

#[tokio::test]
async fn test_parallel_process() {
    // 准备测试数据
    let data = Arc::new(vec![TradingData {
        symbol: "BTC".to_string(),
        time_range: (0, 100),
        data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
        timestamp_precision: 1,
    }]);

    // 执行并行处理
    let result = parallel_process(data.clone());

    // 验证结果
    assert_eq!(result, vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0]);
}

#[tokio::test]
async fn test_low_latency_parallel_process() {
    // 准备测试数据
    let data = Arc::new(vec![TradingData {
        symbol: "ETH".to_string(),
        time_range: (0, 100),
        data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
        timestamp_precision: 1,
    }]);

    // 执行低延迟并行处理
    let result = low_latency_parallel_process(data.clone());

    // 验证结果
    assert_eq!(result, vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0]);
}