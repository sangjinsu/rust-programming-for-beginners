// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn make_coordinate(x: f64, y: f64) -> (f64, f64) {
    (x, y)
}

fn main() {
    let coord = make_coordinate(1.0, 2.0);
    match coord {
        (_, y) if y == 5.0 => println!("{} is equal to 5", y),
        (_, y) if y > 5.0 => println!("{} is greater than 5", y),
        (_, y) if y < 5.0 => println!("{} is less than 5", y),
        _ => println!("{:?} is not a valid coordinate", coord),
    }
}
