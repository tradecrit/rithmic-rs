#[derive(Debug, Clone)]
pub struct RithmicBracketOrder {
    pub action: i32,
    pub duration: i32,
    pub exchange: String,
    pub localid: String,
    pub ordertype: i32,
    pub price: Option<f64>,
    pub profit_ticks: i32,
    pub qty: i32,
    pub stop_ticks: i32,
    pub symbol: String,
}

#[derive(Debug, Clone)]
pub struct RithmicModifyOrder {
    pub id: String,
    pub exchange: String,
    pub symbol: String,
    pub qty: i32,
    pub price: f64,
    pub ordertype: i32,
}

#[derive(Debug, Clone)]
pub struct RithmicCancelOrder {
    pub id: String,
}
