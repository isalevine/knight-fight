fn main() {

    struct Character {
        name: String,
        hp: i32,
        attack: String,
        damage: i32,
    };

    impl Character {
        fn print_character_hp(&self) {
            println!("{}'s HP is currently {}.", self.name, self.hp);
            if self.hp <= 0 {
                println!("{} is dead!", self.name)
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

    let mut knight = Character {
        name: String::from("Knight"),
        hp: 10,
        attack: String::from("Swing Sword"),
        damage: 5,
    };

    let mut lizardman = Character {
        name: String::from("Lizardman"),
        hp: 8,
        attack: String::from("Chomp"),
        damage: 3,
    };

    knight.print_character_hp();
    lizardman.print_character_attack();
    knight.take_damage(&lizardman);
    knight.print_character_hp();

    lizardman.print_character_hp();
    lizardman.print_character_attack();
    lizardman.take_damage(&knight);
    lizardman.print_character_hp();
    lizardman.take_damage(&knight);
    lizardman.print_character_hp();

    // next functions:
    // if-statement for hp 0 or below => dead
    // menu!!
    // randomizer??
    
}


