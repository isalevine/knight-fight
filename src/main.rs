extern crate rand;

use std::io; 
use std::process;
use rand::Rng;


fn main() {

    // refactor or move all attacks to a separate tuple/array, and make it globally accessible
    // for instantiating knight and lizardman? (what is most efficient way to do that?)
    let mut swing_sword = Attack {
        name: String::from("Swing Sword"),
        damage: 5,
    };

    let mut lizardman_chomp = Attack {
        name: String::from("Chomp"),
        damage: 3,
    };

    // consider refactoring attacks to be a tuple/array of multiple options in each Character?

    // critical temp. set to 50 for testing - default = 10 ??
    let mut knight = Character {
        name: String::from("Knight"),
        hp: 10,
        attack: swing_sword,
        // attack: String::from("Swing Sword"),
        // damage: 5,
        evade: 50,
        defend: false,
        defend_bonus: 30,
        critical: 10,
    };

    let mut lizardman = Character {
        name: String::from("Lizardman"),
        hp: 8,
        attack: lizardman_chomp,
        // attack: String::from("Chomp"),
        // damage: 3,
        evade: 30,
        defend: false,
        defend_bonus: 20,
        critical: 5,
    };

    welcome_menu();

    main_turn_loop(knight, lizardman);

    // next functions:
    // if-statement for hp 0 or below => dead
    // menu!!
    // randomizer??
    
}



pub struct Character {
    name: String,
    hp: i32,
    attack: Attack,
    // attack: String,
    // damage: i32,
    evade: u32,
    defend: bool,
    defend_bonus: u32,
    critical: u32,
}

impl Character {
    fn print_character_hp(&self) {
        println!("{}'s HP is currently {}.", self.name, self.hp);
        if self.hp <= 0 {
            // NOT a fan of how this string line-spacing is written here...find a more readable way?
            println!("
{} is dead!"
            , self.name);

            // temp - ADD LOSE CONDITION, but not here? (main_loop/menu??)
            println!("You win!! (Exiting program now.)");
            process::exit(0);
        };
    }

    fn print_character_attack(&self) {
        println!("{}'s attack is {}, and it does {} damage.", self.name, self.attack.name, self.attack.damage);
    }

    fn take_damage(&mut self, character: &Character, dmg_multiplier: i32) {
        println!("{} takes {} damage from {}.", self.name, character.attack.damage * dmg_multiplier, character.name);
        if self.hp - character.attack.damage < 0 {
            self.hp = 0
        } else {
            self.hp -= character.attack.damage * dmg_multiplier
        }
    }
}


pub struct Attack {
    name: String,
    damage: i32,
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

    // tuples implemented to practice using/returning them and ownership--
    // see notes below re: calling enemy_turn inside player_menu...
    let mut tuple = player_menu(player, enemy);

    tuple = enemy_turn(tuple.0, tuple.1);

    main_turn_loop(tuple.0, tuple.1);
}


pub fn player_menu(mut player: Character, mut enemy: Character) -> (Character, Character) {
    println!("
Your turn! Please select an option:
1) Attack
2) Defend
3) Retreat
0) Quit");

    let input = get_input();

    if input == "1" || input == "attack" || input == "Attack" || input == "a" {
        println!("Player attacks with {}!", player.attack.name);
        let roll = roll_attack(&player, &enemy);
        let mut evade = enemy.evade;
        
        if enemy.defend == true {
            evade += enemy.defend_bonus;
            enemy.defend = false;
        };

        // add in 'if' branch for critical hits 
        // (range w/ min: 100 - critical, max: 100)
        let critical_min = 100 - player.critical;
        if roll >= critical_min {
            println!("Critical hit by {}! Double damage!!", player.name);
            enemy.take_damage(&player, 2);
            enemy.print_character_hp();
        } else if roll > evade {
            println!("{} hits!", player.name);
            enemy.take_damage(&player, 1);
            enemy.print_character_hp();
        } else {
            println!("{} misses!", player.name);
        };

    } else if input == "2" || input == "defend" || input == "Defend" || input == "d" {
        println!("Player defends!");
        player.defend = true;

    } else if input == "3" || input == "retreat" || input == "Retreat" || input == "r" {
        println!("Player retreats!");
        let retreat = (player.evade * 2) - enemy.evade;

    } else if input == "0" || input == "quit" || input == "Quit" || input == "q" {
        println!("Exiting program. Goodbye!");
        std::process::exit(0);
    } else {
        println!("Invalid input!");
    };

    // lazy formatting - extra space at tend of each turn (better way?)
    println!("");

    // consider calling enemy_turn here and passing ownership instead of return?
    return (player, enemy);

    // player_menu(player, enemy);

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
    println!("(roll is: {})", roll);
    return roll;
}


pub fn enemy_turn(mut player: Character, mut enemy: Character) -> (Character, Character) {
    let mut rng = rand::thread_rng();
    let action = rng.gen_range(1, 11);
    println!("(enemy action is {})", action);

    // action 1 & 2 copied from Player function above, 
    // refactor both as one Attack and one Defend function
    if action > 3 {    
        println!("{} attacks with {}!", enemy.name, enemy.attack.name);
        let roll = roll_attack(&enemy, &player);
        let mut evade = player.evade;
        
        if player.defend == true {
            evade += player.defend_bonus;
            player.defend = false;
        };

        // add in 'if' branch for critical hits 
        // (range w/ min: 100 - critical, max: 100)
        let critical_min = 100 - enemy.critical;
        if roll >= critical_min {
            println!("Critical hit by {}! Double damage!!", enemy.name);
            player.take_damage(&enemy, 2);
            player.print_character_hp();
        } else if roll > evade {
            println!("{} hits!", enemy.name);
            player.take_damage(&enemy, 1);
            // player.print_character_hp();
        } else {
            println!("{} misses!", enemy.name);
        };
    } else if action <= 3 {
        println!("Enemy defends!");
        enemy.defend = true;
    };

    // lazy formatting - extra space at tend of each turn (better way?)
    println!("");

    // consider calling enemy_turn here and passing ownership instead of return?
    return (player, enemy);
}


pub fn welcome_menu() {
    println!("
Welcome to KNIGHT FIGHT!
=========================
You are a knight fighting
a lizardman. Select your
actions, and don't die!
Good luck!!
=========================
    ")
}