// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name:String,
    fav_color:String,
    age:i32
}

fn print(data:&str){
    println!("{:?}",data);
}

fn main() {
    let people = vec![
        Person{
            name:"George".to_owned(),
            fav_color:"green".to_owned(),
            age:7
        },
        Person{
            name: "Anna".to_owned(),
            fav_color:"Purple".to_owned(),
            age:9,
        },
        Person{
            name:"Katie".to_owned(),
            fav_color:"String".to_owned(),
            age:14
        },
    ];
    for person in people{
        if person.age <=10{
            print(&person.name);
            print(&person.fav_color)
        }
    }
}
