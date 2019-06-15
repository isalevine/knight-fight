fn main() {

    struct Character {
        name: String,
        hp: u32,
        attack: String,
        damage: u32,
    };

    let knight = Character {
        name: String::from("Knight"),
        hp: 10,
        attack: String::from("Swing Sword"),
        damage: 5,
    };

    print_character(knight);
    take_three_damage(knight);

    fn print_character(character: Character) {
        println!("{}'s HP is currently {}.", character.name, character.hp);
        println!("{}'s attack is {}, and it does {} damage.", character.name, character.attack, character.damage);
    }

    fn take_three_damage(character: Character) -> Character {
        println!("{} takes 3 damage!", character.name)
        character.hp -= 3
    }
}


