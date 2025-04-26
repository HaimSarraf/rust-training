use std::io;

fn main() {
    println!("Enter a number:");

    let mut _n: i32;        //the input which user enters
    let mut fibo: i32 = 1; //the n'th sentence of fibonacci
    let mut a:i32 = 1;     //counter one
    let mut b:i32 = 1; //counter two

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    _n = input.trim().parse().expect("Please enter a number");


    if _n == 1 || _n == 2 {
        println!("{}", fibo);
    } else {
        for _ in 3..=_n {
            fibo = a + b;
            a = b;
            b = fibo;
        }

        println!("The {}'th term of Fibonacci is: {}", _n, fibo);
    }
}
