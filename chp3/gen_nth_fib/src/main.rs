use std::io;

fn main() {
    let n: u128 = get_input();
    if n <= 60 {
        println!("Calculating the {}th Fibonacci number...", n);
        let fib = fibonacci(n);
        println!("The {}th Fibonacci number is {}", n, fib);
    } else {
        println!("NOT Calculating the {}th Fibonacci number. It is too large. ", n);
    }
    
}

//Recursion in rust (: 
fn fibonacci(n: u128) -> u128 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn get_input() -> u128 {
    println!("Enter the value of n: ");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read input.");
    let n: u128 = n.trim().parse().expect("Invalid input. Please enter a positive integer.");
    n
}