
//NO 1.
// fn main(){
//     let mut wood :i32 = 35;
//     bush(&mut wood);
//     wood = wood * 2;
//     println!("The value of wood is :{}",wood);   

// }
// fn bush (plank: &mut i32){
//     *plank =*plank/5;
//     let wood = *plank/3;//In this code this point was used as a distraction
//     println!("The value of plank is :{}",plank);
// }
//Parse by reference




//NO 2.
// fn main (){
//     let mut link :i32 = 25;
//     sledge(link);
//     link = link * 3;
//     println!("The value of link is:{}",link);

// }
// fn sledge (mut go_link:i32){
//     go_link = go_link/5;
//     println!("go_link value is: {}",go_link);
// }
//Parse by value








//NO 3. 
// fn main(){

//     let first = "Santa Claus".to_string();
//     let noel = &first [3..10];
//     println!("{}",noel);
// }
















//No 4.
// fn main(){
//     let data = ["Ade","Ola","Joye","Adam","Yemi","Sam","Tola"];
//     pass_me(&data[4..]);   
// }

// fn pass_me (use_data:&[&str]){

//     println!("The length of use_data is:{:?}",use_data.len());
//     println!("{:?}",use_data);
// }










// NO 5.
// fn main(){

//     for magic_key in 20..29{

//         if magic_key<=25{
//             continue;
//         }
//         println!("Key is {}",magic_key-3);
//     }
// }













//NO 6.
fn main(){

    let mut lab =15;
    let mut class = 50;
    let mut min =4;
    let mut max = 7;

    while lab < class{
        lab+=min;
        class-=max;
        println!("The value of class = {}",class);
    }
}











//NO 7.
// fn main(){

//     for x in 29..31{
//         for mut m in 15..17{
//             m+=3;
//             let z = m+x;
//             println!("The value of z is {} \n",z);
//         }
//     }
// }














//NO 8.
// fn main(){

//     for num1 in 8..10{
//         for num2 in 16..17{
//             for num3 in 15..17{
//                 let val = num1 + num2 + num3;
//                 println!("{}",val)
//             }
//         }
//     }
// }


















//NO 9.
// fn main(){

//     let mut game :i32 = 25;
//     let mut two :i32 =15;

//     if game > 0{  
//         game += 3;
//         two -= 2;
//         let grass = game+two;
//         game = grass/2; //in this point the .5 is discarded
//         two = game*3;//and rhen this becomes 60 
//         println!("grass ,gmae and two are {} ,{}, {}",grass , game, two);
//     }
// }
