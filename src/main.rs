use std::io;


fn main(){

    let mut name = String::new();
    println!("What is your name? ");
    io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();
    println!("Hello {}!", name);


}