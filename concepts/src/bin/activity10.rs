// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_message(gt_100:bool){
    match gt_100{
        true => println!("its big"),
        false => println!("its small")
    }
    println!();
}

fn main() {
    let value = 101;
    let is_get_100 = value >100;
    print_message(is_get_100);
}