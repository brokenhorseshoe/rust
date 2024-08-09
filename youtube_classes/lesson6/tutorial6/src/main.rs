fn main() {


    // let x: u8 = 9; // u8 is an unsigned 8 bit integer 0-255
    // let y: i8 = 10; // i8 is a signed 8 bit integer -128-127

    // another way to declare data type
    let b = 255.0f32; // f32 is a 32 bit floating point number
    let c = 10.0f32; // f32 is a 32 bit floating point number
// OR
    let d = 127.0_f32; // f32 is a 32 bit floating point number
    // you can also use underscocres to make the number more readable
    // let d = 127_000i64; // i64 is a signed 64 bit integer -9223372036854775808-9223372036854775807

    let e = 10.2_f32; // f32 is a 32 bit floating point number
    let f = 10.2 as f64; // you can also use the as keyword to convert the number to a different data type
    // use can use that as a implicit conversion as well


    //let z = x + y; // no implementation for `u8 + i8`
    let x = 127_000i64; // i64 is a signed 64 bit integer -9223372036854775808-9223372036854775807
    let y = 10_i64; // i64 is a signed 64 bit integer -9223372036854775808-9223372036854775807
    let z = x / (y as i64); // you can use the as keyword to convert the number to a different data type
    println!("The value of z is: {}", z);

    // this will give you an overflow with out being able to catch it
    let r = (i32::MAX as i64) + 1;
    let q = 10_i32;
    let t = r as i32 / q;
    println!("The value of r is: {}", t);
    // we dont want that to happen so convert smaller to larger and not the other way around.



    println!("END OF LINE");
}
