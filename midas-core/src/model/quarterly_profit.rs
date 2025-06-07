#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuarterlyProfit {
    pub quarter: String,  // 格式如 "2022-Q1"
    pub index_profit: f64,
    pub ma_profit: f64,
}