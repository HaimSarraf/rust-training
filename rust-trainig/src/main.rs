// use std::io;
// use std::any::type_name;
// use text_io::read;

// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }

fn main() {
    //* Variables in Rust

    { /*
        

        let mut x = 5;

        print!("The Value of x = {}\n", x);

        x = 10;

        print!("The Value of x = {x}\n");

        const HOUR_IN_SECONDS: u32 = 60 * 60;

        print!("1 Hour = {} Seconds\n", HOUR_IN_SECONDS);

        let y: i32 = 5;

        {
        let y: i32 = y + 1;

        print!("Y inside the brackets : {y}\n");
        }
        print!("Y Outside the brackets : {y}\n");

        let spaces = "     ";
        print!("spaces is a : {}\n", type_of(spaces));

        let spaces = spaces.len();
        print!("spaces is a : {}\n", type_of(spaces));

        let t: bool = true;
        print!("type of 't' is  : {}\n", type_of(t));

        let f: bool = false;
        print!("type of 'f' is  : {}\n", type_of(f));

        let z: char = 'Z';
        print!("type of 'z' is  : {}\n", type_of(z));

        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (a, b, c) = tup;
        let a_1 = tup.0;
        let b_1 = tup.1;
        let c_1 = tup.2;
        print!("a = {} , b = {} , c = {}\n", a, b, c);
        print!("a_1 = {} , b_1 = {} , c_1 = {}\n", a_1, b_1, c_1);
         */
    }

    //* Arrays in Rust

    { /*
        let a: [i32; 5] = [10, 20, 30, 40, 50];
        print!("a = {:?}\n", a);

        let b: [char; 4] = [char::from('b'); 4];
        print!("b = {:?}\n", b);

        let slice: &[i32] = &a[1..4];
        print!("slice = {:?}\n", slice);

        let sum: i32 = slice.iter().sum();
        print!("Sum of slice elements = {}\n", sum);

        let mut index = String::new();

        io::stdin()
        .read_line(&mut index)
        .expect("Failed to read the Line");

        let index: usize = index.trim().parse().expect("Index Entered is not a number");

        let element = a[index-1];

        println!("The valu
        e of the element at index {index} is : {element}");
         */
    }

    //* Function in Rust
    //* Statements/Expressions

    { /*
        print_labeled_measurement(32, "kg");

        let y = {
        let x = 5;
        x + 1 //awareness!! : this line is an Expression & hasn't semicolon {;}  !!
        };
        println!("The value of y = {y}");

        let five_function_indicator = five();
        println!("The value of indicator is :{five_function_indicator}")
         */
    }

    //* IF-expression

    { /* 
        println!("Please Enter a Number :");

        let x: i64 = read!();

        if x > 0 {
        println!("Positive\n")
        } else if x < 0 {
        println!("Negative\n")
        } else {
        println!("Zero\n")
        }

        let y:i32 = if x > 0 { 1 } else if x < 0 { -1 } else { 0 };

        println!("The value of y = {y}");
         */
    }

    //* Loops & Loop_Labels

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The Result is : {result}\n");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}\n");
        let mut remaining = 10;

        loop {
            println!("remaining ={remaining}");
            if remaining == 9  {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End Count = {count}");
}

// fn print_labeled_measurement(value: i32, unit_label: &str) {
//    println!("The measurement is: {value}{unit_label}\n");
// }

// fn five() -> i32 {
//     5 // that equals :  return 5;
// }
