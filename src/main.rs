use serde::{Serialize, Deserialize};
use serde_json::{Result};


//
// Define structs 
//
#[derive(Serialize, Deserialize)]
struct Category  {
    budgeted: i32,
    out: i32,
    balance: i32,
}


fn main() {
    println!("Hello, world!");
}
