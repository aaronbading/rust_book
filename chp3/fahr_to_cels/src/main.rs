use std::io;

fn main() {
    println!("Enter the temperature in Fahrenheit: ");
    let f: f64 = get_fahrenheit_input();
    let c = convert_to_celsius(f);
    println!("The temperature in Celsius is: {:.2}°C", c);
}

fn get_fahrenheit_input() -> f64 {
    let mut f = String::new();
    io::stdin().read_line(&mut f).expect("Failed to read input.");
    let f: f64 = f.trim().parse().expect("Invalid input. Please enter a number.");
    f
}
fn convert_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}