use std::{io, path::PrefixComponent};
/*
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

*/
fn main(){
    // main menu
    let title = "\
        - - -| Jogo da Velha |- - -\
     \nSelecione o player inicial:\
     \n          X ou O
        ";
    println!("{title}");
    
    let mut start = String::new();
    io::stdin().read_line(&mut start).expect("Error reading console");
    
    if start.trim() != "X" && start.trim() != "O" {
        println!("Selecione X ou O");
        return;
    }
    game(&start);
}

fn game(player: &str) {
    let a1 = "."; let a2 = "."; let a3 = ".";
    let b1 = "."; let b2 = "."; let b3 = ".";
    let c1 = "."; let c2 = "."; let c3 = ".";
    
    let mut turno = player.trim();
    
    let mut count = 0;
    while count < 5 {
        count += 1;
        
        if turno == "X" {
            println!("Turno {turno}");
            
            turno = "O";
        }
        if turno == "O" {
            println!("Turno {turno}");
            
            turno = "X";
        }
        //error
        if count == 5 {
            println!("Limite de ciclos maximos excedido!")
        }
    }
    
    
    /*
    println!("\
        \n *---+---+---+---*    \
        \n |   | A | B | C |    \
        \n +---+---+---+---+    \
        \n | 1 | {} | {} | {} | \
        \n +---+---+---+---+    \
        \n | 2 | {} | {} | {} | \
        \n +---+---+---+---+    \
        \n | 3 | {} | {} | {} | \
        \n *---+---+---+---*    \
        ",a1,a2,a3,b1,b2,b3,c1,c2,c3);
        */
}