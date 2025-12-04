use std::io;

fn main(){
  loop{
      println!("\n=====STUDENT LOAN ESTIMATOR=====");

    let mut input = String::new();
    println!("...Enter your name...: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut r = String::new();
    println!("..Enter yor given rate: ");
    io::stdin().read_line(&mut r).expect("Failed to read line");
    let r:u32 = r.trim().parse().expect("Invalid input");


    let mut amount_borrowed = String::new();
    println!("..Enter amount you want to borrow: ");
    io::stdin().read_line(&mut amount_borrowed).expect("Failed to read line");
    let amount_borrowed:f32= amount_borrowed.trim().parse().expect("Invalid input");

    let mut years = String::new();
    println!("..Enter the years: ");
    io::stdin().read_line(&mut years).expect("Failed to read line");
    let years :u32= years.trim().parse().expect("Invalid input");

    let mut time = String::new();
    println!("..Enter time: ");
    io::stdin().read_line(&mut time).expect("Failed to read line");
    let time:u32 = time.trim().parse().expect("Invalid input");


    
  }

}
