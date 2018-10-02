pub struct ItemCalcMethod {
    tax: i32,
    discount: i32,
    price: i32,
    amount: f64,
}

impl ItemCalcMethod {
    pub fn new(
        tax: Option<i32>,
        discount: Option<i32>,
        price: Option<i32>,
        amount: Option<f64>,
    ) -> Self {
        ItemCalcMethod {
            tax: tax.unwrap_or(0),
            discount: discount.unwrap_or(0),
            price: price.unwrap_or(0),
            amount: amount.unwrap_or(0.0),
        }
    }

    pub fn subtotal(&self) -> f64 {
        self.price as f64 * self.amount
    }

    pub fn subtotal_without_discount(&self) -> f64 {
        self.subtotal() - self.calculate_discount()
    }

    pub fn calculate_total(&self) -> f64 {
        self.subtotal_without_discount() + self.calculate_taxes()
    }

    pub fn calculate_discount(&self) -> f64 {
        (self.discount as f64 * self.subtotal()) / 100.0
    }

    pub fn calculate_taxes(&self) -> f64 {
        (self.subtotal_without_discount() * self.tax as f64) / 100.0
    }
}
