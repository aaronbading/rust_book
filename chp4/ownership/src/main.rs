fn main() {

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`



    let x = 5; //Creates an actual copy
    let y = x;

    let s1 = String::from("hello"); //Creates a pointer to the string
    let s2 = s1;    //copies the pointer . RESULTS IN ERROR 
    let s2 = s1.clone(); //deep copy..
    println!("{}, world!", s1);

}
