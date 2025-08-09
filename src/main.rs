use std::io;


fn main(){

    let mut name = String::new();
    println!("What is your name? ");
    io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();
    println!("Hello {}!", name);


    
    let mut num1 = String::new();
    let mut num2 = String::new();


    



    println!("Enter num1 > ");
    io::stdin().read_line(&mut num1).unwrap();

    println!("Enter num2 > ");
    io::stdin().read_line(&mut num2).unwrap();

    // 8 bits
    let num1: i8 = num1.trim().parse().expect("msg");
    let num2: i8 = num2.trim().parse().expect("msg");

    let mut output = num1 + num2;

    println!("{} + {} = {}", num1, num2, output);


}
