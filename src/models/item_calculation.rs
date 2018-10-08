use models::money::Money;

#[derive(Clone, Debug)]
pub struct ItemCalculation {
    tax: Money,
    discount: Money,
    price: Money,
    amount: f64,
}

impl ItemCalculation {
    pub fn new(
        tax: &Money,
        discount: &Money,
        price: &Money,
        amount: f64,
    ) -> Self {
        ItemCalculation {
            tax: (*tax).clone(),
            discount: (*discount).clone(),
            price: (*price).clone(),
            amount,
        }
    }

    pub fn subtotal(&self) -> Money {
        self.clone().price * self.amount
    }

    pub fn subtotal_without_discount(&self) -> Money {
        self.subtotal() - self.calculate_discount()
    }

    pub fn calculate_total(&self) -> Money {
        self.subtotal_without_discount() + self.calculate_taxes()
    }

    pub fn calculate_discount(&self) -> Money {
        (self.clone().discount * self.subtotal()) / 100.0
    }

    pub fn calculate_taxes(&self) -> Money {
        (self.subtotal_without_discount() * self.clone().tax) / 100.0
    }
}
