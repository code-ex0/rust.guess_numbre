#![feature(allocator_api)]

extern crate savefile;
extern crate serde_derive;
#[macro_use]
extern crate savefile_derive;

use std::io;
use std::io::{stdin, Write};
use rand::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use savefile::prelude::*;
use serde_derive::{Deserialize, Serialize};
use savefile::load_file;
use std::path::Path;


#[derive(Debug, Default, Clone, Serialize, Deserialize, Savefile)]
struct Number {
    number_to_find: i32,
    limit_upper :i32,
    limit_lower : i32
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Savefile)]
struct Historic {
    won: bool,
    attempts: i32,
    proposition: Vec<i32>,
    game_duration: i64,
    number: Number
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Savefile)]
struct Statistic {
    all_games: i32,
    won_games: i32,
    historic: Vec<Historic>
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Savefile)]
struct Player {
    id: i32,
    user_name: String,
    age: i32,
    statistic: Statistic
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Savefile)]
struct Game {
    players: Vec<Player>
}


impl Number {
    fn new() -> Number {
        Number {
            number_to_find: 0,
            limit_upper: 1000,
            limit_lower: 0
        }
    }

    pub fn get_number_to_find(&self) -> i32 {
        self.number_to_find
    }

    fn set_number_to_find(&mut self) {
        let mut rng = rand::thread_rng();
        let num: i32 = rng.gen_range(self.limit_lower..self.limit_upper);
        self.number_to_find = num;
    }

    pub fn get_limit_upper(&self) -> i32 {
        self.limit_upper
    }

    fn set_limit_upper(&mut self, n: i32) {
        self.limit_upper = n;
    }

    pub fn get_limit_lower(&self) -> i32 {
        self.limit_lower
    }

    fn set_limit_lower(&mut self, n: i32) {
        self.limit_lower = n;
    }

    fn compare_number(&self, user_number: i32) -> bool {
        let mut result: bool = false;
        if user_number < self.get_number_to_find() {
            println!("is upper")
        } else if user_number > self.get_number_to_find() {
            println!("is lower")
        } else if user_number == self.get_number_to_find() {
            result = true;
        }
        result
    }
}

impl Player {
    fn new() -> Player {
        Player {
            id: 0,
            user_name: input("What is your user name : "),
            age: ask_number("what is your age : "),
            statistic: Default::default()
        }
    }
}

impl Game {
    fn new() -> Game {
        if Path::new("save-game.bin").exists() {
            let game: Result<Game, SavefileError> = load_game();
            game.unwrap()
        } else {
            Game {
                players: Default::default()
            }
        }
    }
    fn new_player(&mut self, player: Player) {
        self.players.push(player);
    }
    fn save_game(&self) {
        save_file("save-game.bin", 0, self).unwrap();
    }
}

fn load_game() -> Result<Game, SavefileError> {
    load_file("save-game.bin", 0)
}

fn start_game(player: &mut Player) {
    let mut number = Number::new();
    number.set_number_to_find();
    //println!("number tu find is : {}",number.get_number_to_find());
    let mut proposition= vec![];
    let mut won: bool = false;
    let mut attempts = 1;
    let time = get_time_ms();

    for i in 1..11 {
        let mut num: i32;
        loop {
            num = ask_number("enter a new number : ");
            if num != -1 {
                break
            }
        }
        &proposition.push(num);
        attempts = i;
        if number.compare_number(num) == true {
            won = true;
            break
        }

    }

    if won == true {
        println!("you won in {} try", attempts);
        player.statistic.won_games += 1;
    } else {
        println!("sorry it's lose")
    }
    player.statistic.all_games += 1;
    player.statistic.historic.push(
        Historic{
            won,
            attempts,
            proposition,
            game_duration: get_time_ms() - time,
            number,
        }
    );
}

fn get_time_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

fn input(question: &str) -> String {
    let mut input_line = String::new();
    print!("{}",question);
    io::stdout().flush();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line.trim().to_string()

}

fn ask_number (question: &str) -> i32 {
    let mut input = String::new();
    print!("{}", question);
    io::stdout().flush();
    stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

fn design(id: i32) -> &'static str {
    match id {
        1 => {
"╔═══════════════════════════════════════╗
║ Welcome in my video games             ║
╠═══════════════════════════════════════╣
║   1. -new user                        ║
║   2. -start new game                  ║
║   3. -statistic                       ║
║  /q. -exit program                    ║
╚═══════════════════════════════════════╝"
        }
        _ => {
            ""
        }
    }
}

fn menu(game: &mut Game) {
    loop {
        println!("{0}", design(1));
        match input("choose a number for make action : ").as_str() {
            "1" => {
                game.new_player(Player::new());
                for i in &game.players {
                    println!("{0}-user name - {1}",i.id , i.user_name)
                }
            }
            "2" => {
                start_game(&mut game.players[0])
            }
            "3" => {
                for i in &game.players {
                    let mut x: i32 = 0;
                    let mut average_try: i32 = 0;
                    let mut proposition = vec![];
                    for j in &i.statistic.historic {
                        x += 1;
                        average_try += j.attempts;
                        for k in &j.proposition {
                            proposition.push(k)
                        }
                    };
                    println!("-- {} -- {} -- {} -- {} -- {} --", i.user_name, i.statistic.all_games, i.statistic.won_games, i.statistic.won_games as f32 / i.statistic.all_games as f32, average_try as f32 / x as f32, )
                }
            }
            "/q" => {
                break;
            }
            _ => {

            }
        }
    }
}

fn main(){
    let mut game = Game::new();
    menu(&mut game);
    game.save_game();
}
