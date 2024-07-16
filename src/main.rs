use std::{io, process::exit};

fn main() {
    println!(
        "
        \n+------------------------+ \
        \n|      Jogo da Velha     | \
        \n+------------------------+ \
        \nDigite o nome ou o número  \
        \nda opção desejada.         \
        \n[1] * começar *            
        "
    );
    
    let turno = "X"; // TEMPORÁRIO: PADRÃO ALEATÓRIO OU ESCOLHIDO PELO JOGADOR
    
    loop {
        let mut start_selection = String::new();
        io::stdin()
            .read_line(&mut start_selection)
            .expect("Erro na leitura da linha.");
        
        let start_selection = start_selection.trim();
        
        match start_selection {
            "começar"       | "1" => jogo(turno),
            _ => println!("Opção inválida, tente novamente.")
        }
    }
}

fn jogo(mut turno: &str) {
    println!("\n\n\n");
    let mut max = 0;
    while max < 9 {
        max += 1; println!("\nTurno {turno} ({max}/9)");
        
        // CRIAR: LOOP{
        //          EXIBIR ÚLTIMO ESTADO DO MAPA.
        //          LER LINHA - ESPERAR POR COORDENADA
        //          COORDENADA VÁLIDA? {
        //              APLICAR {TURNO} NA COORDENADA
        //          }
        //        }
        
        verificar(turno);
        match turno {
            "X" => turno = "O",
            "O" => turno = "X",
            _ => {println!("PLAYER INVÁLIDO"); exit(2)}
        }
    }
    println!("Empate!")
}

fn verificar(turno: &str) {
    let win = false; // TEMPORÁRIO: EXISTE ALGUMA LINHA NO TABULEIRO?
    
    if  win == true {
        println!("Vitória de {turno}!");
        exit(0);
    }
}