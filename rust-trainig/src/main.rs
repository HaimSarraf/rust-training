fn main() {
    let s1 = gives_ownership();
    println!("s1 : {}", s1);

    let s2 = String::from("hello");
    println!("s2 : {}", s2);

    let s3 = take_and_give_back(s2);
    println!("s3 : {}", s3);

    let (s4, len) = calculate_length(s3);
    println!("s4 : {}, length: {}", s4, len);
}

fn gives_ownership() -> String {
    let some_string: String = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned & moves out to the calling function
}
// gives_ownership will move its return value into the function that calls it

fn take_and_give_back(input: String) -> String {    // 'input' comes into scope

    input // 'input' is returned & moves out to the calling function

}

fn calculate_length(input: String) -> (String, usize) {
    let length = input.len();
    (input, length)
    
}