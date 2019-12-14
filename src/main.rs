use rand::Rng;
use std::io;

enum Mode {
    Game,
    Config,
    Error,
}

enum Clue {
    NumberCorrect,
    PositionCorrect,
    Nothing,
}

struct GameConfig {
    max_tries: u32,
    code_length: u32,
    code_numbers: [String; 6],
}

fn main() {
    let mut conf = GameConfig {
        max_tries: 12,
        code_length: 4,
        code_numbers: [
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
            String::from("6"),
        ],
    };

    let mode = show_main_menu();
    match mode {
        Mode::Game => show_game(&conf),
        Mode::Config => show_config(),
        _ => (),
    }
}

fn show_main_menu() -> Mode {
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

    if usrinput.trim().eq("1") {
        Mode::Game
    } else if usrinput.trim().eq("2") {
        Mode::Config
    } else {
        Mode::Error
    }
}

fn show_game(config: &GameConfig) {
    println!("generating code....");
    let code = generate_code(config);
    /* if you need debug test code
    let code = [
        String::from("4"),
        String::from("6"),
        String::from("4"),
        String::from("6"),
    ];
    */
    let mut tries = 0;
    println!("Guess the code!");
    loop {
        let mut usrinput = String::new();
        io::stdin()
            .read_line(&mut usrinput)
            .expect("Failed to read line");
        if check_usrinput_valid(&usrinput, &config) {
            tries += 1;
            let clues = check_usrinput_correct(&usrinput, &code);
            let mut clues_output = String::new();
            for clue in clues.iter() {
                match clue {
                    Clue::NumberCorrect => clues_output += "0",
                    Clue::PositionCorrect => clues_output += "X",
                    _ => (),
                }
            }
            if check_game_won(&clues) {
                println!("You´re Winner");
                break;
            } else {
                println!(
                    "Clue: {} (0 means correct number, X means correct number on right position)",
                    clues_output
                );
            }
        } else {
            println!("Invalid input, type again!");
        }

        if tries >= config.max_tries {
            println!("A Loser is you");
            break;
        } else {
            let tries_left = config.max_tries - tries;
            if tries_left == 1 {
                println!("\n\nYour last try!");
            } else {
                println!("\n\nYou still have: {} tries! Keep trying!", tries_left);
            }
        }
    }
}

fn show_config() {}

fn generate_code(config: &GameConfig) -> [String; 4] {
    let mut code_nums = [String::new(), String::new(), String::new(), String::new()];
    let mut counter = 0;
    while counter < config.code_length as usize {
        let secret_number = rand::thread_rng().gen_range(1, 7).to_string();
        code_nums[counter] = secret_number;
        counter += 1;
    }

    code_nums
}

fn check_usrinput_valid(usrinput: &String, config: &GameConfig) -> bool {
    if usrinput.trim().len() != config.code_length as usize {
        return false;
    } else {
        for (i, single_digit) in usrinput.trim().chars().enumerate() {
            let mut was_allowed = false;
            for allowed_digit in config.code_numbers.iter() {
                if single_digit.to_string().eq(allowed_digit) {
                    was_allowed = true;
                    break;
                }
            }
            if !was_allowed {
                return false;
            }
        }

        return true;
    }
}

fn check_usrinput_correct(usrinput: &String, code: &[String; 4]) -> [Clue; 4] {
    let mut clues = [Clue::Nothing, Clue::Nothing, Clue::Nothing, Clue::Nothing];
    // TODO: hier gibts noch Bugs, z.B. beim code 4646 ist der output bei user input 4646: 00XX, es müsste aber XXXX sein!
    for (i, digit) in code.iter().enumerate() {
        let mut found_clue = false;
        for (j, single_digit) in usrinput.trim().chars().enumerate() {
            if single_digit.to_string().eq(digit) && j == i {
                clues[i] = Clue::PositionCorrect;
                found_clue = true;
                break;
            }
        }
        if !found_clue {
            for (j, single_digit) in usrinput.trim().chars().enumerate() {
                if single_digit.to_string().eq(digit) {
                    clues[i] = Clue::NumberCorrect;
                    break;
                }
            }
        }
    }
    // TODO: randomize order
    return clues;
}

fn check_game_won(clues: &[Clue; 4]) -> bool {
    for clue in clues.iter() {
        match clue {
            Clue::Nothing => return false,
            Clue::NumberCorrect => return false,
            Clue::PositionCorrect => (),
        }
    }
    return true;
}
