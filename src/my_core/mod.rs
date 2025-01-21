pub mod game_state;
pub mod user_input;
pub mod words;
pub mod game_core;

pub use game_state::{remaining_attempts, EndState};
pub use words::{get_words, random_word};
pub use game_core::execute_all;