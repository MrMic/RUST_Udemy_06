use category::Category;

pub struct Product {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) price: f64,
    pub(crate) category: Category,
}

impl Product {
    fn calculate_tax(&self) -> f64 {
        self.price * 0.1
    }

    pub fn product_price(&self) -> f64 {
        self.price - self.calculate_tax()
    }
}

// INFO: CATEGORY Module
mod category;
