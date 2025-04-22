// use std::any::type_name;

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

    let a : [i32;5] = [1,2,3,4,5];
    print!("a = {:?}\n" , a);
    
    
    let b: [char; 4] = [char::from('b'); 4];
    print!("b = {:?}\n", b);
    

    let slice: &[i32] = &a[1..4];
    print!("slice = {:?}\n", slice);


    let sum: i32 = slice.iter().sum();
    print!("Sum of slice elements = {}\n", sum);



}
