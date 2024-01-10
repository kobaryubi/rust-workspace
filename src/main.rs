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
        let a = [1, 2, 3, 4, 5];
        println!("Please enter an array index.");
        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");
        let element = a[index];
        println!("The value of the element at index {index} is: {element}");
    }

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

    // The Tuple Type
    {
        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;
        println!("{x}, {y}, {z}");
        println!("{}, {}, {}", tup.0, tup.1, tup.2);
    }

    another_function(5);
    print_labeled_measurement(5, 'h');

    {
        let y = {
            let x = 3;
            x + 1
        };
        println!("The value of y is: {y}");
    }

    {
        let x = five();
        println!("The value of x is: {x}");
    }

    {
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", s);
    }
}

fn five() -> i32 {
    5
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
