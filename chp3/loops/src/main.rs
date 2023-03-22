fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // break out of the inner loop
            }
            if count == 2 {
                break 'counting_up; // break out of the outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}\n");

    while_loop();

    for_loop();

    rev_for_loop();

}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!\n");

}

fn for_loop(){
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}\n");
    }
}

fn rev_for_loop(){

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!\n");
}