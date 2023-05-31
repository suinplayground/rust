use std::marker::PhantomData;

pub trait TransactionType {}
pub struct Sell;
pub struct Buy;
impl TransactionType for Sell {}
impl TransactionType for Buy {}

pub struct Transaction<T: TransactionType> {
    price: f64,
    amount: i32,
    _marker: PhantomData<T>,
}

impl<T: TransactionType> Transaction<T> {
    pub fn new(price: f64, amount: i32) -> Self {
        Self {
            price,
            amount,
            _marker: PhantomData,
        }
    }

    pub fn total_price(&self) -> f64 {
        self.price * self.amount as f64
    }
}

impl<T: TransactionType + Tax> Transaction<T> {
    pub fn total_price_with_tax(&self) -> f64 {
        let tax = self.total_price() * T::tax_rate();
        self.total_price() + tax
    }
}

pub trait Tax {
    fn tax_rate() -> f64;
}

impl Tax for Sell {
    fn tax_rate() -> f64 {
        0.05
    }
}

impl Tax for Buy {
    fn tax_rate() -> f64 {
        0.12
    }
}

fn main() {
    let sell = Transaction::<Sell>::new(10.0, 5);
    let buy = Transaction::<Buy>::new(12.0, 4);

    println!("Sell total without tax: {}", sell.total_price());
    println!("Buy total without tax: {}", buy.total_price());

    println!("Sell total with tax: {}", sell.total_price_with_tax());
    println!("Buy total with tax: {}", buy.total_price_with_tax());
}
