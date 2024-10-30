pub fn init_ownership() {
    println!("Ownership concept in Strings:");
}

// Demonstration on Ownership and Borrowing in the rust programming language.
fn main() {
    println!("Ownership demonstration in strings\n");
    let str1 = String::from("Sandeep K. Dasari!");
    let str2 = str1;
    // println!("{}", str1);
    println!("{}", str2);
}