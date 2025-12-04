extern crate rand;


use std::io;
use rand::Rng;


fn main(){
    println!("\n===GUESSING GAME!!===");

    loop{
        let secret_number = rand::thread_rng().gen_range(1,101);// here i a using my rand ilbrary.
        let mut input1 = String::new();
    println!("\n ...Enter your name...: ");// to get the users name 
    io::stdin().read_line(&mut input1).expect("Failed to read input");


    println!("\n..You are to enter a guess..😃");//THE GAME BEGINS
    let mut guess = String::new();
    println!("\n..Enter your guess.: ");
    io::stdin().read_line(&mut guess).expect("Invalid input");
    let guess :u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("The secret number is {}",secret_number);// output of computers secret number

    if guess == secret_number{
        println!("\n...WOW YOU GUESSED RIGHT🥳...");//if else statement to capture if the person gused right or not

        break;
    }
    else{
        println!("...SORRY YOU GUESED WRONGLY 👎...{}",input1);
    }
    let mut quit = String::new();
    println!("..QUIT (Y/N)..");
    io::stdin().read_line(&mut quit).expect("Failed to read line");
    let quit = quit.trim().to_uppercase();

    if quit == "Y"{
        println!("...OKAY BYE 👋...");
        break;
    }
    else if quit == "N"{
        println!("\n ARE YOU SURE !! \n Then OKAYY 🤝");
        continue;

    }
    else{
        println!("ENTER YES OR NO !!!! 😤");// if the player wants to end the game or not
        continue;

    }


    }    
}