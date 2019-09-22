extern crate serde;
extern crate serde_pickle;
extern crate chrono;

mod structs;

use std::{path::Path, fs::OpenOptions};
use std::io::{prelude::*, stdin, stdout, Write};
use serde::{Serialize, de::DeserializeOwned};

const PICKLE_FILE : &str = "json.pkl";


fn main()
{
    println!("Rust JSON interface");

    let mut json = JsonTop::new();
    let mut record = String::new();
    let mut input  = String::new();
    while input != "exit" {
        input = get_input();
        match input.as_str() {
            "save" => {
                pickle_json(&record, PICKLE_FILE).expect("Error Pickeling");
                println!("record saved");
            },
            "load" => {
                record = depickle_json(PICKLE_FILE).expect("Error depickeling");
                println!("record loaded");
            },
            "show" => println!("record: {}", record),
            "add" => json.add_entry();
            _ =>  println!("Invalid command: {}", input),
        }
        println!("Record: {}", record);
    }
    println!("Exiting");
}

fn get_input() -> String
{
    let mut s = String::new();
    print!("> ");
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
    let ser = serde_pickle::to_vec(&b, true).expect("Pickeling - converting to String");
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
