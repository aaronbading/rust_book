fn main() {
    let mut s = String::from("hello");

    change(&mut s);


    //Restriction on mutable references is that you can only have one mutable reference to one piece of data 
    
    let mut error_s = String::from("hello");

    let r1 = &mut error_s;
    let r2 = &mut error_s;  //This line results in an error 

    println!("{}, {}", r1, r2);

    //The following is okay 
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;


    /// Another ERROR in the following : if r1 and r2 are not used, the code will compile fine.
    /// 
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);

    //The following is okay

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);


}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}