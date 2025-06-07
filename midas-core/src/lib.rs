pub mod annual_profit;
pub mod index_code;
pub mod index_data;
pub mod model;
pub mod order_book;  // 新增订单簿模块
pub mod simulate;
pub mod slippage;
pub mod history;
pub mod mq;
pub mod parallel;

pub fn init_rayon_pool() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())  // 使用所有CPU核心
        .stack_size(4 * 1024 * 1024)  // 增大栈大小
        .build_global()
        .unwrap();
}
