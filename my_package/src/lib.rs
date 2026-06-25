// INFO: PRODUCT Module
mod product {
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
    mod category {
        pub enum Category {
            Electronics,
            Clothing,
            Books,
        }
    }
}

// INFO: CUSTOMER Module
mod customer {
    pub struct Customer {
        pub(crate) id: u64,
        pub(crate) name: String,
        pub(crate) email: String,
    }
}

// INFO: ORDER Module
mod order {
    use crate::{customer::Customer, product::Product};

    struct Order {
        id: u64,
        product: Product,
        customer: Customer,
        quantity: u32,
    }

    impl Order {
        fn calculate_discount(&self) -> f64 {
            if self.quantity > 5 {
                0.1
            } else {
                0.0
            }
        }

        fn total_bill(&self) -> f64 {
            let discount = self.calculate_discount();
            let total_before_discount = self.product.product_price() * self.quantity as f64;
            total_before_discount - (total_before_discount * discount)
        }
    }
}
