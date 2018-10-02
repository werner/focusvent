use models::item_calc_methods::ItemCalcMethod;

pub struct CalcMethods {
    items: Vec<ItemCalcMethod>
}

impl CalcMethods {
    pub fn new(
        items: Vec<ItemCalcMethod>
    ) -> Self {
        CalcMethods { items }
    }

    pub fn subtotal(&self) -> f64 {
        self.items.iter().map(|item| item.subtotal()).collect::<Vec<f64>>().iter().sum()
    }

    pub fn subtotal_without_discount(&self) -> f64 {
        self.subtotal() - self.calculate_discount()
    }

    pub fn calculate_total(&self) -> f64 {
        self.subtotal_without_discount() + self.calculate_taxes()
    }

    pub fn calculate_discount(&self) -> f64 {
        self.items.iter().map(|item| item.calculate_discount()).collect::<Vec<f64>>().iter().sum()
    }

    pub fn calculate_taxes(&self) -> f64 {
        self.items.iter().map(|item| item.calculate_taxes()).collect::<Vec<f64>>().iter().sum()
    }
}