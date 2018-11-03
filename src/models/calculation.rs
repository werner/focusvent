use crate::models::money::Money;
use crate::models::item_calculation::ItemCalculation;

pub struct Calculation {
    items: Vec<ItemCalculation>
}

impl Calculation {
    pub fn new(
        items: Vec<ItemCalculation>
    ) -> Self {
        Calculation { items }
    }

    pub fn subtotal(&self) -> Money {
        self.items.iter().map(|item| item.subtotal()).collect::<Vec<Money>>().iter().sum()
    }

    pub fn subtotal_without_discount(&self) -> Money {
        self.subtotal() - self.calculate_discount()
    }

    pub fn calculate_total(&self) -> Money {
        self.subtotal_without_discount() + self.calculate_taxes()
    }

    pub fn calculate_discount(&self) -> Money {
        self.items.iter().map(|item| item.calculate_discount()).collect::<Vec<Money>>().iter().sum()
    }

    pub fn calculate_taxes(&self) -> Money {
        self.items.iter().map(|item| item.calculate_taxes()).collect::<Vec<Money>>().iter().sum()
    }
}
