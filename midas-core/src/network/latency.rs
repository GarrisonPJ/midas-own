//! 网络延迟计算模块

use std::simd::f32x8;
use tokio::time::{self, Duration};

/// 表示网络延迟的结构体
#[derive(Debug, Clone)]
pub struct NetworkLatency {
    pub min: f32,
    pub max: f32,
    pub avg: f32,
}

/// 使用SIMD指令优化延迟计算
pub async fn calculate_latency(distance: f32, hop_count: u32) -> NetworkLatency {
    // 计算物理距离延迟（假设光速为299792458 m/s）
        // 使用SIMD优化计算
        // 使用SIMD批量处理8个距离值
        let distances = f32x8::from_array([distance, distance * 1.1, distance * 0.9, distance * 1.2, distance * 0.8, distance * 1.3, distance * 0.7, distance * 1.4]);
        let light_speed = f32x8::splat(299792458.0);
        let distance_delays = distances / light_speed;
        // 取第一个距离值作为基准延迟
        let distance_delay = distance_delays[0];

        // 网络设备处理时间（假设每个设备处理时间为0.0001秒）
        let device_processing_time = hop_count as f32 * 0.0001;

        // 传输协议开销（假设TCP协议开销为0.00005秒）
        let protocol_overhead = 0.00005;

        // 总延迟
        let total_latency = distance_delay + device_processing_time + protocol_overhead;

        NetworkLatency {
            min: total_latency * 0.9,
            max: total_latency * 1.1,
            avg: total_latency,
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    #[tokio::test]
    async fn test_calculate_latency() {
        let latency = calculate_latency(1000.0, 5).await;
        assert!(latency.min > 0.0);
        assert!(latency.max > 0.0);
        assert!(latency.avg > 0.0);
        assert!(latency.min < latency.avg);
        assert!(latency.avg < latency.max);
    }

    pub fn criterion_benchmark(c: &mut Criterion) {
        c.bench_function("calculate_latency", |b| {
            b.to_async(tokio::runtime::Runtime::new().unwrap())
                .iter(|| async { calculate_latency(black_box(1000.0), black_box(5)).await });
        });
    }

    criterion_group!(benches, criterion_benchmark);
    criterion_main!(benches);
}