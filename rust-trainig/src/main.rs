fn main() {
    
    //? introduction of the Ownership in Rust ?//

    {
        //* "s" is  valid just in this scope
        //* after we do some stuff with it and 
        //* exit the scope, it would be no longer valid
    

        let mut s1: String = String::from("hello");

        s1.push_str(" world !");

        let s2 = s1;
        
        // println!("{s1}"); 
        //* to prevent 'double free' error,
        //* Rust consider s1 is no longer valid
        
        println!("{s2}");

    } //* after this curly bracket,Rust calls
      //* the \drop\ function

}
