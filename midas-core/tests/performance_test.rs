// 高频交易回测模拟引擎性能测试

use midas_core::parallel::{parallel_process, TradingData};
use midas_core::slippage::{calculate_slippage, analyze_slippage};
use std::sync::Arc;
use tokio::time::{Instant, Duration};
use sysinfo::{System, SystemExt, ProcessExt};

#[tokio::test]
async fn test_performance() {
    // 初始化系统监控
    let mut sys = System::new_all();
    sys.refresh_all();

    // 准备测试数据
    let trading_data = Arc::new(vec![TradingData {
        symbol: "BTC".to_string(),
        time_range: (0, 100),
        data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
        timestamp_precision: 1,
    }]);

    // 开始计时
    let start = Instant::now();

    // 执行并行处理
    let processed_data = parallel_process(trading_data.clone());

    // 计算滑点
    let slippage = calculate_slippage(&processed_data).await;

    // 分析滑点
    let analysis = analyze_slippage(slippage).await;

    // 结束计时
    let duration = start.elapsed();

    // 输出执行时间
    println!("执行时间: {:?}", duration);

    // 输出CPU和内存使用情况
    sys.refresh_all();
    println!("CPU使用率: {}%", sys.global_cpu_info().cpu_usage());
    println!("内存使用: {}MB", sys.used_memory() / 1024 / 1024);

    // 验证结果
    assert!(analysis.contains("滑点级别"));


}