use std::io::stdin;

mod fizzbuzz;

fn main() {
    println!("input number");

    let mut input: String = String::new();

    stdin().read_line(&mut input).expect("failed");

    let num: i32 = input.trim().parse().unwrap();
    println!("{}", fizzbuzz::get_fizzbuzz(num));
}
