use std::io;

fn main() {
    let secret_word: String = String::from("ordinateur");
    let mut attempts = secret_word.len() + 3;
    let word_as_table = string_to_table(&secret_word);
    let mut progression = progression_table(&secret_word);
    let mut letter_count = 0;

    while !winning_status(&progression, &word_as_table) && attempts > 0 {
        let user_input = char_input();
        for i in 0..word_as_table.len() {
            if user_input == word_as_table[i]{
                progression[i] = word_as_table[i];
                letter_count += 1;
            }
        }
        println!("{:?}", progression);
        if letter_count > 0 {
            println!("Cette lettre est présente {} fois dans le mot", letter_count);
        } else {
            println!("Cette lettre n’est pas présente dans le mot");
            attempts -= 1;
        }
        println!("Il te reste {} vies", attempts);
        letter_count = 0;
    }
    if attempts > 0 {
        println!("Fin du jeu ! Bien joué");
    }
    else {
        println!("Booo ! T'es mort");
    }
}
fn char_input() -> char {
    println!("=======\nEnter une lettre : \n");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input doesn't work");
    let char_vec: Vec<char> = input.chars().collect();
    let ch = char_vec[0];
    return ch;
}

fn string_to_table(word: &String) -> Vec<char> {
    let mut i = 0;
    let mut word_as_table =  Vec::new();
    for c in word.chars() {
        word_as_table.push(c);
        i = i + 1;
    }
    return word_as_table;
}

fn progression_table(word: &String) -> Vec<char>{
    let mut progression= Vec::new();
    for _i in 0..word.len() {
        progression.push('_');
    }
    return progression;
}

fn winning_status(progression_table : &Vec<char>, secret_word_table : &Vec<char>) -> bool {
    let mut nb_ = 0;
    let mut i = 0;

    while i < secret_word_table.len() && nb_ == 0 {
        if progression_table[i] == '_' {
            nb_ = nb_ + 1;
        }
        i = i + 1;
    }
    return nb_ == 0;
}

