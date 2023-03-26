# rust_book

#### Rust Book : https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

#### Rust Book Github : https://github.com/rust-lang/book/tree/main/src

#### Cargo Book : https://doc.rust-lang.org/cargo/

# Chapter 1 | Getting Started

```bash
    $ rustc main.rs
    $ ./main
        Hello, world!
```

### Create Project Using Cargo

```bash
    $ cargo new hello_cargo
    $ cd hello_cargo

```

### Building && Running Project Using Cargo

```bash
    $ cargo build  # Build the project

    $ cargo run  # Run the project

    $ cargo check  #Check if it compiles

    $ cargo build --release #This builds the project with optimizations
```

# Chapter 2 | Programming a Guessing Game

```bash
    $ cargo doc --open #Open Docs
```

# Chapter 3 | Common Programming Concepts

learned about variables, scalar and compound data types, functions, comments, if expressions, and loops!

## Practice Problems

- Convert temperatures between Fahrenheit and Celsius.
- Generate the nth Fibonacci number.
- Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

# Chapter 4 | Understanding Ownership

## Takeaways

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

# Chapter 5 | Using Structs

## Summary

- Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. In impl blocks, you can define functions that are associated with your type, and methods are a kind of associated function that let you specify the behavior that instances of your structs have.

# Chapter 6 | Enum and pattern Matching

- We’ve now covered how to use enums to create custom types that can be one of a set of enumerated values. We’ve shown how the standard library’s Option<T> type helps you use the type system to prevent errors. When enum values have data inside them, you can use match or if let to extract and use those values, depending on how many cases you need to handle.

# Chapter 7 | Managing Growing Projects with Packages, Crates, and Modules
