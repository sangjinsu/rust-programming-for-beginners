// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    println!("{}", sum(1, 2));
}

fn sum(a: i64, b: i64) -> i64 {
    a + b
}
