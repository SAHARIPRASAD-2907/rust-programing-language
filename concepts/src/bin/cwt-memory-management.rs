enum Light{
    Bright,
    Dull,
}
fn display_light(light:&Light){
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}
// the process of moving
// fn main(){
//     let dull = Light::Dull;
//     display_light(dull);
//     display_light(dull);
// }

// the process of borrowing 
fn main(){
    let dull = Light::Dull;
    display_light(&dull);
    display_light(&dull);
}