// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

fn main() {
    match_color(Color::Blue)
}

fn match_color(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::Black => println!("Black"),
        Color::White => println!("White"),
    }
}


enum Color {
    Red,
    Green,
    Blue,
    Black,
    White,
}