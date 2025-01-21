use super::{user_input, game_state, EndState};
use std::io;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};

/*
    Recebe como argumento no parâmetro um &str,
    que designa a palavra sorteada;
    O principal funcionamento do código está nessa 
    função.
*/
pub fn execute_all(word: &str) {
    let mut attempts: Vec<String> = Vec::new();
    let mut current_attempt = String::new();
    let mut tot_attempts: u32 = 0;
    let mut char_attempts: Vec<char> = Vec::new();
    let mut rem_att: u8 = 9;

    // current_attempt é preenchido com '_' de acordo com o tamanho de 'word'
    for _ in 0..word.chars().count() {
        current_attempt.push('_');
    }

    let init_text = String::from("Bem vindo ao jogo da forca!\nPara jogar, tente descobrir qual a palavra secreta por trás da sua atual jogada;\nDigite apenas uma letra por vez!");
    
    println!("{init_text}");

    loop {
        println!("Tentativa atual: {}", current_attempt);
        let mut new_attempt = String::new();

        let input_char = user_input::user_input_char();
        if char_attempts.contains(&input_char) {
            println!("Você já usou a letra '{input_char}' anteriormente, tente usar outra ;)");
            tot_attempts += 1;
            continue;
        }

        for (index, value) in current_attempt.chars().enumerate() {
            let word_char = word.chars().nth(index).unwrap();
            // caso o valor atual seja '_'
            if value == '_' {
                // se o caracter de mesmo índice da palavra sorteada 
                // é igual ao caracter inserido pelo jogador
                if word_char == input_char {
                    new_attempt.push(input_char);
                } else {
                    new_attempt.push('_');
                }
            } else {
                new_attempt.push(value as char);
            }
        }

        if new_attempt == current_attempt { // caso no momento o usuário não tenha acertado uma letra
            rem_att = game_state::remaining_attempts(rem_att);
        } else { // caso tenha acertado alguma letra da palavra
            attempts.push(new_attempt.clone());
            current_attempt = new_attempt;
        }
        
        tot_attempts += 1;
        char_attempts.push(input_char);

        match EndState::check_game_end_state(rem_att, &current_attempt) {
            EndState::OutOfAttempts => {
                execute!(io::stdout(), Clear(ClearType::All)).unwrap(); // limpa o terminal
                println!("Fim de jogo!!\nA palavra era {word}.\nJogue novamente :)");
                break;
            }
            EndState::PlayerWon => {
                // Chamado para caso o usuário acerte a palavra
                // Dá opções finais após o acerto da palavra
                execute!(io::stdout(), Clear(ClearType::All)).unwrap(); // limpa o terminal
                println!("Parabéns! Você adivinhou a palavra!");
                game_state::final_choices(word, tot_attempts, char_attempts, attempts);
                break;
            }
            EndState::GameNotEnded => {
                continue;
            }
        }
    }
}