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
    transactions: Vec<Transaction>,
    budget      : Budget,
}
#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
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

enum EntryType {
    Transaction(Transaction),
    Budget,
}

impl JsonTop {
    fn add_entry(&mut self, e: EntryType)
    {

    }

    fn print(&self)
    {
        println!("TODO: print json structure");
    }
}