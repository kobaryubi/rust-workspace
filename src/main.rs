use std::io;

fn main() {
    // Hello World
    println!("Hello World!");
    println!("I'm a Rustacean!");

    // Guessing Game
    println!("Guess the number!");
    println!("Please input your guess.");

    // Processing a Guess
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    {
        let x = 5;
        let y = 10;

        println!("x = {x} and y + 2 = {}", y + 2);
    }

    {
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
    }

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing
    {
        let x = 5;
        let x = x + 1;

        {
            let x = x + 2;
            println!("The value of x in the inner scope is: {x}");
        }

        println!("The value of x is: {x}");
    }
}
