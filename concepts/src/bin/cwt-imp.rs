// object
struct Temperature {
    degree_f:f64
}


//class
impl Temperature {
    // Self is used for creating a new object
    fn freezing()->Self{
        Self{
            degree_f:32.0
        }
    }
    fn boiling() ->Self{
        Self {degree_f:212.0}
    }
    // self is used for using existing object
    fn show_temp(&self){
        println!("{:?} degree F",self.degree_f);
    }  
}

fn main(){
    let hot = Temperature {degree_f:99.9}; // new object with parameters
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();

    let boiling = Temperature::boiling();
    boiling.show_temp()
}