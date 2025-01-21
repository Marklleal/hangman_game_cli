pub mod game_state {
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
}