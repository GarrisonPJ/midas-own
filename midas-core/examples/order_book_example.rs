use midas_core::order_book::{OrderBook, Order, OrderBookDelta};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let mut order_book = OrderBook::new();
    let (delta_tx, mut delta_rx) = mpsc::channel(100);
    
    // 启动市场数据处理
    tokio::spawn(async move {
        midas_core::market_data::start_market_data_processor(
            "order-book-deltas",
            order_book,
            delta_tx,
        ).await.unwrap();
    });

    // 处理增量更新
    while let Some(delta) = delta_rx.recv().await {
        println!("Received delta: {:?}", delta);
        
        // 实时分析
        let liquidity = order_book.liquidity_analysis(5);
        let spread = order_book.spread_analysis();
        println!("Liquidity: {:?}, Spread: {:?}", liquidity, spread);
    }
}