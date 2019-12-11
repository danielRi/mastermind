use std::io;
use std::cmp

struct GameConfig {
    max_tries: u32,
    code_length: u32,
    code_numbers: [u32; 6],
}

fn main() {
    let default_conf = GameConfig {
        max_tries: 12,
        code_length: 4,
        code_numbers: [1, 2, 3, 4, 5, 6],
    };

    show_main_menu();
}

fn show_main_menu() {
    println!("MASTER MIND");
    println!("-----------");
    println!("");
    println!("Start Game.........................1");
    println!("Config.............................2");
    println!("");
    println!("Your Input (Number 1 or 2)");

    let mut usrinput = String::new();
    io::stdin()
        .read_line(&mut usrinput)
        .expect("Failed to read line");

    println!("user input was: {}", usrinput);
    // TODO: compare
}
