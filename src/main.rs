use crate::garden::vegetables::Asparagus;
use std::io;

pub mod garden;

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

    {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {count}");

        let mut counter = 0;
        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };
        println!("The result is {result}");
    }

    {
        let s = String::from("hello");
        takes_ownership(s);

        let x = 5;
        makes_copy(x);
    }

    {
        let a = [10, 20, 30, 40, 50];

        for element in a {
            println!("the value is: {element}");
        }

        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }

    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        print!("The length of '{}' is {}.", s1, len)
    }

    {
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }

    println!("======== Convert temperatures between Fahrenheit and Celsius. ========");
    {
        let mut fahrenheit = 32.0;
        let celsius = convert_to_celsius(fahrenheit);
        println!("{celsius}");

        fahrenheit = convert_to_fahrenheit(celsius);
        println!("{fahrenheit}");
    }

    println!("======== Rectangle ========");
    {
        let scale = 2;
        let rect = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };
        println!("rect is {:#?}", rect);
        dbg!(&rect);

        println!(
            "The area of the rectangle is {} square pixels.",
            rect.area()
        );

        let square = Rectangle::square(3);
        println!("square is {:#?}", square);

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }

    println!("======== Defining Modules to Control Scope and Privacy ========");
    {
        let plant = Asparagus {};
        println!("I'm growing {:?}!", plant);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn convert_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn convert_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
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
