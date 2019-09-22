extern crate serde;

use serde::{Serialize, Deserialize};


//
// Define structs
//
#[derive(Serialize, Deserialize, Debug)]
struct Category  {
    budgeted: i32,
    out     : i32,
    balance : i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Field {
    name: String,
    val : i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Month {
    entry: Vec<Category>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Budget {
    month: Vec<Month>,
}
#[derive(Serialize, Deserialize, Debug)]
struct JsonTop {
    transactions: Vec<Transactions>,
    budget      : Budget,
}
#[derive(Serialize, Deserialize, Debug)]
struct Transactions {
    id      : u128,
    date    : MyDate,
    payee   : String,
    category: String,
    outflow : Option<i32>,
    inflow  : Option<i32>,
    cleared : TransactionStatus,
}
#[derive(Serialize, Deserialize, Debug)]
enum TransactionStatus {
    CreditUncleared,
    CreditCleared,
    DirectBank,
}
#[derive(Serialize, Deserialize, Debug)]
struct MyDate {
    year : u32,
    month: u8,
    day  : u8,
}