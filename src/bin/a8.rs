// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Cola,
    Cider,
    Lemon,
    Orange,
    Pineapple,
}

struct Drink {
    flavor: Flavor,
    ounce: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Cola => println!("{}, {}", "Flavor is Cola", drink.ounce),
        Flavor::Cider => println!("{}, {}", "Flavor is Cider", drink.ounce),
        Flavor::Orange => println!("{}, {}", "Flavor is Orange", drink.ounce),
        Flavor::Pineapple => println!("{}, {}", "Flavor is Pineapple", drink.ounce),
        Flavor::Lemon => println!("{}, {}", "Flavor is Lemon", drink.ounce),
        _ => println!("{}, {}", "Flavor is Unknown", drink.ounce)
    }
}

fn main() {
    let drink = Drink { flavor: Flavor::Cola, ounce: 750.00 };
    print_drink(drink)
}
