// 并行计算模块
// 使用rayon线程池处理不同交易品种或时间区间的数据

use rayon::prelude::*;
use std::sync::Arc;

// 交易品种和时间区间结构体
#[derive(Debug, Clone)]
pub struct TradingData {
    pub symbol: String,  // 交易品种
    pub time_range: (u64, u64),  // 时间区间
    pub data: Vec<f32>,  // 交易数据
    pub timestamp_precision: u32,  // 时间戳精度，单位：纳秒
}

// 并行处理函数
pub fn parallel_process(data: Arc<Vec<TradingData>>) -> Vec<f32> {
    use std::simd::f32x8;
    data.par_iter()
        .flat_map(|trading_data| {
            trading_data.data.par_chunks(8)
                .map(|chunk| {
                    let simd_data = f32x8::from_slice(chunk);
                    let result = simd_data * f32x8::splat(2.0);
                    result.to_array()
                })
                .flatten()
        })
        .collect()
}

// 针对超低延迟优化的并行处理
pub fn low_latency_parallel_process(data: Arc<Vec<TradingData>>) -> Vec<f32> {
    use std::simd::f32x8;
    data.par_iter()
        .with_min_len(512)  // 更细粒度的任务划分
        .flat_map(|trading_data| {
            trading_data.data.par_chunks(8)
                .with_min_len(256)  // 子任务粒度优化
                .map(|chunk| {
                    let simd_data = f32x8::from_slice(chunk);
                    let result = simd_data * f32x8::splat(2.0);
                    result.to_array()
                })
                .flatten()
        })
        .collect()
}