fn main() {

    struct Character {
        name: String,
        hp: u32,
        attack: String,
        damage: u32,
    };

    impl Character {
        fn print_character_hp(&self) {
            println!("{}'s HP is currently {}.", self.name, self.hp);
        }

        fn print_character_attack(&self) {
            println!("{}'s attack is {}, and it does {} damage.", self.name, self.attack, self.damage);
        }

        fn take_three_damage(&mut self) {
            println!("{} takes 3 damage!", self.name);
            self.hp -= 3
        }
    }

    let mut knight = Character {
        name: String::from("Knight"),
        hp: 10,
        attack: String::from("Swing Sword"),
        damage: 5,
    };

    knight.print_character_hp();
    knight.print_character_attack();
    knight.take_three_damage();
    knight.print_character_hp();
    
}


