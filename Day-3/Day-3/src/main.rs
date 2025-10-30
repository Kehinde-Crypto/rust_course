// use std::io;

// fn main() {
//     // 1️⃣ Ask the user to input a number
//     println!("Enter a number:");

//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read input");

//     let number: i32 = input
//         .trim()
//         .parse()
//         .expect("Please enter a valid number");

    
//     if number % 2 == 0 {
//         println!("{} is even.", number);
//     } else {
//         println!("{} is odd.", number);
//     }

//     // 3️⃣ Use a `loop` to print numbers from 1 to 5
//     println!("\nPrinting numbers from 1 to 5:");
//     let mut count = 1;
//     loop {
//         println!("{}", count);
//         if count == 5 {
//             break;
//         }
//         count += 1;
//     }

//     // 4️⃣ Use a `match` statement for days of the week
//     println!("\nEnter a day of the week (e.g., Monday, Friday):");

//     let mut day = String::new();
//     io::stdin()
//         .read_line(&mut day)
//         .expect("Failed to read day");

//     let day = day.trim(); // remove newline

//     match day {
//         "Monday" => println!("Start of the week!"),
//         "Tuesday" => println!("Second day of the week."),
//         "Wednesday" => println!("Midweek already!"),
//         "Thursday" => println!("Almost Friday!"),
//         "Friday" => println!("Weekend is coming!"),
//         "Saturday" => println!("Enjoy your weekend!"),
//         "Sunday" => println!("Rest and recharge!"),
//         _ => println!("That's not a valid day of the week."),
//     }
// }

fn main(){
    // Create a program that calculates the factorial of a given number using a `while` loop.
    // factorial is the a value that is been reduced to the last value using the multipliacion value so what should we do
    // step: 1 assuming is 3 so it will be 3 * 2 * 1 

    // factorial
   fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let result = factorial(5);
    println!("Factorial of 5 is: {}", result);
}
// factorial

      for num in 1..50{
      if num % 2 == 0{
        println!(num);
        
      }
      }
}

// 1. Create a program that calculates the factorial of a given number using a `while` loop.
// 2. Write a program that simulates a countdown timer using a `loop` and breaks when the countdown reaches zero.
// 3. Use the `for` loop to calculate the sum of even numbers from 1 to 50.
// 4. Write a program that reads a string input and uses the `match` statement to respond with different outputs based on the input (e.g., "hello" => "Hi there!", "bye" => "Goodbye!", etc.).
// 5. Implement a program that uses `if` statements inside a `for` loop to print all the odd numbers from 1 to 20.
// 6. Create a small game where the program generates a random number between 1 and 10, and the user has to guess it. Use a `loop` to keep asking until the user gets it right.