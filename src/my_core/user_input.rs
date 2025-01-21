use std::io;

/*
    Retorna o caractere que foi digitado pelo usuário;
    São feitos os devidos tratamentos e que foram 
    cabíveis para a situação.
*/
pub fn user_input_char() -> char {
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
pub fn user_input_final() -> u8 {
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
