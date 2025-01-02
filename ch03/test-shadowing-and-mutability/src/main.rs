fn main() {
    println!("Hello, world!");
    let x = 1;
    println!("Outer: {x}"); // 1
{
        let x = 2;
        println!("Shadowed inner: {x}"); // 2
    }
    println!("Outer after inner shadow: {x}"); // 1

    let x = 3;
    println!("Outer shadow: {x}"); // 3
}
