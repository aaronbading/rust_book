
// fn longest_without_lifetimes(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("Hello, world!");

    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    let string_1 = String::from("abcd");
    let string_2 = String::from("xyz");
    let longest_string = longest(string_1.as_str(), string_2.as_str());
    println!("The longest string is {}", longest_string);


    //This works 
    // let string1 = String::from("long string is long");

    // {
    //     let string2 = String::from("xyz");
    //     let result = longest(string1.as_str(), string2.as_str());
    //     println!("The longest string is {}", result);
    // }

    //This does not work
    //Due to the fact that string2 is dropped before the println! statement
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
}
