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

enum Flavour {
    Sparkling,
    Sweet,
    Fruity
}

struct Drink {
    flavor: Flavour,
    fluid_oz:f64,
}

fn print_drink(drink:Drink){
    match drink.flavor{
        Flavour::Sparkling => println!("flavor: sparkling"),
        Flavour::Sweet => println!("flavor: sweet"),
        Flavour::Fruity => println!("flavor: fruity")
    }
    println!("oz: {:?}",drink.fluid_oz);
}

fn main() {
    let sweet = Drink{
        flavor:Flavour::Sweet,
        fluid_oz:6.0
    };
    print_drink(sweet);
}