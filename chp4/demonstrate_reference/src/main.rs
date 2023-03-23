fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let s = String::from("hello");
    change(&s);  //ERROR BECAUSE ITS NOT A MUTABLE REFERENCE

}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

  fn change(some_string: &String) {
    some_string.push_str(", world");
}