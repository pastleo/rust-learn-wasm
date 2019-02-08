use std::io;
use std::io::Write;

pub fn guessing_game() {
    println!(">> rs_book::ch2: guessing_game");
    let answer:u16 = rand::random::<u16>() % 100 + 1;
    let mut guessing:u16 = 0;
    let mut hint_min:u16 = 1;
    let mut hint_max:u16 = 100;

    while answer != guessing {
        print_hint(hint_min, hint_max);
        guessing = get_guess_number();
        if guessing > answer && guessing < hint_max {
            hint_max = guessing;
        } else if guessing < answer && guessing > hint_min {
            hint_min = guessing;
        }
    }

    println!("GG! answer: {}", answer);
}

fn get_guess_number() -> u16 {
    let mut guessing = String::new();
    match io::stdin().read_line(&mut guessing) {
        Ok(_) => {}
        Err(_) => {
            println!("cannot readline!");
            return 0
        }
    }
    match guessing.trim().parse::<u16>() {
        Ok(result) => { return result; }
        Err(_) => {
            println!("cannot parse, please input a number!");
            return 0
        }
    }
}

fn print_hint(hint_min: u16, hint_max: u16) {
    print!("please input your guess: ({} ~ {}) ", hint_min, hint_max);
    io::stdout().flush().unwrap();
}
