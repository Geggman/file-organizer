use rand::Rng;

pub fn print_stage(stage: u8) {
    let stage_one = "
         ----
        |    |
        |
        |
        |
    "
    .to_string();

    let stage_two = "
         ----
        |    |
        |    o
        |    
        |
    "
    .to_string();

    let stage_three = "
         ----
        |    |
        |    o
        |    |    
        |
    "
    .to_string();

    let stage_four = "
         ----
        |    |
        |    o
        |   -|    
        |
    "
    .to_string();

    let stage_five = "
         ----
        |    |
        |    o
        |   -|-    
        |
    "
    .to_string();

    let stage_six = "
         ----
        |    |
        |    o
        |   -|-    
        |   |
    "
    .to_string();

    let stage_seven = "
         ----
        |    |
        |    o
        |   -|-    
        |   | |
    "
    .to_string();

    if stage == 1 {
        println!("{}", stage_one);
    }
    if stage == 2 {
        println!("{}", stage_two);
    }
    if stage == 3 {
        println!("{}", stage_three);
    }
    if stage == 4 {
        println!("{}", stage_four);
    }
    if stage == 5 {
        println!("{}", stage_five);
    }
    if stage == 6 {
        println!("{}", stage_six);
    }
    if stage == 7 {
        println!("{}", stage_seven);
    }
}

pub fn pick_random_word() -> String {
    let content =
        std::fs::read_to_string("./data/words.txt").expect("Error reading this f**** file");
    let words: Vec<&str> = content.lines().collect();
    words[rand::thread_rng().gen_range(0, words.len())].to_string()
}

pub fn strip(s : &mut String) -> String {
    s.retain(|c| !c.is_whitespace());    
    s.to_string()
}

pub fn check_right(guess: &str, word: &str) -> bool {
    let guess_char: Vec<char> = guess.chars().collect();
    let word_char: Vec<char> = word.chars().collect();
    if word_char.iter().any(|&i| guess_char.contains(&i)) {
        return true;
    }
    return false;
}

pub fn update_prompt(guess: &String, word: &String, prompt: &mut Vec<char>) -> Vec<char> {
    let guess_char: Vec<char> = guess.chars().collect();
    let word_char: Vec<char> = word.chars().collect();

    for i in 0..word_char.len() {
        if word_char[i] == guess_char[0] {
            prompt.remove(i);
            prompt.insert(i, guess_char[0]);
        }
    }
    return prompt.to_vec();
}

pub fn check_win(prompt: &Vec<char>) -> bool {
    if !prompt.contains(&'_') {
        return true;
    }
    return false;
}
