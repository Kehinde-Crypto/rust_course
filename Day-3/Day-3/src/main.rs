use std::io;

fn main() {
    // 1️⃣ Ask the user to input a number
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: i32 = input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    
    if number % 2 == 0 {
        println!("{} is even.", number);
    } else {
        println!("{} is odd.", number);
    }

    // 3️⃣ Use a `loop` to print numbers from 1 to 5
    println!("\nPrinting numbers from 1 to 5:");
    let mut count = 1;
    loop {
        println!("{}", count);
        if count == 5 {
            break;
        }
        count += 1;
    }

    // 4️⃣ Use a `match` statement for days of the week
    println!("\nEnter a day of the week (e.g., Monday, Friday):");

    let mut day = String::new();
    io::stdin()
        .read_line(&mut day)
        .expect("Failed to read day");

    let day = day.trim(); // remove newline

    match day {
        "Monday" => println!("Start of the week!"),
        "Tuesday" => println!("Second day of the week."),
        "Wednesday" => println!("Midweek already!"),
        "Thursday" => println!("Almost Friday!"),
        "Friday" => println!("Weekend is coming!"),
        "Saturday" => println!("Enjoy your weekend!"),
        "Sunday" => println!("Rest and recharge!"),
        _ => println!("That's not a valid day of the week."),
    }
}

