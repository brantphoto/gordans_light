extern crate rand;
use rand::distributions::{IndependentSample, Range};

struct Character {
    hp: u16,
    armor: u16,
    name: String,
}

fn main() {
    let mut vec: Vec<Character> = Vec::with_capacity(10);
    for _ in 0..vec.capacity() {
        let character = build_character();
        vec.push(character);
    }
    for character in &vec {
        println!("Character {} has been created with {} hp and {} armor.", character.name, character.hp, character.armor)
    }
}

fn build_character() -> Character {
    let step = Range::new(1, 50);
    let mut rng = rand::thread_rng();
    let hp = step.ind_sample(&mut rng);
    let armor = step.ind_sample(&mut rng);

    Character {
        hp,
        armor,
        name: String::from("Gary"),
    }
}
