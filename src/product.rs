#[derive(Debug, Clone)]
pub struct ProductInfo {
    pub category: String,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub specs: Option<String>,
    pub price: Option<String>,
    pub specific_model: Option<String>,
}
