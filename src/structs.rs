////////////////////////////////////////////////////////////////////////////////
//  Module:   structs.rs
//
//  © Zach Nielsen
//  [description]
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
//  Included Modules
////////////////////////////////////////////////////////////////////////////////
extern crate serde;
extern crate serde_json;
extern crate serde_pickle;

////////////////////////////////////////////////////////////////////////////////
//  Namespaces
////////////////////////////////////////////////////////////////////////////////
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use self::pickle_fns::depickle_json;

////////////////////////////////////////////////////////////////////////////////
//  CODE
////////////////////////////////////////////////////////////////////////////////

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
    ////////////////////////////////////////////////////////////////////////////
     // NAME:   JsonTop::add_entry
     //
     // NOTES:
     // ARGS:
     // RETURN:
     //
    pub fn add_entry(&mut self, e: EntryType)
    {

    }
    ////////////////////////////////////////////////////////////////////////////
     // NAME:   JsonTop::print
     //
     // NOTES:  Prints the Json structure.
     // ARGS:   n/a
     // RETURN: n/a
     //
    pub fn print(&self)
    {
        println!("TODO: print json structure");
    }

    ////////////////////////////////////////////////////////////////////////////
     // NAME:   JsonTop::new
     //
     // NOTES:
     //     Attempts to load the Json object from the specified pickle file, or
     //     constructs a new blank one if None is passed in.
     // ARGS:   file - Path to a pickle file to load from
     // RETURN: JsonTop object
     //
    pub fn new(file: Option<&str>) -> Result<JsonTop, &str>
    {
        if file.is_some() {
            // Check if file exists.  Load it
            let filename = &(String::from("./")+file.unwrap());
            let path = std::path::Path::new(filename);
            if path.exists() {
                let j = depickle_json(path).expect("Error depickling");
                Ok(j)
            }
            else {
                return Err("Path does not exist");
            }
        }
        // Else, create empty data for JsonTop
        let transactions: Vec<Transaction> = Vec::new();
        let budget: Option<Budget> = None;
        let j: JsonTop = JsonTop{ transactions, budget };
        Ok(j)
    }
}

pub mod pickle_fns {
    ////////////////////////////////////////////////////////////////////////////
    // NAME:   pickle_json
    //
    // NOTES:
    // ARGS:
    // RETURN:
    //
    pub fn pickle_json<T: Serialize + DeserializeOwned>
            (b: &T, path: &std::path::Path) -> serde_pickle::Result<()>
    {
        println!("path is {}", path.display());
        let ser = serde_pickle::to_vec(&b, true).expect("Pickling - converting to String");
        println!("ser is: {:?}", ser);
        let mut fp = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)
            .expect(&format!("Opening file: {}", path.display()));
        fp.write(&ser).expect("Writing file");
        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////
    // NAME:   depickle_json
    //
    // NOTES:
    // ARGS:
    // RETURN:
    //
    pub fn depickle_json<T: Serialize + DeserializeOwned>
            (path: &std::path::Path) -> Result<T, std::io::Error>
    {
        let mut fp = std::fs::OpenOptions::new()
            .read(true)
            .open(&path)
            .expect(&format!("Opening file: {}", path.display()));
        let mut ser: Vec<u8> = Vec::new();
        fp.read_to_end(&mut ser).expect(&format!("Reading from: {}", path.display()));
        println!("ser is {:?}", ser);
        let x = serde_pickle::from_slice(&ser).expect("Loading pickled data");
        Ok(x)
    }
}