fn main(){
  //   Write a program that checks if a number is even or odd using the `if` statement.
//2. Create a `while` loop that prints numbers from 1 to 10.
//3. Use the `for` loop to iterate over an array of your favorite colors and print each one.
//4. Create a simple calculator using the `match` statement that performs addition, subtraction, multiplication, or division based on user input.
// 5. Write a program that continuously takes user input until the word "exit" is typed, using a `loop`.
  

   let mut count = 0;
   loop{
    if count % 2 == 0{
        println!("{} is even", count);
    } else {
        println!("{} is odd", count);
    }
    count += 1; 
       if count > 10 {
    break;
   }
}
}

fn main() {
    for count in 0..=10 {
        if count % 2 == 0 {
            println!("{} is even", count);
        } else {
            println!("{} is odd", count);
        }
    }
}
