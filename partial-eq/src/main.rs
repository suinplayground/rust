use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert("A", 1);
    map.insert("C", 3);
    map.insert("B", 2);
    for (_, value) in map {
        println!("{}", value);
    }
    // 1
    // 2
    // 3
}

trait ThreeFunctions {
    fn a() -> ();
    fn b() -> ();
    fn c() -> ();
}

#[derive(Debug)]
struct MyStruct {}

impl ThreeFunctions for MyStruct {
    fn a() -> () {}
    fn b() -> () {}
    fn c() -> () {}
}

pub struct Transaction<T: TransactionType> {
    pub price: f64,
    pub amount: f64,
    _marker: std::marker::PhantomData<T>,
}

impl<T: TransactionType> Transaction<T> {
    pub fn total_price(&self) -> f64 {
        self.price * self.amount
    }
}

pub trait TransactionType {}

pub struct Income;
pub struct Expense;

impl TransactionType for Income {}

impl TransactionType for Expense {}
