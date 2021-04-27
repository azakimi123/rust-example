use std::io;

fn main() {
    //mut allows variables to be mutated and changed ex. let mut mars_weight
    let mut input = String::new();
    //& sign is for passing references
    //$mut is for mutable references
    io::stdin().read_line(&mut input);

    borrow_string(&input);
    own_string(input);
    // println!("Input {}", input); 
    // let mars_weight = calculate_weight_on_mars(input);
    // println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight/9.81) * 3.71
}

fn borrow_string(s:&String){
    println!("{}", s)
}

fn own_string(s:String){
    println!("{}", s)
}

//ownership
//1. Each value in Rust is owned by a variable.
//2. When the owner goes out of scope, the value will be deallocated.
//3. There can only be one owner at a time