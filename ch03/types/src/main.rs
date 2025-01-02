fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup; // "Destructuring" aka unpacking
    println!("y is {y}");

    let five_hundred = tup.1; // tuple indexing
    println!("Tuple index at 1: {five_hundred}");

}
