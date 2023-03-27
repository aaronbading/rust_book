use rand::Rng;
use std::collections::HashMap;


use std::io;
use std::io::Write;
// equivalent to 
use std::io::{self, Write};


fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}