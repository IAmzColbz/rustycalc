use std::env;
use std::io;
use std::process::Command;

fn input_float() -> f64 {
    loop {
        let mut user_float = String::new();
        io::stdin()
            .read_line(&mut user_float)
            .expect("Failed to read line...");
        // Reads User input

        match user_float.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid Float");
                continue;
                // Makes sure the user input is valid
            }
        };
    }
}

fn input_int() -> usize {
    loop {
        let mut user_int: String = String::new();
        io::stdin()
            .read_line(&mut user_int)
            .expect("Failed to read line...");
        // Reads user input

        match user_int.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid Integer");
                continue;
                // Validation match
            }
        };
    }
}

fn input_operand() -> &'static str {
    loop {
        let mut user_operand: String = String::new();
        io::stdin()
            .read_line(&mut user_operand)
            .expect("Failed to read line...");

        match user_operand.trim() {
            "+" => return "+",
            "-" => return "-",
            "*" => return "*",
            "/" => return "/",
            _ => {
                println!("Invalid Operand");
                continue;
                // Validation match
            }
        };
    }
}

fn clear() {
    if env::consts::OS == "windows" {
        // On Windows, use the "cls" command
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear screen");
    } else {
        // On other platforms, use the "clear" command
        Command::new("clear")
            .status()
            .expect("Failed to clear screen");
    }
}

fn calculate(first_number: f64, second_number: f64, operand: &str) -> f64 {
    // Using the input operand, match to the correct calculation requirement
    match operand {
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        "*" => first_number * second_number,
        "/" => first_number / second_number,
        _ => {
            panic!("No *valid* operand provided.");
        }
    }
}

fn main() {
    clear();
    println!("Welcome to Colby's revised calculator, now in Rust!");
    println!("Please insert the first number you wish to calculate.");
    let first_num: f64 = input_float();
    clear();
    println!("{first_num}  _?_  X");
    println!("Please insert you operand: ( + | - | * | / )");
    let user_op: &str = input_operand();
    clear();
    println!("{first_num}  {user_op}  X");
    println!("Please insert the second number you wish to calculate.");
    let second_num: f64 = input_float();
    clear();
    println!("{first_num}  {user_op}  {second_num}");
    let result: f64 = calculate(first_num, second_num, user_op);
    println!("How many digits would you like to round to?");
    let round: usize = input_int();
    clear();
    println!("{first_num}  {user_op}  {second_num}");
    println!("The answer is {:.1$}.", result, round);
    // Syntactically, {:.1$} just rounds the result to the 'round' integer with $ being the insertion.
}
