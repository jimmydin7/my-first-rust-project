use std::io::{self,Write};


fn main(){

    let mut name = String::new();
    println!("---------------------");
    println!("What is your name? ");
    println!("---------------------");
    io::stdin().read_line(&mut name).unwrap();
    println!("---------------------");
    name = name.trim().to_string();
    println!("Hello {}!", name);
    println!("---------------------");


    
    let mut num1 = String::new();
    let mut num2 = String::new();


    



    print!("Enter num1 > ");
    io::stdout().flush();
    io::stdin().read_line(&mut num1).unwrap();

    print!("Enter num2 > ");
    io::stdout().flush();
    io::stdin().read_line(&mut num2).unwrap();

    // 8 bits
    let num1: i8 = num1.trim().parse().expect("msg");
    let num2: i8 = num2.trim().parse().expect("msg");

    let output = num1 + num2;

    println!("{} + {} = {}", num1, num2, output);


}
