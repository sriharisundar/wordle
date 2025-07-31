use std::fs::File;
use std::io::{self,BufReader};
use std::io::prelude::*;
use rand::random_range;

fn main() -> io::Result<()>{
    let file = File::open("src/wordle-list/words")?;
    let reader = BufReader::new(file); 
    let words: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    let random_word: usize = random_range(..words.len());
    let random_word: &str = &words[random_word];
    println!("{}", {random_word});

    let mut user_guess = String::new();
    
    for trial_num in {1..7} {
        
        io::stdin()
            .read_line(&mut user_guess)
            .trim();

        println!("{trial_num}")
    }

    Ok(())
}
