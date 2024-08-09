fn main() {
    //primitive data types
    let x: i32 = 4; // 32-bit integer default value in Rust
    let y: u32 = 15; // unsigned 32-bit integer no negative values
    // the following is all the data types in Rust
    let a: i8 = 0; // 8-bit integer
    let b: i16 = 0; // 16-bit integer
    let c: i32 = 0; // 32-bit integer
    let d: i64 = 0; // 64-bit integer
    let e: i128 = 0; // 128-bit integer
    let f: u8 = 0; // unsigned 8-bit integer
    let g: u16 = 0; // unsigned 16-bit integer
    let h: u32 = 0; // unsigned 32-bit integer
    let i: u64 = 0; // unsigned 64-bit integer
    let j: u128 = 0; // unsigned 128-bit integer
    let k: f32 = 0.0; // 32-bit float
    let l: f64 = 0.0; // 64-bit float
    let m: bool = true; // boolean
    let n: char = 'a'; // character
    let o: &str = "Hello, World!"; // string
    const SECONDS_IN_MINTUE: u32 = 60; // unsigned 32-bit integer a const you can't change
    println!("Value of x is: {}", x);
    println!("Value of y is: {}", y);
    println!("Value of a is: {}", a);
    println!("Value of b is: {}", b);
    println!("Value of c is: {}", c);
    println!("Value of d is: {}", d);
    println!("Value of e is: {}", e);
    println!("Value of f is: {}", f);
    println!("Value of g is: {}", g);
    println!("Value of h is: {}", h);
    println!("Value of i is: {}", i);
    println!("Value of j is: {}", j);
    println!("Value of k is: {}", k);
    println!("Value of l is: {}", l);
    println!("Value of m is: {}", m);
    println!("Value of n is: {}", n);
    println!("Value of o is: {}", o);
    println!("Value of SECONDS_IN_MINTUE is: {}", SECONDS_IN_MINTUE);
    // with u8 0 - 2^8-1 = 0 - 255
    // with i8 -2^7 - 2^7-1 = -128 - 127
    // bianry
    let floating_point = 2.0; // f64 which is the default
    let true_flause = true; // bool
    let character = '😻'; // char
    //compound data types

   // Correct tuple definition
   let tup: (i32, f64, i32) = (1, 6.4, 1); // tuple
   let tup2: (i32, bool, char) = (1, true, '😻'); // tuple
   let mut tup3: (i32, f64, i32) = (1, 6.4, 1); // tuple
   tup3.0 = 10;


   // Accessing tuple elements
   println!("The value of tup3.0 is: {}", tup3.0); // debug

   // Destructuring the tuple
   let (x, y, z) = tup;
   println!("The value of x is: {}", x);
   println!("The value of y is: {}", y);
   println!("The value of z is: {}", z);
   println!("The value of tup3.0 is: {}", tup3.0);

   let arr = [1, 2, 3, 4, 5]; // array
   println!("The value of arr[0] is: {}", arr[0]);
   println!("arr[0] = 10; // error: cannot assign to immutable indexed content but you can do this with mut");
   let mut arr_mut = [1, 2, 3, 4, 5]; // array
    arr_mut[0] = 10;
    println!("The value of arr_mut[0] is: {}", arr_mut[0]);
    let mut arr3: [i32; 15] = [0; 15]; // array with 15 elements all initialized to 0
    println!("The value of arr3[0] is: {}", arr3[0]);

    // you have to be careful with the the type 
    let x : u8 = 4;
    // let y: u32 = x; // error: mismatched types
    println!("The value of y is: {}", y);


   









}
