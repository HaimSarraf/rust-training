use std::io;

fn main() {
    println!("Please Enter a Number :");

    let mut _n: i32;
    let mut fibo: i32 = 1;
    let mut a: i32 = 1;
    let mut b: i32 = 1;
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let _n: i32 = input.trim().parse().expect("Please type a number!");

    if _n == 0 {
        println!("Fibonacci of 0 is 0");
    } else if _n == 1 || _n == 2 {
        println!("Fibonacci of {} is 1", _n);
    } else {
        for _ in 3..=_n {
            fibo = a + b;
            a = b;
            b = fibo;
        }
        println!("Fibonacci of {} is {}", _n, fibo);
    }
}
