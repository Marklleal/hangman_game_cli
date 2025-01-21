// Jogo da forca, CLI

use hangman_game::{game_core, my_core::game_state, words};

use std::io;
use crossterm::{
    execute,
    terminal::{
        Clear, 
        ClearType
    },
};

fn main() {
    let words = words::get_words();

    loop {
        match words::random_word(&words) {
            Some(word) => {
                game_core::execute_all(word);
                let rerun_bool = game_state::do_rerun();
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
