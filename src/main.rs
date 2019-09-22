extern crate serde;
extern crate serde_pickle;

mod structs;

use std::{path::Path, fs::OpenOptions};
use std::io::{prelude::*, stdin, stdout, Write};
use serde::{Serialize, de::DeserializeOwned};

const PICKLE_FILE : &str = "json.pkl";


fn main()
{
    println!("Rust JSON interface");

    let mut json  = JsonTop::new();
    let mut input = String::new();
    while input != "exit" {
        print!("> ");
        input = get_user_input();
        match input.as_str() {
            "save" => {
                pickle_json(&json, PICKLE_FILE).expect("Error Pickling");
                println!("JSON saved to {}", PICKLE_FILE);
            },
            "load" => {
                print!("Loading: Are you sure?  Existing JSON will be overwritten? (y/N): ");
                if get_user_input().as_str().to_uppercase() == "Y" {
                    json = depickle_json(PICKLE_FILE).expect("Error depickling");
                    println!("JSON loaded from {}", PICKLE_FILE);
                }
                else {
                    println!("Canceled");
                }
            },
            "show" => json.print();
            "add" => {
                let e = get_entry_from_user();
                if (e.is_some()) {
                    json.add_entry(e.unwrap());
                }
            }
            _ =>  println!("Invalid command: {}", input),
        }
    }
    println!("Exiting");
}

fn get_entry_from_user() -> Option<EntryType>
{
    print!(r#"What would you like to do?
        1) Add Transaction (default)
        2) Add Budget Category
        3) Edit Transaction
        4) Edit Budget Category

        Enter one of the numbers above: "#);
    match get_user_input() {
        "1" | "" =>
        "2" => {println!("TODO: add budget category"); None},
        "3" => {println!("TODO: edit transaction"); None},
        "4" => {println!("TODO: edit budget category"); None},
        _   => {println!("Error.  Please only provide one of the options shown."); None},
    }
}

fn get_user_input() -> String
{
    let mut s = String::new();
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
