use std::fs::File;
use std::io::{self,BufReader};
use std::io::prelude::*;
use rand::random_range;

fn check_guess(word:&str, guess:&str) -> String {

    let mut display_str = String::new();

    for (index,char) in guess.char_indices(){

        let pos_display = {
                if word.contains(char) && word.find(char) == Some(index) {'*'}
                else if word.contains(char) {'^'}
                else {'0'}
                };
        
        display_str.push(pos_display);                    
    }

    return display_str;
}

fn main() -> io::Result<()>{
    // Read all words from our list of words and create a vector of strings
    let file = File::open("src/wordle-list/words")?;
    let reader = BufReader::new(file); 
    let words: Vec<String> = reader.lines()
        .filter_map(Result::ok)
        .collect();

    // Find a random number and use that to select a random word from the list
    let random_word: usize = random_range(..words.len());
    let random_word: &str = &words[random_word];

    // println!("{random_word}");

    println!("You have 6 guesses. First guess:");

    for trial_num in 1..7 {

        // Read in the user's guess and remove \n at the end
        let mut user_guess = String::new();
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read word!");
        let user_guess = user_guess.trim();

        // TODO: Test if user guess is a valid word, 5 chars and from word list

        if user_guess == random_word {
            let trialstring = (trial_num).to_string() 
                                        + {if trial_num==1 {" trial"} else {" trials"}};
            println!("You won in {trialstring}!!");
            break;
        }

        println!("{}\nNext guess:",{check_guess(random_word, user_guess)});

    }

    // TODO: Print out word if user doesn't get it
    
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn test_norepetition(){
        assert_eq!(check_guess("bring","crabs"),"0*0^0");
    }

    #[test]
    fn test_repetition(){
        // TODO: Failing test, correct logic
        assert_eq!(check_guess("twist","swats"),"^*0^0");
    }
}
