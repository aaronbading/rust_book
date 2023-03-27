use std::collections::HashMap;

// Example 1 
// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// Example 2
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}


fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map);
}