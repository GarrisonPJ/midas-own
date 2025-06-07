use tokio::sync::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;

/// 历史数据存储结构
#[derive(Debug)]
pub struct HistoryStore {
    spreads: Arc<Mutex<VecDeque<f64>>>,
    order_flows: Arc<Mutex<VecDeque<f64>>>,
    volatilities: Arc<Mutex<VecDeque<f64>>>,  // 存储波动率数据
    order_flow_toxicities: Arc<Mutex<VecDeque<f64>>>,  // 存储订单流毒性指标
    spread_file: Arc<Mutex<tokio::fs::File>>,  // 价差数据文件
    order_flow_file: Arc<Mutex<tokio::fs::File>>,  // 订单流数据文件
    volatility_file: Arc<Mutex<tokio::fs::File>>,  // 波动率数据文件
    toxicity_file: Arc<Mutex<tokio::fs::File>>,  // 订单流毒性数据文件
}

impl HistoryStore {
    /// 创建新的历史数据存储
    pub async fn new() -> Self {
        let spread_file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("spread_history.dat")
            .await
            .unwrap();
        let order_flow_file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("order_flow_history.dat")
            .await
            .unwrap();
        let volatility_file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("volatility_history.dat")
            .await
            .unwrap();
        let toxicity_file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .await
            .open("toxicity_history.dat")
            .unwrap();
        Self {
            spreads: Arc::new(Mutex::new(VecDeque::with_capacity(1000))),
            order_flows: Arc::new(Mutex::new(VecDeque::with_capacity(1000))),
            volatilities: Arc::new(Mutex::new(VecDeque::with_capacity(1000))),
            order_flow_toxicities: Arc::new(Mutex::new(VecDeque::with_capacity(1000))),
            spread_file: Arc::new(Mutex::new(spread_file)),
            order_flow_file: Arc::new(Mutex::new(order_flow_file)),
            volatility_file: Arc::new(Mutex::new(volatility_file)),
            toxicity_file: Arc::new(Mutex::new(toxicity_file)),
        }
    }

    /// 异步存储价差数据
    pub async fn store_spread(&self, spread: f64) {
        let mut spreads = self.spreads.lock().await;
        if spreads.len() >= 1000 {
            spreads.pop_front();
        }
        spreads.push_back(spread);
        let mut file = self.spread_file.lock().await;
        let _ = file.write_all(format!("{}\n", spread).as_bytes()).await.unwrap();
    }

    /// 异步存储订单流数据
    pub async fn store_order_flow(&self, order_flow: f64) {
        let mut order_flows = self.order_flows.lock().await;
        if order_flows.len() >= 1000 {
            order_flows.pop_front();
        }
        order_flows.push_back(order_flow);
        let mut file = self.order_flow_file.lock().await;
        let _ = file.write_all(format!("{}\n", order_flow).as_bytes()).await.unwrap();
    }

    /// 获取历史价差数据
    pub async fn get_spreads(&self) -> Vec<f64> {
        let spreads = self.spreads.lock().await;
        spreads.iter().cloned().collect()
    }

    /// 获取历史订单流数据
    pub async fn get_order_flows(&self) -> Vec<f64> {
        let order_flows = self.order_flows.lock().await;
        order_flows.iter().cloned().collect()
    }

    /// 异步存储波动率数据
    pub async fn store_volatility(&self, volatility: f64) {
        let mut volatilities = self.volatilities.lock().await;
        if volatilities.len() >= 1000 {
            volatilities.pop_front();
        }
        volatilities.push_back(volatility);
        let mut file = self.volatility_file.lock().await;
        let _ = file.write_all(format!("{}\n", volatility).as_bytes()).await.unwrap();
    }

    /// 获取历史波动率数据
    pub async fn get_volatilities(&self) -> Vec<f64> {
        let volatilities = self.volatilities.lock().await;
        volatilities.iter().cloned().collect()
    }

    /// 异步存储订单流毒性指标
    pub async fn store_order_flow_toxicity(&self, toxicity: f64) {
        let mut toxicities = self.order_flow_toxicities.lock().await;
        if toxicities.len() >= 1000 {
            toxicities.pop_front();
        }
        toxicities.push_back(toxicity);
        let mut file = self.toxicity_file.lock().await;
        let _ = file.write_all(format!("{}\n", toxicity).as_bytes()).await.unwrap();
    }
}