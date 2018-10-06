use models::money::Money;

pub struct ItemCalculation {
    tax: Money,
    discount: Money,
    price: Money,
    amount: f64,
}

impl ItemCalculation {
    pub fn new(
        tax: Money,
        discount: Money,
        price: Money,
        amount: Option<f64>,
    ) -> Self {
        ItemCalculation {
            tax,
            discount,
            price,
            amount: amount.unwrap_or(0.0),
        }
    }

    pub fn subtotal(&self) -> Money {
        self.price * self.amount
    }

    pub fn subtotal_without_discount(&self) -> Money {
        self.subtotal() - self.calculate_discount()
    }

    pub fn calculate_total(&self) -> Money {
        self.subtotal_without_discount() + self.calculate_taxes()
    }

    pub fn calculate_discount(&self) -> Money {
        (self.discount * self.subtotal()) / 100
    }

    pub fn calculate_taxes(&self) -> Money {
        (self.subtotal_without_discount() * self.tax) / 100
    }
}
