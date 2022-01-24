use std::io;
use rand::Rng;

#[allow(non_snake_case)]
fn isNumberToLowOrHigh(number: i32) -> bool {
    number < 0 || number > 5
}

#[allow(non_snake_case)]
fn randomNumber() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, 5)
}

fn main() {
    println!("Please enter a number between 0 and 5");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: i32 = input.trim().parse().expect("Please enter a number (floats do not count)");

    if isNumberToLowOrHigh(number) {
        println!("Please enter a number between 0 and 5");
        return;
    };

    let random_number = randomNumber();

    if number == random_number {
        println!("You guessed the correct number!");
    } else {
        println!("You guessed the wrong number, the number was {}", random_number);
    };
}
