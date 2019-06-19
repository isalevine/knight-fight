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
        critical: 50,
    };

    let mut lizardman = Character {
        name: String::from("Lizardman"),
        hp: 8,
        attack: String::from("Chomp"),
        damage: 3,
        evade: 30,
        critical: 5,
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
    critical: u32,
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

    fn take_damage(&mut self, character: &Character, dmg_multiplier: i32) {
        println!("{} takes {} damage from {}.", self.name, character.damage * dmg_multiplier, character.name);
        if self.hp - character.damage < 0 {
            self.hp = 0
        } else {
            self.hp -= character.damage * dmg_multiplier
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
        let roll = roll_attack(&player, &enemy);

        // add in 'if' branch for critical hits 
        // (range w/ min: 100 - critical, max: 100)
        let critical_min = 100 - player.critical;
        if roll >= critical_min {
            println!("Critical hit by {}! Double damage!!", player.name);
            enemy.take_damage(&player, 2);
            enemy.print_character_hp();
        } else if roll > enemy.evade {
            println!("{} hits!", player.name);
            enemy.take_damage(&player, 1);
            enemy.print_character_hp();
        } else {
            println!("{} misses!", player.name);
        };

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

// practice using an Enum to return either a "hit" or "miss"
pub fn roll_attack(player: &Character, enemy: &Character) -> u32 {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(1, 101);
    println!("roll is: {}", roll);
    return roll;
}


