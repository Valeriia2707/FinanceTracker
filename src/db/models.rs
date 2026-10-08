#[derive(Debug, Clone, PartialEq)]
pub struct Category {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewExpense {
    pub title: String,
    pub category_id: i64,
    pub date: String,
    pub original_amount: f64,
    pub currency: String,
    pub exchange_rate: f64,
    pub converted_amount_uah: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpenseItem {
    pub id: i64,
    pub title: String,
    pub category_name: String,
    pub date: String,
    pub original_amount: f64,
    pub currency: String,
    pub exchange_rate: f64,
    pub converted_amount_uah: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Income {
    pub id: i64,
    pub title: String,
    pub date: String, // Format YYYY-MM-DD
    pub original_amount: f64,
    pub currency: String, // USD, EUR, PLN, GBP, UAH
    pub exchange_rate: f64,
    pub converted_amount_uah: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExchangeRate {
    pub currency: String,
    pub date: String, // Format YYYY-MM-DD
    pub rate: f64,
}