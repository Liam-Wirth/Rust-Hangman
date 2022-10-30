#[allow(non_snake_case)]
use rand::Rng;
use std::fs;
use std::io; 
use std::path::Path;
fn main() -> io::Result<()> {
    let path = Path::new("src/words.txt");
    let mut rng = rand::thread_rng();
    let words = if let Ok(path) = fs::read_to_string(&path) {
        path.split("\n")
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
    } else {
        println!("SHIT FUCKED!");
        Vec::new()
    };
    
    let word: &String = &words[rng.gen_range(0..words.len())];
        let mut display: String = String::from("");
        {
        let mut i = 0;
        while i < word.len() {
            display.push_str("_");
            i+=1;
            }
        }
    let mut used_letters:String = String::from("");
    let mut tries:u8 = 0; 
    while tries < 8 {
        nuke_terminal();
        if tries >= 1 {
            println!("Letters Used: {}\n",used_letters);
        }
        if &display == word {
            println!("You've found the word! the word was {}, and you had {} guesses remaining", word,8-tries);
            break
        }
        println!("{}\n",display);
        println!("please input a letter you would like to try!");
        let mut temp= String::new();
        io::stdin().read_line(&mut temp)?;
        let input:String = temp.chars().filter(|c| c.is_alphabetic()).collect();
        println!("the filtered string is {}", &input);
        if input.len() <2 && input.len() != 0 {
            if !used_letters.contains(&input){ 
            //this returns a tuple, with the first value being a boolean, and the second being a vector
            //with the needed indices
            let check = check_letter(&word,&input);
           // println!("just checked the letter against the word");
           // println!("the word is {}", word); 
        if check.0{
            let result : Vec<_> = check.1.iter().collect();
            println!("{:?}",result);
            for i in result.iter() {
                display.insert_str(**i,&input);
                display.remove(*i+1);
                used_letters.push_str(&input);
            }
            println!("{}",display);
        } else {
            tries+=1;
            used_letters.push_str(&input);
            
            println!("you have {} tries left",8-tries);
        }

        } else{
            println!("I'm sorry!, It looks like you've already used that letter, please try another letter!");
        }
        

    } else{
        println!("It looks like the input you provided was either not an alphabetic character, or contained more than one letter");
    }
}
    if tries == 8{
    println!("you fucking suck at this lmao, the word was {}", word);
    }
    Ok(())
}
        
pub fn check_letter(word:&String, letter: &str) -> (bool,Vec<usize>) {
    //word.contains(letter)?
     let v  = word.match_indices(&letter).map(|t| t.0).collect();
    if word.contains(&letter){
         return (true,v);
    }
    else {
        (false,v)
    }
}
pub fn nuke_terminal(){
    for i in 0..500{
        println!();
    }
}
