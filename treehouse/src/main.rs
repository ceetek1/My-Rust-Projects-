//use std::io::stdin;

//fn main(){
   // println!("Hello Whats your name? ");
    //let mut your_name = String::new();
   // stdin()
  //  .read_line(&mut your_name)
  //  .expect("Failed to read line");

    //println!("Hello {}",your_name);
//}

// The Dry principle states that u should use a function when you are typing a code repeatedly, Meaning DO NO REPEAT YOUR SELF
//EG

use std::io::stdin;

fn what_is_your_name() -> String{
    let mut your_name = String::new();
    stdin()
    .read_line(&mut your_name)
    .expect("Failed to read line");
    your_name
        .trim()
        .to_lowercase()
}

fn main(){
    println!("Hello what is your name");
    let name = what_is_your_name();
    println!("Hello,{}", name);
    let visitors_list = ["charles","david","daniel","john","kamsi","zikora"];
    let mut allow_them_in = false;
    for visitor in &visitors_list {
        if visitor == &name {
            allow_them_in = true;
        }
    }
    if allow_them_in {
        println!("WELCOME TO MY TREE HOUSE !!!");
    }
    else {
        println!("SORRY YOU ARE NOT ON MY LIST BITCH !!!");
    }
    
}