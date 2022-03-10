use std::io;
fn main() {
    let mut guesses =  Vec::new();
    let mut guesses_left = 6;
    let word = String::from("ARBITRARY").into_bytes();
    let mut display = format!("{}", "_".repeat(word.len()));
    println!("Word : {}", &display);
    while guesses_left > 0{
        let mut guess = String::new();
        println!("Guess a letter");
        io::stdin().read_line(&mut guess).unwrap();
        if guess.len()==2 {
            println!("Input a character");
            continue
        }
        if guess.len()>3 {
            println!("Input one character at a time");
            continue
        }
        guesses.push(guess.to_ascii_uppercase().as_bytes()[0]);
        if word.contains(&guesses[guesses.len()-1]){
            display = remove_guess(&word, guesses[guesses.len()-1] as u8, &guesses);
        } else{
            guesses_left = guesses_left-1;
        }
        println!("Guesses left : {}", guesses_left);
        println!("Word : {}", &display);
        if !display.contains("_"){
            println!("\x1b[92mYou Win!\x1b[0m");
            break;
        }
        println!("-----------------------------------------------------");
        }
        if guesses_left==0{
            println!("\x1b[91mYOU LOSE\nThe word was {}\x1b[0m", String::from_utf8(word).unwrap());
        }
    }

fn remove_guess(arr : &[u8], guess: u8, guesses: &Vec<u8>) -> String{
    let mut result = Vec::new();
    
    for i in 0..arr.len(){
        if arr[i] == guess{
            result.push(guess);
        } else if guesses.contains(&arr[i]){
            result.push(arr[i]);
        } else{
            result.push(b"_"[0]);
        }
    }

    String::from_utf8(result).unwrap()
}