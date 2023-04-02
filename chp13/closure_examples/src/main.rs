use std::thread;

fn main() {

    // this closure captures an immutable reference to list
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    println!("");
    // this closure captures a mutable reference to list
    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list2);

    let mut borrows_mutably = || list2.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list2);
    println!("");


    //If you want to force the closure to take ownership of the values
    //using the move syntax
    let list3 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list3);

    thread::spawn(move || println!("From thread: {:?}", list3))
        .join()
        .unwrap();

}