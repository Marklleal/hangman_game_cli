use super::user_input;
use std::io;

pub enum EndState {
    OutOfAttempts, 
    PlayerWon,
    GameNotEnded,
}

impl EndState {
    // faz a checagem da jogada e seu estado atual
    pub fn check_game_end_state(rem_att: u8, current_attempt: &str) -> EndState {
        // caso o jogador tenha gastado todas as usas tentativas disponíveis
        if rem_att == 0 { 
            return EndState::OutOfAttempts;
        } 
        // caso não contenha mais underscores, ou seja, 
        // a palavra tenha sido adivinhada
        else if !current_attempt.contains("_") { 
            return EndState::PlayerWon;
        } 
        // jogo continua normalmente...
        else { 
            return EndState::GameNotEnded;
        }
    }
}

/*
    Quantidade de tentativas que ainda estão disponíveis,
    sempre que a função é chamada, ela decrementa o valor
    natural até que chegue em zero, caso ocorra;
    Retorna um valor u8 natural para que o valor original
    seja atualizado toda vez que a função for executada;
*/
pub fn remaining_attempts(mut rem_att: u8) -> u8 {
    rem_att -= 1;
    println!("Tentativas restantes: {rem_att}");
    rem_att
}

// Pergunta ao usuário se é de seu desejo o rerun do programa
pub fn do_rerun() -> bool {
    let mut input = String::new();

    println!("Deseja jogar novamente?(S/N)");
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler linha!");

    // retorna um boolean que dita o reinicio do programa caso seja true
    if input.trim().eq_ignore_ascii_case("s") {
        true
    } else {
        false
    }
}

// Chamado para caso o usuário acerte a palavra
pub fn final_choices(word: &str, tot_attempts: u32, char_attempts: Vec<char>, attempts: Vec<String>) {
    let choice = user_input::user_input_final();
    
    match choice {
        1 => println!("Palavra sorteada: {word}"),
        2 => println!("Total de tentativas: {tot_attempts}"),
        3 => println!("Letras digitadas: {:?}", char_attempts),
        4 => println!("Etapas de descoberta({}): {:?}", attempts.len(), attempts),
        5 => {
            println!("Palavra sorteada: {word}");
            println!("Total de tentativas: {tot_attempts}");
            println!("Letras digitadas: {:?}", char_attempts);
            println!("Etapas de descoberta({}): {:?}", attempts.len(), attempts);
        }
        6 => {
            println!("Saindo...");
            std::process::exit(1);
        }
        _ => {
            println!("Você deveria ter escolhido um número de 1 a 6!");
            do_rerun();
        }
    }
}