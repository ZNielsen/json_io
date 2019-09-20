extern crate serde;
extern crate serde_pickle;

use std::{fs::File, io::prelude::*};
use serde::{Serialize, Deserialize};
// use serde_json::{Result};

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

fn main() {
    println!("Hello, world!");
}


fn pickle_json(b: &Budget, file: &str) -> serde_pickle::Result<()> {
    let ser = serde_pickle::to_vec(&b, true)?;
    let mut fp = File::open(file)?;
    fp.write(&ser)?;
    Ok(())
}
fn depickle_json(file: &str) -> serde_pickle::Result<Budget> {
    let mut fp = File::open(file)?;
    let mut ser = String::new();
    fp.read_to_string(&mut ser)?;
    let b: Budget = serde_json::from_str(ser.as_str()).unwrap();
    Ok(b)
}