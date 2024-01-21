use crate::garden::vegetables::Asparagus;

use rust_workspace::server;
use rust_workspace::temperature::{convert_to_celsius, convert_to_fahrenheit};
use std::io;
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

mod garden;

fn main() {
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

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // The Tuple Type
    {
        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;
        println!("{x}, {y}, {z}");
        println!("{}, {}, {}", tup.0, tup.1, tup.2);
    }

    print_labeled_measurement(5, 'h');

    {
        let y = {
            let x = 3;
            x + 1
        };
        println!("The value of y is: {y}");
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

    println!("======== 16. Fearless Concurrency ========");
    {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    println!("======== 20. Final Project: Building a Multithreaded Web Server ========");
    {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            server::handle_connection(stream);
        }
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

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
