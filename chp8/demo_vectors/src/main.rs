fn show_how_to_access_vectors() {

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

}
// fn show_immutable_borrow_error() {

//     let mut v = vec![1, 2, 3, 4, 5];

//     let first = &v[0];

//     v.push(6);

//     println!("The first element is: {first}");


// }

fn show_access_outside_of_bounds(){

    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

}

fn iterate_over_vector_immutable() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

}
fn iterate_over_vector_mutable(){

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn store_different_types_using_enum(){

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}

fn main() {
    println!("Hello, world!");
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    println!("{:?}", v);


    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    show_how_to_access_vectors();
    // show_access_outside_of_bounds();
    // show_immutable_borrow_error();

    iterate_over_vector_immutable();
    iterate_over_vector_mutable();

    store_different_types_using_enum();

}
