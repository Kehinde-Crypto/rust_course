fn main() {

   struct User{
    user_name:String,
    email:String,
    is_active:bool,
}

  let user1 = User{
       user_name: String::from("Rustacean"),
        email: String::from("rust@example.com"),
        is_active: true,
        
  };
    println!("{} is the value",user1.user_name);
}
// fn main(){
//     let user1 = user_creds(String::from("rustacean@rust.com"), String::from("Rustacean"));
//      println!("Email: {}", user1.email);
// }
// fn user_creds(email:String, name:String) -> User{
//    User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// struct AlwaysEqual;
// fn main() {
//     let _subject = AlwaysEqual;
// }

// // using the implement method
// struct Rectangle{
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect = Rectangle { width: 30, height: 50 };
//     println!("Area: {}", rect.area());
// }

// ## 🎯 Hands-On Challenge

// 1. Create a program that defines and uses a struct for a book with fields like `title`, `author`, `pages`, and `publisher`.
// 2. Write functions to calculate and display book details using the struct.

// ## 💻 Exercises - Day 6

// ### ✅ Exercise: Level 1

// 1. Define a struct for a rectangle and implement methods to calculate area and perimeter.
// 2. Create a program to demonstrate the use of struct update syntax.

// ### ✅ Exercise: Level 2

// 1. Write a program that uses a tuple struct to store RGB color values.
// 2. Implement a method on a struct that compares two struct instances for equality.
// 3. Write a function that returns an instance of a unit-like struct.