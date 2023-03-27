fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();


    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);

    // characters
    for c in "Зд".chars() {
        println!("{c}");
    }
    // bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
    
    

}
