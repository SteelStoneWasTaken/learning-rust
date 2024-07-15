use std::{io, process::exit};

fn main(){
    let title = "\
        - - -| Jogo da Velha |- - -\
     \nSelecione o player inicial:\
     \n          X ou O
        ";
    println!("{title}");
    
    loop /* Resposta incorreta? Tente novamente.*/{
        let mut sstart = String::new(); // Cria uma variável para armazenar uma String de tamanho desconhecido.
        io::stdin().read_line(&mut sstart).expect("Não pode ler o console."); // Ler a linha e colocar em ss start. Não conseguiu ler? Erro.
        let start = sstart.trim().to_uppercase(); // coloca em start o valor de sstart sem último caractere (enter) e coloca em maiúsculo.
        
        if start == "X" || start == "O" /* Resposta válida: Iniciar o jogo. */ {
            game(&start); // Inicia o jogo.
        } 
        else if start.to_lowercase() == "exit" /* exit para sair. */ {
            exit(1) // Finaliza o programa sem erro.
        }
        
        println!("É aceito apenas 'X' ou 'O'!\
            \n digite 'exit' para sair.");
    }
}

fn game(player: &str) {
    // mapa
    //  1             2             3 
    let a1 = "."; let a2 = "."; let a3 = "."; // A
    let b1 = "."; let b2 = "."; let b3 = "."; // B
    let c1 = "."; let c2 = "."; let c3 = "."; // C
    // de quem é o primeiro turno?
    let mut turno = player.trim();
    
    let mut count = 1;
    while count <= 9 /* Cria 9 turnos enumerados de 1 a 9 */ {
        
        if turno == "X" && count <= 9 /* Turno X --------------------------------------------------- */ {
            println!(" Turno X ({count})"); // Mostra de quem é o turno e o turno atual.
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
                ",a1,a2,a3,b1,b2,b3,c1,c2,c3); // mostra situação atual do jogo.
            
            let mut pos = String::new(); // onde será a posição?
            io::stdin().read_line(&mut pos).expect("Error reading console"); //...
            if pos.trim().to_lowercase() == "exit" {
                exit(1)
            } // exit para sair.
            println!("");
            
            //a cordenada existe? Substituir a cordenada pelo simbolo do jogador.
            
            verify_win(); // Alguém ganhou?
            println!(" Última colocação: {}", pos.trim().to_uppercase()); // finalizando turno.
            turno = "O"; count += 1; //...
        }
        
        if turno == "O" && count <= 9 /* Turno O --------------------------------------------------- */{
            println!(" Turno O ({count})"); // Mostra de quem é o turno e o turno atual.
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
                ",a1,a2,a3,b1,b2,b3,c1,c2,c3);  // mostra situação atual do jogo.
            
            let mut pos = String::new(); // onde será a posição?
            io::stdin().read_line(&mut pos).expect("Error reading console"); //...
            if pos.trim().to_lowercase() == "exit" {
                exit(1)
            }// exit para sair.
            println!("");
            
            //a cordenada existe? Substituir a cordenada pelo simbolo do jogador.
            
            verify_win(); // Alguém ganhou?
            println!("Última colocação: {}", pos.trim().to_uppercase()); // finalizando turno.
            turno = "X"; count += 1; //...
        };

        //--------------------------------------------------------------------------------------------
        
        if count >= 9 {
            println!("EMPATE!") // Ninguém venceu dentro de 9 turnos? Fim. (empate)
        }
    }
}

fn verify_win(){
    let win = false;
    if win {
        println!("placeholder")
    }
}