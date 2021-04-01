#![feature(allocator_api)]

#[warn(dead_code)]
#[warn(unused_must_use)]
mod lib;

#[macro_use]
extern crate savefile_derive;
extern crate savefile;
extern crate serde_derive;

use rand::prelude::*;
use savefile::prelude::*;
use serde_derive::{Deserialize, Serialize};
use savefile::load_file;
use std::path::Path;

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
    propositions: Vec<i32>,
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
    players: Vec<Player>,
    player_select: i32
}

impl Number {
    fn new() -> Number {
        Number {
            number_to_find: 0,
            limit_upper: 1000,
            limit_lower: 0,
        }
    }
    fn get_number_to_find(&self) -> i32 {
        self.number_to_find
    }
    fn set_number_to_find(&mut self) {
        let mut rng = rand::thread_rng();
        let num: i32 = rng.gen_range(self.limit_lower..self.limit_upper);
        self.number_to_find = num;
    }
    fn get_limit_upper(&self) -> i32 {
        self.limit_upper
    }
    fn set_limit_upper(&mut self, n: i32) {
        self.limit_upper = n;
    }
    fn get_limit_lower(&self) -> i32 {
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

impl Historic {
    fn get_won(&self) -> bool {
        self.won
    }
    fn get_attempts(&self) -> i32 {
        self.attempts
    }
    fn get_propositions(&self) -> &Vec<i32> {
        &self.propositions
    }
    fn get_game_duration(&self) -> i64 {
        self.game_duration
    }
    fn get_number(&self) -> &Number {
        &self.number
    }
    fn convert_game_duration(&self) -> String {
        let minutes = self.game_duration / 1000 / 60 ;
        let seconds = self.game_duration /1000 - minutes * 60;
        let milliseconds = self.game_duration - minutes * 60 * 1000 - seconds * 1000;
        format!("{:0>2}:{:0>2}.{:0>3}", minutes, seconds, milliseconds)
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
        let user_name = lib::input("║ What is your user name : ");
        let age: i32 = loop {
            let input = lib::ask_number("║ what is your age : ");
            if input != -1 {
                break input;
            }
            else {
                println!("║ please enter a right answer");
            }
        };
        Player {id: 0, user_name, age, statistic: Default::default()}
    }
    fn get_id(&self) -> i32 {
        self.id
    }
    fn set_id(&mut self, id: i32) {
        self.id = id
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
            Game {players: Default::default(), player_select: 0 }
        }
    }
    fn new_player(&mut self, player: Player) {
        self.players.push(player);
    }
    fn save_game(&self) {
        save_file("save-game.bin", 0, self).unwrap();
    }
    fn get_player(&mut self, id: i32) -> &mut Player {
        &mut self.players[id as usize]
    }
    fn get_players(&self) -> &Vec<Player> {
        &self.players
    }
    fn get_player_select(&self) -> i32 {
        self.player_select
    }
    fn set_player_select(&mut self, id: i32) {
        self.player_select = id
    }
}

fn load_game() -> Result<Game, SavefileError> {
    load_file("save-game.bin", 0)
}

fn start_game(player: &mut Player) {
    let mut number = Number::new();
    number.set_number_to_find();
    let mut propositions: Vec<i32> = vec![];
    let mut attempts = 1;
    let time = lib::get_time_ms();

    for i in 1..11 {
        println!("{0}", design(3));
        let num: i32 = loop {
            let input = lib::ask_number("║ enter a new number : ");
            if input != -1 {
                break input;
            }
            else {
                println!("║ please enter a right answer");
            }
        };
        &propositions.push(num);
        attempts = i;
        if number.compare_number(num) == true {
            break;
        }
    }
    let won: bool = propositions.last().unwrap() == &number.get_number_to_find();
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
            propositions,
            game_duration: lib::get_time_ms() - time,
            number,
        }
    );
}

fn design(id: i32) -> &'static str {
    match id {
        1 => {
            "║ Welcome in my video games
╠═════════════════════════════════════════════════
║   1. -new user
║   2. -change user
║   3. -start new game
║   4. -statistic
║   5. -game historic
║  /q. -exit program"
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

fn new_player(game: &mut Game) {
    let mut player = Player::new();
    if !&game.players.is_empty() {
        player.set_id(game.get_players().last().unwrap().get_id() + 1);
    }
    game.new_player(player);
    for i in &game.players {
        println!("║ {0}-user name - {1}", i.get_id(), i.get_user_name())
    }
}

fn choose_player(game: &mut Game) {
    println!("{0}", design(3));
    println!("║ switch user");
    for user in game.get_players() {
        println!("║ {} -- {}", user.get_id(), user.get_user_name());
    }
    loop {
        let input = lib::ask_number("║ choose a number for select a user : ");
        if input != -1 {
            if (game.get_players().first().unwrap().get_id()..game.get_players().last().unwrap().get_id() + 1).contains(&input) {
                game.set_player_select(input);
                break
            }
        }
    }
}

fn print_statistics(game: &mut Game) {
    if !&game.get_players().is_empty() {
        for i in game.get_players() {
            let mut x: i32 = 0;
            let mut average_try: i32 = 0;
            let mut propositions: Vec<i32> = vec![];
            for j in i.get_statistic().get_historic() {
                x += 1;
                average_try += j.get_attempts();
                for k in j.get_propositions() {
                    propositions.push(*k)
                }
            };
            println!("║ - {} {} -- {} -- {} -- {} -- {} --",
                     i.get_id(),
                     i.get_user_name(), i.get_statistic().get_all_game(),
                     i.get_statistic().get_won_game(),
                     i.get_statistic().get_won_game() as f32 / i.get_statistic().get_all_game() as f32,
                     average_try as f32 / x as f32)
        }
    } else {
        println!("║ no player has been created")
    }
}

fn print_historic(game: &mut Game) {
    if !&game.get_players().is_empty() {
        for i in game.get_players() {
            println!("║\n╠═══ {}", i.get_user_name());
            for (index, j) in i.get_statistic().get_historic().iter().enumerate() {
                if index != i.get_statistic().get_historic().len() - 1 {
                    println!("║    ╠═══ {}", j.convert_game_duration());
                }
                else {
                    println!("║    ╚═══ {}", j.convert_game_duration());
                }
            }
        }
    }
}

fn menu(game: &mut Game) {
    println!("{0}\n{1}\n{2}", design(2), design(1), design(3));
    loop {
        match lib::input("║ choose a number for make action : ").as_str() {
            "1" => {
                new_player(game);
            }
            "2" => {
                choose_player(game);
            }
            "3" => {
                if !&game.get_players().is_empty() {
                    start_game(game.get_player(game.get_player_select()));
                }
                else {
                    println!("║ not player has been created");
                }
            }
            "4" => {
                print_statistics(game);
            }
            "5" => {
                print_historic(game);
            }
            "/q" => {
                break;
            }
            _ => {
                println!("{0}", design(1));
            }
        }
        game.save_game();
        println!("{0}", design(3));
    }
    println!("{0}", design(4));
}

fn main() {
    let mut game = Game::new();
    menu(&mut game);
    game.save_game();
}






#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test(){
        println!("{number:>width$}", number=100, width=6);
        assert_eq!(0, 0)
    }

}