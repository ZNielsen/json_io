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
pub struct Month {
    entry: Vec<Category>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Budget {
    pub month: Vec<Month>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonTop {
    pub transactions: Vec<Transaction>,
    pub budget      : Option<Budget>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id      : u128,
    pub date    : MyDate,
    pub payee   : String,
    pub category: String,
    pub outflow : Option<i32>,
    pub inflow  : Option<i32>,
    pub cleared : TransactionStatus,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionStatus {
    CreditUncleared,
    CreditCleared,
    DirectBank,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MyDate {
    pub year : u16,
    pub month: u8,
    pub day  : u8,
}

pub enum EntryType {
    Transaction(Transaction),
    Budget,
}

impl JsonTop {
    pub fn add_entry(&mut self, e: EntryType)
    {

    }

    pub fn print(&self)
    {
        println!("TODO: print json structure");
    }

    pub fn new(file: &str) -> JsonTop
    {
        // Check if file exists.  Load it
        // Else, create empty data for JsonTop
    }
}
