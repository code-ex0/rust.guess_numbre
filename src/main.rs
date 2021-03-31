#![feature(allocator_api)]
#[warn(dead_code)]

mod lib;

extern crate savefile;
extern crate serde_derive;
#[macro_use]
extern crate savefile_derive;

use rand::prelude::*;
use savefile::prelude::*;
use serde_derive::{Deserialize, Serialize};
use savefile::load_file;
use std::path::Path;
use std::panic::resume_unwind;
use std::alloc::Global;

#[derive(Debug, Default, Clone, Serialize, Deserialize, Savefile)]
struct Number {
    number_to_find: i32,
    limit_upper: i32,
    limit_lower: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Savefile)]
struct Historic {
    won: bool,
    attempts: i32,
    proposition: Vec<i32>,
    game_duration: i64,
    number: Number,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Savefile)]
struct Statistic {
    all_games: i32,
    won_games: i32,
    historic: Vec<Historic>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Savefile)]
struct Player {
    id: i32,
    user_name: String,
    age: i32,
    statistic: Statistic,
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
            limit_lower: 0,
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
            println!("║ is upper")
        } else if user_number > self.get_number_to_find() {
            println!("║ is lower")
        } else if user_number == self.get_number_to_find() {
            result = true;
        }
        result
    }
}

impl Statistic {
    fn get_all_game(&self) -> i32 {
        self.all_games
    }
    fn set_all_game(&mut self, all_game: i32) {
        self.all_games = all_game
    }
    fn get_won_game(&self) -> i32 {
        self.won_games
    }
    fn set_won_game(&mut self, won_game: i32) {
        self.won_games = won_game
    }
    fn get_historic(&self) -> &Vec<Historic> {
        &self.historic
    }
}

impl Player {
    fn new() -> Player {
        Player {
            id: 0,
            user_name: lib::input("║ What is your user name : "),
            age: lib::ask_number("║ what is your age : "),
            statistic: Default::default(),
        }
    }
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_user_name(&self) -> &str {
        &self.user_name
    }

    fn set_user_name(&mut self, user_name: String) {
        self.user_name = user_name;
    }
    fn get_age(&self) -> i32 {
        self.age
    }
    fn set_age(&mut self, age: i32) {
        self.age = age
    }

    fn get_statistic(&self) -> &Statistic {
        &self.statistic
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
    fn get_player(&mut self, id: usize) -> &Player {
        &mut self.players[id]
    }
    fn get_players(&self) -> &Vec<Player> {
        &self.players
    }
}

fn load_game() -> Result<Game, SavefileError> {
    load_file("save-game.bin", 0)
}

fn start_game(player: &mut Player) {
    let mut number = Number::new();
    number.set_number_to_find();
    let mut proposition = vec![];
    let mut attempts = 1;
    let time = lib::get_time_ms();

    for i in 1..11 {
        println!("{0}", design(3));
        let mut num: i32;
        loop {
            num = lib::ask_number("║ enter a new number : ");
            if num != -1 {
                break;
            }
        }
        &proposition.push(num);
        attempts = i;
        if number.compare_number(num) == true {
            break;
        }
    }

    let won: bool = proposition.last().unwrap() == &number.get_number_to_find();
    if won == true {
        println!("║ you won in {} try", attempts);
        player.statistic.won_games += 1;
    } else {
        println!("║ sorry it's lose")
    }
    player.statistic.all_games += 1;
    player.statistic.historic.push(
        Historic {
            won,
            attempts,
            proposition,
            game_duration: lib::get_time_ms() - time,
            number,
        }
    );
}

fn design(id: i32) -> &'static str {
    match id {
        1 => {
            "╔═════════════════════════════════════════════════
║ Welcome in my video games
╠═════════════════════════════════════════════════
║   1. -new user
║   2. -change user
║   3. -start new game
║   4. -statistic
║  /q. -exit program
╠═════════════════════════════════════════════════"
        }
        2 => {
            "╔═════════════════════════════════════════════════"
        }
        3 => {
            "╠═════════════════════════════════════════════════"
        }
        4 => {
            "╚═════════════════════════════════════════════════"
        }
        _ => {
            ""
        }
    }
}

fn menu(game: &mut Game) {
    println!("{0}", design(1));
    loop {
        match lib::input("║ choose a number for make action : ").as_str() {
            "1" => {
                game.new_player(Player::new());
                for i in &game.players {
                    println!("║ {0}-user name - {1}", i.get_id(), i.get_user_name())
                }
            }
            "2" => {}
            "3" => {
                start_game(&mut game.get_player(0).clone())
            }
            "4" => {
                if !&game.players.is_empty() {
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
                        println!("║- {} -- {} -- {} -- {} -- {} --", i.user_name, i.statistic.all_games, i.statistic.won_games, i.statistic.won_games as f32 / i.statistic.all_games as f32, average_try as f32 / x as f32)
                    }
                } else {
                    println!("║ no player has been created")
                }
            }
            "/q" => {
                break;
            }
            _ => {}
        }
        println!("{0}", design(3));
    }
    println!("{0}", design(4));
}

fn main() {
    let mut game = Game::new();
    menu(&mut game);
    game.save_game();
}
