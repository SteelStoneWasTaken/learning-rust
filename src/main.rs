use std::{io, process::exit};

static  INVALID_OP: &str= "OPÇÃO INVÁLIDA.";
static  ERR_COORD1: &str= "COORDENADA OCUPADA.";
static  TURNO: &str = "X";


fn main() {
    println!(
        "
        \n+------------------------+ \
        \n|      Jogo da Velha     | \
        \n+------------------------+ \
        \nDigite o nome ou o número  \
        \nda opção desejada.         \
        \n                           \
        \n1 - Começar.               \
        \n2 - Configurações. (temp.) \
        \n3 - Sair.
        "
    );

    //PADRÃO ALEATÓRIO OU ESCOLHIDO PELO JOGADOR (menu: configurações)
    
    loop {
        let mut start_selection = String::new();
        io::stdin()
            .read_line(&mut start_selection)
            .expect("{INVALID_OP}");
        
        let start_selection = start_selection.trim().to_uppercase();
        
        match start_selection.as_str() {
            "COMEÇAR" | "1" => jogo(TURNO),
            "SAIR"    | "3" => exit(0),
            _ => println!("{INVALID_OP}")
        }
    }
}



fn jogo(mut turno: &str) {
    let mut map = [
        " "," "," ",
        " "," "," ",
        " "," "," "];
    
    let mut max = 0;
    while max < 9 {   
        max += 1;
        
        loop {  
            println!("\nTurno {turno} ({max}/9)");
            println!("\
                \n  | 1 | 2 | 3    \
                \n--+---+---+---   \
                \nA | {} | {} | {} \
                \n--+---+---+---   \
                \nB | {} | {} | {} \
                \n--+---+---+---   \
                \nC | {} | {} | {} 
                ",map[0],map[1],map[2],map[3],map[4],map[5],map[6],map[7],map[8]);
            
            let mut coord = String::new();
            io::stdin()
                .read_line(&mut coord)
                .expect("{INVALID_OP}");
            let coord = coord.trim().to_uppercase();
            
            match coord.as_str() { 
                "A1" => {if map[0] == " " {map[0] = turno; break;} println!("{ERR_COORD1}")},
                "A2" => {if map[1] == " " {map[1] = turno; break;} println!("{ERR_COORD1}")},
                "A3" => {if map[2] == " " {map[2] = turno; break;} println!("{ERR_COORD1}")},
                "B1" => {if map[3] == " " {map[3] = turno; break;} println!("{ERR_COORD1}")},
                "B2" => {if map[4] == " " {map[4] = turno; break;} println!("{ERR_COORD1}")},
                "B3" => {if map[5] == " " {map[5] = turno; break;} println!("{ERR_COORD1}")},
                "C1" => {if map[6] == " " {map[6] = turno; break;} println!("{ERR_COORD1}")},
                "C2" => {if map[7] == " " {map[7] = turno; break;} println!("{ERR_COORD1}")},
                "C3" => {if map[8] == " " {map[8] = turno; break;} println!("{ERR_COORD1}")},
                "SAIR" => exit(0),
                "VOLTAR" | "MENU" => main(),
                _ => println!("{INVALID_OP}")
            }
        } 
        verificar(turno, map);
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



fn verificar(turno: &str, map: [&str; 9]) {
    if 
    map[0] == map[1] && map[1] == map[2] && map[0] != " " || // Primeira linha completa.
    map[3] == map[4] && map[4] == map[5] && map[3] != " " || // Segunda linha completa.
    map[6] == map[7] && map[7] == map[8] && map[6] != " " || // Terceira linha completa
    map[0] == map[3] && map[3] == map[6] && map[0] != " " || // Primeira coluna completa.
    map[1] == map[4] && map[4] == map[7] && map[1] != " " || // Segunda coluna completa.
    map[2] == map[5] && map[5] == map[8] && map[2] != " " || // Terceira coluna completa.
    map[0] == map[4] && map[4] == map[8] && map[0] != " " || // Diagonal \ completa
    map[2] == map[4] && map[4] == map[6] && map[2] != " "    // Diagonal / completa.
    {println!("
            \n+-----------------+   \
            \n|    | 1 | 2 | 3  |   \
            \n+ ---+---+---+--- +   \
            \n|  A | {} | {} | {}  |\
            \n+ ---+---+---+--- +   \
            \n|  B | {} | {} | {}  |\
            \n+ ---+---+---+--- +   \
            \n|  C | {} | {} | {}  |\
            \n+-----------------+\
            \n|  Vitória de {turno}!  |\
            \n+-----------------+
            ",map[0],map[1],map[2],map[3],map[4],map[5],map[6],map[7],map[8]);
    exit(0);}
}