
enum Message {
    Hello { id: i32 },
}



fn main() {
    println!("Hello, world!");

    let msg = Message::Hello { id: 5 };

//Using @ lets us test a value and save it in a variable within one pattern.


match msg {
    Message::Hello {
        id: id_variable @ 3..=7,
    } => println!("Found an id in range: {}", id_variable),
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    }
    Message::Hello { id } => println!("Found some other id: {}", id),
}
}
