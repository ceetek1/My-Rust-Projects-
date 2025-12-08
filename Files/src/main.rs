//use std::io::Write;

//fn main(){
    //To write into a file in rust it is sha tideous but sha pussible tho 

// In this line i want to like declare a variable to be able to input into the file 
// It is not really necessary sha i can still write into the file its still sha the same thing 

//let written_data = "Omo i am sha writing into this file now i want to experiment this to see wether it works \n na to sha get 5.0 be the goal tho and to make my parents really proud";
//let  mut file = std::fs::File::create("my data.txt").expect("Failed to open file");
//file.write_all("I LOVE RUST".as_bytes()).expect("write failed");
//println!("I don write am abeg 🥳😭");

//}
//use std::io::Read;
//fn main(){
    //let mut file = std::fs::File::open("my data.txt").expect("failed to open file");
    //let mut contents = String::new();
    //file.read_to_string(&mut contents).unwrap();
    //print!("{}",contents);
//}
//use std::fs::OpenOptions;
//use std::io::Write;
//fn main(){

    //let mut file = OpenOptions::new().append(true).open("my data.txt").expect("cannot open file");
    //file.write_all("I just dey change am".as_bytes()).expect("write failed");
   // println!("guy i don append am oooo😭");
//}

use std::io::Write;


fn main(){
    println!("NIGERIAN BREWERY LTD");

    let larger = [
    "33 Export",
    "Desperados",
    "Goldberg",
    "Heineken",
    "Star"];
    

    let stout= [
    "Legend"
    ,"Turbo king"
    ,"Wiliams"];


    let non_alcoholic=[
    "Maltina",
    "Amstel Malt",
    "Malta Gold",
    "Fayrouz"];
let mut file = std::fs::File::create("Drinks.txt").expect("create failed");
file.write_all(larger.join("\n").as_bytes()).expect("write failed");
file.write_all(stout.join("\n").as_bytes()).expect("write failed");
file.write_all(non_alcoholic.join("\n").as_bytes()).expect("write failed");

   



}
