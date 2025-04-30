fn main() {
    
    //? introduction of the Ownership in Rust ?//

    {
        //* "s" is  valid just in this scope
        //* after we do some stuff with it and 
        //* exit the scope, it would be no longer valid.
    

        let mut s1: String = String::from("hello");

        println!("s1 before assignment & push_str: {s1}");
        
        s1 = String::from("yoohoo");
        //* when assign a new value to an
        //* existing variable, Rust will free
        //* the original value's memory immediately.
        
        s1.push_str(" world !");


        let s2 = s1.clone();

        println!("s1 still exists : {s1}");
        
        let s3 = s1;

        println!("after 'drop' s1 doesn't last anymore !");

        // println!("{s1}"); 
        //* to prevent 'double free' error,
        //* Rust consider s1 is no longer valid.
        
        println!("s2 (the deep copy of s1 by .clone() method):{s2}");
        println!("s3 (just moved s1 in it):{s3}");

    } //* after this curly bracket,Rust calls the \drop\ function.

    
}
