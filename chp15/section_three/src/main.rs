struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    //demonstrates the drop function getting called automatically when a variable goes out of scope
    // let c = CustomSmartPointer {
    //     data: String::from("my stuff"),
    // };
    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };
    // println!("CustomSmartPointers created.");


    // demonstrates that we cannot explicitly call the drop function
    //it would result in a double free error 
    //instead call std::mem::drop
    // let c = CustomSmartPointer {
    //     data: String::from("some data"),
    // };
    // println!("CustomSmartPointer created.");
    // c.drop();
    // println!("CustomSmartPointer dropped before the end of main.");


    // demonstrates that we can explicitly call the drop function
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}