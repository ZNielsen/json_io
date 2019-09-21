extern crate serde;
extern crate serde_pickle;
extern crate chrono;

use std::{fs::File};
use std::io::{prelude::*, stdin, stdout, Write}
use serde::{Serialize, Deserialize};

const PICKLE_FILE : &str = "json.pkl";

//
// Define structs
//
#[derive(Serialize, Deserialize, Debug)]
struct Category  {
    budgeted: i32,
    out: i32,
    balance: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Field {
    name: String,
    val: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Budget {
    field: Field,
    cat: Category,
}
#[derive(Serialize, Deserialize, Debug)]
struct Transactions {
    id    : u128,
    date  : chrono::Date,
    payee : String,
    category: String,
    outflow: Option<i32>,
    inflow: Option<i32>,
    cleared: TransactionStatus,
}
enum TransactionStatus {
    CreditUncleared;
    CreditCleared;
    DirectBank;
}

fn main() {
    println!("Rust JSON interface");
    let mut input = String::new();
    while input != "exit" {
        input = get_input();

        println!("Got : {}", input);
    }
    println!("Exiting");
}

fn get_input() -> String {
    print("> ");
    stdin()
        .read_line(&mut s)
        .expect("Error while receiving input")
        .trim()
        .to_owned()
}

fn pickle_json(b: &JsonTop, file: &str) -> serde_pickle::Result<()> {
    let ser = serde_pickle::to_vec(&b, true)?;
    let mut fp = File::open(file)?;
    fp.write(&ser)?;
    Ok(())
}
fn depickle_json(file: &str) -> serde_pickle::Result<JsonTop> {
    let mut fp = File::open(file)?;
    let mut ser = String::new();
    fp.read_to_string(&mut ser)?;
    let b: JsonTop = serde_json::from_str(ser.as_str()).unwrap();
    Ok(b)
}
