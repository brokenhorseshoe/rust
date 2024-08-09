use std::io;

// path seperator operator is :: 
// io is a module in the standard library   

fn main() {
    let mut input = String::new();
    // String::new() is a static method of the String struct  
    // struct is a data type that groups named fields under a single name

    println!("This is a program writen in rust that reads user input than prints it out");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // expect is a method that is called on the Result type that takes a string as an argument and returns the value of the Ok variant if the Result is an Err variant it will print the string that was passed to the expect method and then the program will panic
    //&mut input is a mutable reference to the input variable that is stored in the memory not making a copy
    println!("You entered: {}", input);

}
