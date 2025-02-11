pub mod my_core;

pub use my_core::{words, EndState};

use std::io;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};

/*
    Retorna o caractere que foi digitado pelo usuário;
    São feitos os devidos tratamentos e que foram 
    cabíveis para a situação.
*/
fn user_input_char() -> char {
    loop {
        let mut input = String::new();

        println!("Digite apenas uma letra: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler linha!");
    
        // verifica se a entrada do usuário possui apenas um caractere
        if input.trim().chars().count() > 1 { 
            println!("É preciso que seja inserido apenas um caractere!");
            // solicita o retorno do usuário
            continue; 
        } 

        // converte a string para char
        let input_char: char = match input.trim().chars().next() {
            Some(c) => c,

            // HACK: Precisa-se lidar com o erro de uma melhor forma
            None => {
                println!("Erro não tratado!");
                std::process::exit(1);
            }
        };

        if !input_char.is_alphabetic() {
            println!("Números não são permitidos!");
            continue;
        }

        return input_char.to_ascii_lowercase();
    }
}

/*
    Dá ao usuário opções de exibição após a 
    execução ocorrer corretamente;
    Retorna um valor u8, que designa a opção 
    de exibição escolhida
*/
fn user_input_final() -> u8 {
    let mut input = String::new();

    println!(
        "Escolha uma das opções de exibição:\n
        1- Palavra sorteada\n
        2- Número de tentativas\n
        3- Letras inseridas\n
        4- Etapas de descoberta\n
        5- Todas acima\n
        6- Sair"
    );

    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler linha!");

    let choice: u8 = match input.trim().parse() {
        Ok(n) if n > 0 && n < 7 => n,
        _ => {
            println!("Nenhum número de 1 a 6 foi escolhido!");
            0
        }
    };
    choice
}

// Chamado para caso o usuário acerte a palavra
fn final_choices(word: &str, tot_attempts: u32, char_attempts: Vec<char>, attempts: Vec<String>) {
    let choice = user_input_final();
    
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

/*
    Quantidade de tentativas que ainda estão disponíveis,
    sempre que a função é chamada, ela decrementa o valor
    natural até que chegue em zero, caso ocorra;
    Retorna um valor u8 natural para que o valor original
    seja atualizado toda vez que a função for executada;
*/
fn remaining_attempts(mut rem_att: u8) -> u8 {
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

        let input_char = user_input_char();
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
            rem_att = remaining_attempts(rem_att);
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
                final_choices(word, tot_attempts, char_attempts, attempts);
                break;
            }
            EndState::GameNotEnded => {
                continue;
            }
        }
    }
}