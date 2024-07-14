use std::io;

fn soma(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    let mut input1 = String::new();
    println!("Digite um número.");
    io::stdin().read_line(&mut input1).expect("Error reading console");
    let number1: i32 = input1.trim().parse().expect("PRECISA SER UM NÚMERO!");
    
    let mut input2 = String::new();
    println!("Digite outro número.");
    io::stdin().read_line(&mut input2).expect("Error reading console");
    let number2: i32 = input2.trim().parse().expect("PRECISA SER UM NÚMERO!");
    
    let result = soma(number1, number2);
    println!("{number1} + {number2} = {result}")
}