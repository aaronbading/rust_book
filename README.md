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

- Crate is generally synonymous with a library .
- Modules let us organize code within a crate for readability and easy reuse.
- Modules also allow us to control the privacy of items, because code within a module is private by default

- Create new library by running :

```bash
 cargo new lib_name --lib
```

- Summary
- Rust lets you split a package into multiple crates and a crate into modules so you can refer to items defined in one module from another module. You can do this by specifying absolute or relative paths. These paths can be brought into scope with a use statement so you can use a shorter path for multiple uses of the item in that scope. Module code is private by default, but you can make definitions public by adding the pub keyword.

# Chapter 8 | Common Collections

- Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access, and modify data.

# Chapter 9 | Error Handling

-Rust’s error handling features are designed to help you write more robust code. The panic! macro signals that your program is in a state it can’t handle and lets you tell the process to stop instead of trying to proceed with invalid or incorrect values. The Result enum uses Rust’s type system to indicate that operations might fail in a way that your code could recover from. You can use Result to tell code that calls your code that it needs to handle potential success or failure as well. Using panic! and Result in the appropriate situations will make your code more reliable in the face of inevitable problems.

# Chapter 10 | Generic Types, Traits, and Lifetimes

- Genric Types

- Traits are similar to interfaces in other languages
- Traits are used to define shared behavior in an abstract way
- Traits can be used to define a set of methods required to accomplish some purpose

- Lifetimes
- Summary : Now that you know about generic type parameters, traits and trait bounds, and generic lifetime parameters, you’re ready to write code without repetition that works in many different situations. Generic type parameters let you apply the code to different types. Traits and trait bounds ensure that even though the types are generic, they’ll have the behavior the code needs. You learned how to use lifetime annotations to ensure that this flexible code won’t have any dangling references. And all of this analysis happens at compile time, which doesn’t affect runtime performance!

# Chapter 11 | Writin Tests in Rust

```bash
 cargo test

 # run tests with a set amount of threads
 cargo test -- --test-threads=1


 # show output while running tests
 cargo test -- --show-output

 #running a specific test
 cargo test name_of_test_function

 # running tests using filtering
 cargo test filter

 # running only the ignored tests
 cargo test -- --ignored

 # Run tests whether ignored or not
 cargo test -- --include-ignored
```

- Summary : Rust’s testing features provide a way to specify how code should function to ensure it continues to work as you expect, even as you make changes. Unit tests exercise different parts of a library separately and can test private implementation details. Integration tests check that many parts of the library work together correctly, and they use the library’s public API to test the code in the same way external code will use it. Even though Rust’s type system and ownership rules help prevent some kinds of bugs, tests are still important to reduce logic bugs having to do with how your code is expected to behave.

# Chapter 12 | CI Tool Project

```bash
cargo run -- test poem.txt


cargo run -- body poem.txt

#ignoring the case
IGNORE_CASE=1 cargo run -- to poem.txt

#write contents to a file
cargo run -- to poem.txt > output.txt

```

## Some points on Test Driven Development

-Write a test that fails and run it to make sure it fails for the reason you expect.

- Write or modify just enough code to make the new test pass.
- Refactor the code you just added or changed and make sure the tests continue to pass.
- Repeat from step 1!

- Summary : This chapter recapped some of the major concepts you’ve learned so far and covered how to perform common I/O operations in Rust. By using command line arguments, files, environment variables, and the eprintln! macro for printing errors, you’re now prepared to write command line applications. Combined with the concepts in previous chapters, your code will be well organized, store data effectively in the appropriate data structures, handle errors nicely, and be well tested.

# Chapter 13 | Iterators and Closures

- Closures, a function-like construct you can store in a variable
- Iterators, a way of processing a series of elements
- How to use closures and iterators to improve the I/O project in Chapter 12
- The performance of closures and iterators
- Summary : Closures and iterators are Rust features inspired by functional programming language ideas. They contribute to Rust’s capability to clearly express high-level ideas at low-level performance. The implementations of closures and iterators are such that runtime performance is not affected. This is part of Rust’s goal to strive to provide zero-cost abstractions.

# Chapter 14 | More on Cargo and Crate.io

- Customize your build through release profiles
- Publish libraries on crates.io
- Organize large projects with workspaces
- Install binaries from crates.io
- Extend Cargo using custom commands

```bash
cargo doc --open
```

# Chapter 15 | Smart Pointers

- Summary : This chapter covered how to use smart pointers to make different guarantees and trade-offs from those Rust makes by default with regular references. The Box<T> type has a known size and points to data allocated on the heap. The Rc<T> type keeps track of the number of references to data on the heap so that data can have multiple owners. The RefCell<T> type with its interior mutability gives us a type that we can use when we need an immutable type but need to change an inner value of that type; it also enforces the borrowing rules at runtime instead of at compile time.

Also discussed were the Deref and Drop traits, which enable a lot of the functionality of smart pointers. We explored reference cycles that can cause memory leaks and how to prevent them using Weak<T>.

# Chapter 16 | Frearless Concurrency

- Do not communicate by sharing memory; instead, share memory by communicating.”

- What we will learn :
- How to create threads to run multiple pieces of code at the same time
- Message-passing concurrency, where channels send messages between threads
- Shared-state concurrency, where multiple threads have access to some piece of data
- The Sync and Send traits, which extend Rust’s concurrency guarantees to user-defined types as well as types provided by the standard library

- Summary : The Rust standard library provides channels for message passing and smart pointer types, such as Mutex<T> and Arc<T>, that are safe to use in concurrent contexts. The type system and the borrow checker ensure that the code using these solutions won’t end up with data races or invalid references. Once you get your code to compile, you can rest assured that it will happily run on multiple threads without the kinds of hard-to-track-down bugs common in other languages. Concurrent programming is no longer a concept to be afraid of: go forth and make your programs concurrent, fearlessly!

# Chapter 17 | OOP features in RUST

- Summary : No matter whether or not you think Rust is an object-oriented language after reading this chapter, you now know that you can use trait objects to get some object-oriented features in Rust. Dynamic dispatch can give your code some flexibility in exchange for a bit of runtime performance. You can use this flexibility to implement object-oriented patterns that can help your code’s maintainability. Rust also has other features, like ownership, that object-oriented languages don’t have. An object-oriented pattern won’t always be the best way to take advantage of Rust’s strengths, but is an available option.
