use std::io;

// This program is to recommend eligibility for different categories.
fn main(){
    
    println!("..........This program checks your eligibility.........");
    loop{
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new(); 


    let mut input1 = String::new();
    println!("\n Enter your Name");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("\n Enter your age");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age :f32 = input2.trim().parse().expect("Not a valid number");


    println!("\n Enter Your Gender M/F: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let gender = input3.trim().to_uppercase();

    if gender == "M"{
        println!("\n Enter Your location");

    }
    else if gender == "F"{
        println!("\n ....Enter you location...");

    }
    else{
        println!("\n Invalid input !!!{}",input1);
    }
    
    
    println!("\n  ...RURAL/URBAN... : ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let location = input4.trim().to_uppercase();

    if location == "RURAL"{
        println!("...Checking Eligibility...");
    }
    else if location == "URBAN"{
        println!("...Checking Eligibility...");

    }
    else{
        println!("Invalid input......{}",input1);


    }

    
    if gender == "M" && age >= 16.0 && location=="URBAN"{
        println!("You are Eligible to join the youth sports🥳 ....:{}",input1);
    }
    else if gender == "F" && age >=15.0 && location =="RURAL"{
        println!("You are Eligible to join the youth sports🥳 ....:{}",input1);
    }
    else{
        println!("Sorry you are not eligible to join the youth sports event 👎!....{}",input1);
    }
    println!("Are you Done (Y/N): ");
    io::stdin().read_line(&mut input5).expect("Invalid input");
    let conclusion = input5.trim().to_uppercase();

    if conclusion == "N"{
        continue;

    }
    else{
        break;
    }

}




}

