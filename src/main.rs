// Jogo da forca, CLI

use hangman_game::words;

use std::io;
use crossterm::{
    execute,
    terminal::{
        Clear, 
        ClearType
    },
};

// Pergunta ao usuário se é de seu desejo o rerun do programa
fn do_rerun() -> bool {
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

fn main() {
    let words = words::get_words();

    loop {
        match hangman_game::random_word(&words) {
            Some(word) => {
                hangman_game::execute_all(word);
                let rerun_bool = do_rerun();
                if !rerun_bool {
                    println!("Saindo...");
                    break;
                } else {
                    execute!(io::stdout(), Clear(ClearType::All)).unwrap(); // limpa o terminal
                    continue;
                }
            }
            _ => {
                println!("Houve um erro imprevisto no funcionamento do código, algo foi alterado!");
            }
        }
    }
}
