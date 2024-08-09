fn main() {
    // functions in rust
    // functions are defined using fn keyword
    // function name should be snake_case
    // function name should be descriptive
    // function name should not start with number
    // function name should not contain special characters
    // function name should not contain keywords
    // function name should not contain spaces
    // function name should not contain hyphens
    add_number(10, 20);
    test_one();
    let result = add_numbers(10, 20);
    println!("The result is {}", result);

    let number = {
        let x =3;
        x + 1 // no semicolon at the end. by not having the semicolon makes it an expression and not a statement which let it 
            // return the value
    };
    println!("The number is {}", number);
}
fn test_one() {
    println!("test_one");
}   
fn add_number(x: i32, y: i32)  {
    println!("the sum {} + {} is = {}", x,y, x + y);    
}
// fn add_numbers(x: i32, y: i32) -> i32 {
//     x + y
//     // you can also return the value by using return keyword but it is not recommended in rust also dont need to use semicolon
//     // return statement is the last line of the function 
//     }
fn add_numbers(x: i32, y: i32) -> i32 {
    let result =  x + y;
    if result > 10 {
        return result - 10;
    }
    result
    }
    // you can also return the value by using return keyword but it is not recommended in rust also dont need to use semicolon
    // return statement is the last line of the function 

