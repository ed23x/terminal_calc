use std::io;
use num::complex::Complex;

fn main() {
    println!("Welcome to the advanced calculator!");

    loop {
        println!("Please enter the operation you want to perform:");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Complex Addition");
        println!("6. Complex Multiplication");
        println!("7. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 7 {
            break;
        }

        match choice {
            1 => {
                let (a, b) = get_two_numbers();
                println!("Result: {}", a + b);
            },
            2 => {
                let (a, b) = get_two_numbers();
                println!("Result: {}", a - b);
            },
            3 => {
                let (a, b) = get_two_numbers();
                println!("Result: {}", a * b);
            },
            4 => {
                let (a, b) = get_two_numbers();
                if b != 0.0 {
                    println!("Result: {}", a / b);
                } else {
                    println!("Cannot divide by zero");
                }
            },
            5 => {
                let (a, b) = get_two_complex_numbers();
                println!("Result: {}", a + b);
            },
            6 => {
                let (a, b) = get_two_complex_numbers();
                println!("Result: {}", a * b);
            },
            _ => println!("Invalid choice"),
        }
    }

    println!("Goodbye!");
}

fn get_two_numbers() -> (f64, f64) {
    println!("Enter the first number:");
    let a = get_number();
    println!("Enter the second number:");
    let b = get_number();
    (a, b)
}

fn get_number() -> f64 {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");
    num.trim().parse().expect("Please enter a valid number")
}

fn get_two_complex_numbers() -> (Complex<f64>, Complex<f64>) {
    println!("Enter the real part of the first complex number:");
    let re1 = get_number();
    println!("Enter the imaginary part of the first complex number:");
    let im1 = get_number();

    println!("Enter the real part of the second complex number:");
    let re2 = get_number();
    println!("Enter the imaginary part of the second complex number:");
    let im2 = get_number();

    (Complex::new(re1, im1), Complex::new(re2, im2))
}
