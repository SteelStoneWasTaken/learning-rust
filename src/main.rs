use std::{io, process::exit};

static INVALID_OP: &str= "OPÇÃO INVÁLIDA.";
static ERR_COORD1: &str= "COORDENADA OCUPADA.";

fn main() {
    println!(
        "
        \n+------------------------+ \
        \n|      Jogo da Velha     | \
        \n+------------------------+ \
        \nDigite o nome ou o número  \
        \nda opção desejada.         \
        \n                           \
        \n1 - Começar                \
        \n2 - Sair
        "
    );
    
    let turno = "X"; // TEMPORÁRIO: PADRÃO ALEATÓRIO OU ESCOLHIDO PELO JOGADOR (menu: opções)
    
    loop {
        let mut start_selection = String::new();
        io::stdin()
            .read_line(&mut start_selection)
            .expect("{INVALID_OP}");
        
        let start_selection = start_selection.trim().to_uppercase();
        
        match start_selection.as_str() {
            "COMEÇAR" | "1" =>{println!("\n\n\n"); jogo(turno);},
            "SAIR"    | "2" => exit(0),
            _ => println!("{INVALID_OP}")
        }
    }
}

fn jogo(mut turno: &str) {
    // Tabuleiro:
    let mut a1 = " "; let mut a2 = " "; let mut a3 = " ";
    let mut b1 = " "; let mut b2 = " "; let mut b3 = " ";
    let mut c1 = " "; let mut c2 = " "; let mut c3 = " ";
    // Ciclo do jogo:
    let mut max = 0;
    while max < 9 {   
        max += 1;
        // Turno do jogador:
        loop {  
            println!("\nTurno {turno} ({max}/9)");
            println!(
                "\n  | 1 | 2 | 3    \
                \n--+---+---+---   \
                \nA | {} | {} | {} \
                \n--+---+---+---   \
                \nB | {} | {} | {} \
                \n--+---+---+---   \
                \nC | {} | {} | {} 
                ",a1,a2,a3,b1,b2,b3,c1,c2,c3);
            
            let mut coord = String::new();
            io::stdin()
                .read_line(&mut coord)
                .expect("{INVALID_OP}");
            let coord = coord.trim().to_uppercase();
            
            match coord.as_str() { 
                "A1" => {if a1 == " " {a1 = turno; break;} println!("{ERR_COORD1}")},
                "A2" => {if a2 == " " {a2 = turno; break;} println!("{ERR_COORD1}")},
                "A3" => {if a3 == " " {a3 = turno; break;} println!("{ERR_COORD1}")},
                "B1" => {if b1 == " " {b1 = turno; break;} println!("{ERR_COORD1}")},
                "B2" => {if b2 == " " {b2 = turno; break;} println!("{ERR_COORD1}")},
                "B3" => {if b3 == " " {b3 = turno; break;} println!("{ERR_COORD1}")},
                "C1" => {if c1 == " " {c1 = turno; break;} println!("{ERR_COORD1}")},
                "C2" => {if c2 == " " {c2 = turno; break;} println!("{ERR_COORD1}")},
                "C3" => {if c3 == " " {c3 = turno; break;} println!("{ERR_COORD1}")},
                "SAIR" => exit(0),
                "VOLTAR" | "MENU" => main(),
                _ => println!("{INVALID_OP}")
            }
        }
        
        verificar(turno);
        match turno {
            "X" => turno = "O",
            "O" => turno = "X",
            _ => {println!("{INVALID_OP}"); exit(2)}
        }
    }
    println!("
        \n+-----------+\
        \n|  Empate!  |\
        \n+-----------+
        "); exit(0);
}

fn verificar(turno: &str) {
    
    let win = false; // TEMPORÁRIO: EXISTE ALGUMA LINHA NO TABULEIRO?
    
    if  win == true {
        println!("
            \n+-----------------+\
            \n|  Vitória de {turno}!  |\
            \n+-----------------+
            ");
        exit(0);
    }
}