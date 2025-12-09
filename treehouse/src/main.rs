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
use std::io::Write;

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
    loop{
        println!("Hello what is your name");
    let name = what_is_your_name();
    println!("Hello,{}", name);
    let visitors_list = ["charles","david","daniel","john","kamsi","zikora"];
    let mut allow_them_in = false;
    for person in visitors_list {
        if person == name {
            allow_them_in = true;      // I didnt even need to reference anything
        }
    }
    if allow_them_in {
        println!("WELCOME TO MY TREE HOUSE !!!");
    }
    else {
        println!("SORRY YOU ARE NOT ON MY LIST BITCH !!!");
    }
    
    let mut file = std::fs::File::create("visitors allowed in.txt").expect("failed to create file");
    file.write_all(visitors_list.join("\n").as_bytes()).unwrap();
    println!("file printed to screen"); //sucess it worked 


    let mut exit = String::new();
    println!("Exit (YES or NO");
    stdin().read_line(&mut exit).expect("failed to read line");
    let exit = exit.trim().to_uppercase();

    if exit == "YES"{
        println!("GOOD BYE!!");
        break;
    }
    if exit == "NO"{
        println!("OK👍");
        continue;
    }

    
    }
    
}