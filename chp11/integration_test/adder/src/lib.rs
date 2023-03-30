pub fn add_two(a: i32) -> i32 {
    a + 2
}


#[cfg(test)]
mod tests {
    // this brings the code in the outer scope into the tests scope
    use super::*;


    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

}