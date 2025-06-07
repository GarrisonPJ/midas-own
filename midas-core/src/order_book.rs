use std::collections::{BTreeMap, HashMap};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: u64,
    pub price: f64,
    pub quantity: f64,
    pub is_buy: bool,
    pub timestamp: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OrderBook {
    bids: BTreeMap<u64, Vec<Order>>,  // 价格从高到低排序
    asks: BTreeMap<u64, Vec<Order>>,  // 价格从低到高排序
    order_map: HashMap<u64, Order>,   // 快速查找订单
}

impl OrderBook {
    pub fn new() -> Self {
        Self::default()
    }

    // 添加订单
    pub fn add_order(&mut self, order: Order) {
        let price_key = (order.price * 1000.0) as u64;
        let side = if order.is_buy { &mut self.bids } else { &mut self.asks };
        
        side.entry(price_key)
            .or_insert_with(Vec::new)
            .push(order.clone());
            
        self.order_map.insert(order.id, order);
    }

    // 删除订单
    pub fn remove_order(&mut self, order_id: u64) -> Option<Order> {
        if let Some(order) = self.order_map.remove(&order_id) {
            let price_key = (order.price * 1000.0) as u64;
            let side = if order.is_buy { &mut self.bids } else { &mut self.asks };
            
            if let Some(orders) = side.get_mut(&price_key) {
                orders.retain(|o| o.id != order_id);
                if orders.is_empty() {
                    side.remove(&price_key);
                }
            }
            Some(order)
        } else {
            None
        }
    }

    // 获取市场深度
    pub fn get_depth(&self, levels: usize) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
        let bids = self.bids.iter()
            .rev()
            .take(levels)
            .map(|(&p, orders)| {
                let price = p as f64 / 1000.0;
                let quantity = orders.iter().map(|o| o.quantity).sum();
                (price, quantity)
            })
            .collect();
            
        let asks = self.asks.iter()
            .take(levels)
            .map(|(&p, orders)| {
                let price = p as f64 / 1000.0;
                let quantity = orders.iter().map(|o| o.quantity).sum();
                (price, quantity)
            })
            .collect();
            
        (bids, asks)
    }

    // 生成快照
    pub fn snapshot(&self) -> Self {
        self.clone()
    }

    // 增量更新处理
    pub fn apply_delta(&mut self, delta: OrderBookDelta) {
        match delta {
            OrderBookDelta::Add(order) => self.add_order(order),
            OrderBookDelta::Remove(order_id) => { self.remove_order(order_id); },
            OrderBookDelta::Update(order) => {
                self.remove_order(order.id);
                self.add_order(order);
            }
        }
    }

    // 流动性分析
    pub fn liquidity_analysis(&self, price_levels: usize) -> LiquidityAnalysis {
        let (bids, asks) = self.get_depth(price_levels);
        let bid_volume: f64 = bids.iter().map(|(_, q)| q).sum();
        let ask_volume: f64 = asks.iter().map(|(_, q)| q).sum();
        
        LiquidityAnalysis {
            bid_volume,
            ask_volume,
            imbalance: (bid_volume - ask_volume) / (bid_volume + ask_volume).max(1.0),
        }
    }

    // 价差分析
    pub fn spread_analysis(&self) -> SpreadAnalysis {
        let best_bid = self.bids.iter().next_back().map(|(&p, _)| p as f64 / 1000.0);
        let best_ask = self.asks.iter().next().map(|(&p, _)| p as f64 / 1000.0);
        
        SpreadAnalysis {
            best_bid,
            best_ask,
            spread: match (best_bid, best_ask) {
                (Some(bid), Some(ask)) => Some(ask - bid),
                _ => None,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum OrderBookDelta {
    Add(Order),
    Remove(u64),
    Update(Order),
}

#[derive(Debug)]
pub struct LiquidityAnalysis {
    pub bid_volume: f64,
    pub ask_volume: f64,
    pub imbalance: f64,
}

#[derive(Debug)]
pub struct SpreadAnalysis {
    pub best_bid: Option<f64>,
    pub best_ask: Option<f64>,
    pub spread: Option<f64>,
}

#[derive(Debug, Clone)]
// 对应波动率分析结果
pub struct VolatilityAnalysis {
    pub mean: f64,        // 平均价格变化率
    pub std_dev: f64,     // 价格波动标准差  
    pub max_change: f64,  // 最大正向波动
    pub min_change: f64   // 最大负向波动
}

// 综合订单簿分析容器
pub struct OrderBookAnalysis {
    pub liquidity: LiquidityAnalysis, // 现有流动性分析
    pub volatility: VolatilityAnalysis, // 新增波动率分析
    pub order_flow: f64,  // 订单流不平衡值
    pub spread: f64       // 当前买卖价差
}

pub fn volatility_analysis(&self, window_size: usize) -> VolatilityAnalysis {
    let price_changes: Vec<_> = self.price_history
        .windows(window_size)
        .map(|w| (w[w.len()-1] - w[0]) / w[0])
        .collect();
        
    let mean = price_changes.iter().sum::<f64>() / price_changes.len() as f64;
    let variance = price_changes.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / price_changes.len() as f64;
    
    VolatilityAnalysis {
        mean,
        std_dev: variance.sqrt(),
        max_change: price_changes.iter().cloned().fold(f64::NAN, f64::max),
        min_change: price_changes.iter().cloned().fold(f64::NAN, f64::min),
    }
}

pub fn order_flow_imbalance(&self, lookback_period: usize) -> f64 {
    let bids: usize = self.bid_orders.iter().take(lookback_period).map(|o| o.quantity).sum();
    let asks: usize = self.ask_orders.iter().take(lookback_period).map(|o| o.quantity).sum();
    (bids as f64 - asks as f64) / (bids + asks).max(1) as f64
}

// 市场深度分析
pub fn depth_analysis(&self, levels: usize) -> HashMap<PriceLevel, f64> {
    self.get_depth(levels).into_iter()
        .map(|(price, qty)| (price, qty / self.total_volume))
        .collect()
}

// 订单流毒性指标
pub fn order_flow_toxicity(&self) -> f64 {
    let aggressive_buys = self.trade_history.iter()
        .filter(|t| t.is_buy && t.is_aggressive).count();
    (aggressive_buys as f64) / self.trade_history.len().max(1) as f64
}

// VWAP计算
pub fn vwap(&self, period: Duration) -> f64 {
    let filtered = self.trade_history.iter()
        .filter(|t| t.timestamp >= now() - period);
    let (sum_pq, sum_q) = filtered.fold((0.0, 0.0), |(acc_pq, acc_q), t| {
        (acc_pq + t.price * t.quantity, acc_q + t.quantity)
    });
    sum_pq / sum_q.max(1.0)
}