fn main() {
    // comparing two numbers
    // in rust logical are && (and), || (or), ! (not)
    // preffernce with logical operators is !, &&, ||
    let cond = (2 as f32) <= 2.2;
    println!("cond: {}", cond);

    let food = "cookie";

    if food == "cookie" {
        println!("I love cookies");
    } else if food == "cake" {
        println!("I love cake");
    } else {
        println!("I love nothing");
    }

}
