fn main() {
   // owership
    let s = String::from("Hello, Rust");

    println!("{}", s);
    // so the new value is s2 not s1

    let s1 = String::from("Rust");
    let s2 = s1; // s1 is now invalid, ownership is moved to s2
    println!("{}", s2);
    

}
// borrowing
fn main(){
     let s = String::from("Hello Rust");
     print_string(&s);// passed as a refence
     println!("{}",s); 
}
fn print_string(s: &String){
      println!("{}",s);
}
// # 🎯 Hands-On Challenge

// Write a program that:

// 1. Demonstrates moving and copying with variables.
// 2. Creates functions that take ownership of their parameters and return a result.
// 3. Uses references to avoid unnecessary data copying.

// ## 💻 Exercises - Day 5

// ### ✅ Exercise: Level 1

// 1. Write a function that takes a string, borrows it, and returns its length.
// 2. Create a program that demonstrates moving ownership between variables.
// 3. Implement a function that accepts a mutable reference and modifies the data.

// ### ✅ Exercise: Level 2

// 1. Write a program that uses mutable references to swap two variables.
// 2. Implement a function to clone a vector without transferring ownership.
// 3. Create a recursive function to find the factorial of a number using borrowed values.
// 4. Write a function that counts the occurrences of a character in a string without taking ownership.
