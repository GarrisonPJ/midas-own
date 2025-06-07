// 高频交易回测模拟引擎系统集成测试

use midas_core::parallel::{parallel_process, TradingData};
use midas_core::slippage::{calculate_slippage, analyze_slippage};
use std::sync::Arc;

#[tokio::test]
async fn test_integration_workflow() {
    // 准备测试数据
    let trading_data = Arc::new(vec![TradingData {
        symbol: "BTC".to_string(),
        time_range: (0, 100),
        data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
        timestamp_precision: 1,
    }]);

    // 执行并行处理
    let processed_data = parallel_process(trading_data.clone());

    // 计算滑点
    let slippage = calculate_slippage(&processed_data).await;

    // 分析滑点
    let analysis = analyze_slippage(slippage).await;

    // 验证结果
    assert!(analysis.contains("滑点级别"));
}