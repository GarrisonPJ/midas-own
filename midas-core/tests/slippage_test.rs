// 滑点计算模块单元测试

use midas_core::slippage::{calculate_slippage, compensate_slippage, analyze_slippage};

#[test]
fn test_calculate_slippage() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let result = calculate_slippage(&data);
    approx::assert_abs_diff_eq!(result, 0.375, epsilon = 0.001);
}

#[test]
fn test_compensate_slippage() {
    let original = 100.0;
    let compensated = compensate_slippage(original, 0.1);
    approx::assert_abs_diff_eq!(compensated, 100.1, epsilon = 0.001);
}

#[test]
fn test_analyze_slippage() {
    let result = analyze_slippage(0.05);
    assert!(result.contains("滑点级别"));
}