extern crate rand;

use std::io; 
use std::process;
use rand::Rng;


fn main() {

    let mut knight = Character {
        name: String::from("Knight"),
        hp: 10,
        attack: String::from("Swing Sword"),
        damage: 5,
        evade: 50,
    };

    let mut lizardman = Character {
        name: String::from("Lizardman"),
        hp: 8,
        attack: String::from("Chomp"),
        damage: 3,
        evade: 20,
    };


    main_turn_loop(knight, lizardman);

    // next functions:
    // if-statement for hp 0 or below => dead
    // menu!!
    // randomizer??
    
}



pub struct Character {
    name: String,
    hp: i32,
    attack: String,
    damage: i32,
    evade: u32,
}

impl Character {
    fn print_character_hp(&self) {
        println!("{}'s HP is currently {}.", self.name, self.hp);
        if self.hp <= 0 {
            println!("{} is dead!", self.name);

            // temp
            println!("You win!! (Exiting program now.)");
            std::process::exit(0);
        };
    }

    fn print_character_attack(&self) {
        println!("{}'s attack is {}, and it does {} damage.", self.name, self.attack, self.damage);
    }

    fn take_damage(&mut self, character: &Character) {
        println!("{} takes {} damage from {}!", self.name, character.damage, character.name);
        if self.hp - character.damage < 0 {
            self.hp = 0
        } else {
            self.hp -= character.damage
        }
    }
}




pub fn main_turn_loop(mut player: Character, mut enemy: Character) {
    player.print_character_hp();
    // enemy.print_character_attack();
    // player.take_damage(&enemy);
    // player.print_character_hp();

    enemy.print_character_hp();
    // enemy.print_character_attack();
    // enemy.take_damage(&player);
    // enemy.print_character_hp();
    // enemy.take_damage(&player);
    // enemy.print_character_hp();

    player_menu(player, enemy);
}

pub fn player_menu(mut player: Character, mut enemy: Character) {
    println!("Your turn! Please select an option:
    1) Attack
    2) Defend
    3) Retreat
    0) Quit");

    let input = get_input();

    if input == "1" || input == "attack" || input == "Attack" || input == "a" {
        println!("Player attacks!");
        enemy.take_damage(&player);
        enemy.print_character_hp();
    } else if input == "2" || input == "defend" || input == "Defend" || input == "d" {
        println!("Player defends!");
    } else if input == "3" || input == "retreat" || input == "Retreat" || input == "r" {
        println!("Player retreats!");
    } else if input == "0" || input == "quit" || input == "Quit" || input == "q" {
        println!("Exiting program. Goodbye!");
        std::process::exit(0);
    } else {
        println!("Invalid input!");
    };

    player_menu(player, enemy);

    // println!("input is: {}", input)
}


pub fn get_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_string();

    return input
}


