fn main() {
    let n = 30;

    let result = fibonacci(n);
    println!("Fibonacci of {n} is: {result}");

    fn fibonacci(n: i32) -> i32 {
        if n <= 1 {
            n
        } else {
            fibonacci(n-1) + fibonacci(n-2)
        }

    }
}
