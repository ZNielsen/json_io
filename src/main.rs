extern crate serde;
extern crate serde_pickle;

mod structs;

use std::{path::Path, fs::OpenOptions};
use std::io::{prelude::*, stdin, stdout, Write};
use serde::{Serialize, de::DeserializeOwned};
// use chrono;
use crate::structs::{EntryType, Transaction, JsonTop};

const PICKLE_FILE : &str = "json.pkl";


fn main()
{
    println!("Rust JSON interface");

    let mut json: JsonTop = init_json();// = depickle_json(PICKLE_FILE).expect("Error depickling");
    let mut input = String::new();
    while input != "exit" {
        print!("> ");
        input = get_user_input(None);
        match input.as_str() {
            "save" => {
                pickle_json(&json, PICKLE_FILE);
                println!("JSON saved to {}", PICKLE_FILE);
            },
            "load" => {
                print!("Loading: Are you sure?  Existing JSON will be overwritten? (y/N): ");
                if get_user_input(None).as_str().to_uppercase() == "Y" {
                    json = depickle_json(PICKLE_FILE).expect("Error depickling");
                    println!("JSON loaded from {}", PICKLE_FILE);
                }
                else {
                    println!("Canceled");
                }
            },
            "show" => json.print(),
            "add" => {
                let e = get_addition_from_user();
                if e.is_some() {
                    json.add_entry(e.unwrap());
                }
            },
            "edit" => {
                // TODO: edit existing json object
            },
            "new" => {
                // TODO: Create new json
                let b = structs::Budget{ month: Vec::new() };
                let t: Vec<Transaction> = Vec::new();
                json = JsonTop{ transactions: t, budget: Some(b) };
            },
            _ =>  println!("Invalid command: {}", input),
        }
    }
    println!("Exiting");
}

fn get_addition_from_user() -> Option<EntryType>
{
    print!(r#"What would you like to do?
        1) Add Transaction (default)
        2) Add Budget Category

        Enter one of the numbers above: "#);
    match get_user_input(None).as_str() {
        "1" | "" => get_transaction_from_user(),
        "2" => {println!("TODO: add budget category"); None},
        _   => {println!("Error.  Please only provide one of the options shown."); None},
    }
}

fn get_transaction_from_user() -> Option<EntryType>
{
    let id = 0;

    let mut input: String;
    // Get Date
    input = get_user_input(Some("Date: "));
    let mut split = input.split("/");
    let day   = u8 ::from_str_radix(split.next().unwrap(), 10).unwrap();
    let month = u8 ::from_str_radix(split.next().unwrap(), 10).unwrap();
    let year  = u16::from_str_radix(split.next().unwrap(), 10).unwrap();
    let date  = structs::MyDate{ day, month, year };

    let payee = get_user_input(Some("Payee: "));

    let category = get_user_input(Some("Category: "));
    input = get_user_input(Some("Outflow (blank for none): "));

    let outflow = Some(i32::from_str_radix(&input, 10).unwrap());
    input = get_user_input(Some("Inflow (blank for none): "));

    let inflow = Some(i32::from_str_radix(&input, 10).unwrap());
    input = get_user_input(Some(r#"
        0) Credit card: Uncleared
        1) Credit card: Cleared
        2) Debit
        Cleared status (0-2): "#));

    let cleared = match input.as_str() {
        "0" => structs::TransactionStatus::CreditUncleared,
        "1" => structs::TransactionStatus::CreditCleared,
        "2" => structs::TransactionStatus::DirectBank,
        _   => structs::TransactionStatus::DirectBank,
    };

    let t = Transaction {
        id,
        date,
        payee,
        category,
        outflow,
        inflow,
        cleared
    };

    Some(EntryType::Transaction(t))
}

fn get_edit_from_user() -> Option<EntryType>
{
    print!(r#"What would you like to do?
        1) Edit Transaction
        2) Edit Budget Category

        Enter one of the numbers above: "#);
    match get_user_input(None).as_str() {
        "1" => {println!("TODO: edit transaction"); None},
        "2" => {println!("TODO: edit budget category"); None},
        _   => {println!("Error.  Please only provide one of the options shown."); None},
    }
}

fn get_user_input(out: Option<&str>) -> String
{
    let mut s = String::new();
    if out.is_some() {
        print!("{}", out.unwrap());
    }
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Error while receiving input");
    s.trim().to_owned()
}

fn pickle_json<T: Serialize + DeserializeOwned>
        (b: &T, file: &str) -> serde_pickle::Result<()>
{
    let filename = &(String::from("./")+file);
    let path = Path::new(filename);
    println!("path is {}", path.display());
    let ser = serde_pickle::to_vec(&b, true).expect("Pickling - converting to String");
    println!("ser is: {:?}", ser);
    let mut fp = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .expect(&format!("Opening file: {}", path.display()));
    fp.write(&ser).expect("Writing file");
    Ok(())
}
fn depickle_json<T: Serialize + DeserializeOwned>
        (file: &str) -> Result<T, std::io::Error>
{
    let filename = &(String::from("./")+file);
    let path = Path::new(filename);
    let mut fp = OpenOptions::new()
        .read(true)
        .open(path)
        .expect(&format!("Opening file: {}", path.display()));
    let mut ser: Vec<u8> = Vec::new();
    fp.read_to_end(&mut ser).expect(&format!("Reading from: {}", path.display()));
    println!("ser is {:?}", ser);
    let x = serde_pickle::from_slice(&ser).expect("Loading pickled data");
    Ok(x)
}
