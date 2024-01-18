#![allow(unused)]

use std::{
    cmp::Ordering,
    f32::consts::PI,
    fs::File,
    hash::BuildHasherDefault,
    io::{self, BufRead, BufReader, ErrorKind, Read, Write},
    ops::BitAndAssign,
    process::Output,
    vec,
};
// use rand::Rng;

// fn main() {
//     // println!("This is my rust program
//     // tell me your name::- ");

//     // let mut name = String::new();
//     // let greeting = "Noce to meet you";
//     // io::stdin().read_line(&mut name)
//     //     .expect("Didn't recieve input!");
//     // println!("Hello {}!! {}",name.trim_end(),greeting);

//     let mut age: i32 = 19;
//     println!("age is {}",age);
//     age=age+1;
// }

// fn main(){
//shadowing
//     const ONE_MIL: u32 = 1_000_000;
//     const PI:f32 = 3.141592;
//     let age = "47";
//     let mut age:u32 = age.trim().parse()
//         .expect("Age wasn't assigned a number!");

//     age = age + 1;
//     print!("I'am {} and {}",age,ONE_MIL);
// }

// fn main(){
//     // println!("MAX u32 : {}",u32::MAX);
//     // println!("MAX u64 : {}",u64::MAX);
//     // println!("MAX i8 : {}",i8::MAX);
//     // println!("MAX f32 : {}",f32::MAX);
//     // println!("MAX usize : {}",usize::MAX);
//     // println!("MAX isize : {}",isize::MAX);

//     // let is_true = true;
//     // let my_grade = 'A';

//     // let nu:f32= 1.11111111111111;
//     // println!("f32:{}", nu+0.0000005);

//     // l
// let n2:f64= 1.11111111111111;
//     // println!("f64:{}", n2+0.0000005);

//       Random
//     let num = rand::thread_rng().gen_range(0..100);
//     println!("{}", num);

// }

//  if

// fn main(){

//     println!("Give input");
//     let mut input = String::new();
//     print!("Give input2");
//     io::stdin().read_line(&mut input).expect("Failed to read input");
//     print!("Give input3");

//     let mut ez:i32;
//     // Convert the input to an integer
//     let num: i32 = match input.trim().parse() {
//         Ok(num) => num,
//         Err(_) => {
//             println!("Invalid input. Please enter an integer.");
//             return;
//         }
//     };

//     // Check if the number is greater than 5
//     if num > 5 {
//         println!("The number is greater than 5.");
//     } else {
//         println!("The number is not greater than 5.");
//     }
// }

//  ternary
// fn main(){

//     let mut my_age = 47;
//     let can_vote = if my_age>=18{
//         true
//     } else{
//         false
//     };

//     println!("Can Vote: {} ",can_vote);
// }

// Match

// fn main(){

//     // let age2= 8;

//     // match age2{
//     //     1..=18=>println!("Imp bday"),
//     //     21 | 50 => println!("Imp date"),
//     //     65..=i32::MAX=> println!("Imp day"),
//     //     _ => println!("Everything else"),
//     // };
// let my_age = 21;
// let voting_age = 18;

// match my_age.cmp(&voting_age){
//     Ordering::Less => print!("Can't Vote"),
//     Ordering::Greater => print!("Can Vote"),
//     Ordering::Equal => print!("Just Vote"),
// };

// }

// Arrays

// fn main(){

//     let arr_1 = [1,2,3,79,89,6,7,8,9];
//     // println!("1st : {}",arr_1[0]);
//     // println!("length : {}",arr_1.len());

//     let mut loop_idx =0;
// //  Dont use tbh
//     // looping

//     // loop{
//     //     if arr_1[loop_idx]%2 ==0{
//     //         loop_idx +=1;
//     //         continue;
//     //     }

//     //     if arr_1[loop_idx] == 4{
//     //         break;
//     //     }
//     //     print!("Val : {}\n", arr_1[loop_idx]);
//     //     loop_idx+=1
//     // }

// // while loop_idx < arr_1.len(){
// //     println!("Arr: {}", arr_1[loop_idx]);
// //     loop_idx+=1;
// // }

//         // for val in arr_1.iter(){
//         //     println!("VAl : {}",val);
//         // }
// // // with & its dereferenceing
// //         for &value in &arr_1{
// //             println!("VAl: {}", &value);
// //         }

// }

// Tuples

// fn main(){

//     let my_tuples: (u8,String,f64)= (89,"hello".to_string(),3.15);
//     // let my_tuples: (u8,&str,f64)= (89,"ello",3.15);

//     println!("Name: {}",my_tuples.1);
//     let (v1,v2,v3) = my_tuples;
//     print!("V1: {}",v1);
// }

// Strings

// fn main(){
//     // String , &str

// // let mut st1 = String::new();
// // st1.push('A');
// // st1.push_str("Word");

// // for word in st1.split_whitespace(){
// //     print!("{}\n ",word);

// // }

// // // changes are saved in st2 and st1 remains the same
// // let st2 = st1.replace("A","Another");
// // println!("\nst1 now : {}",st1);
// // println!("Change : {}",st2);

// // let st3= String::from("x r t b h i i q p c");
// // let mut v1: Vec<char> = st3.chars().collect();
// // v1.sort();
// // v1.dedup();
// // for char in v1{
// //     println!("char {}",char);
// // }

// // let st4: &str = "Random String";
// // let mut st5= String::from(st4);
// // // let mut st5: String=st4.to_string();  //same

// // println!("{}",st5);

// // let byte_arr1 = st5.as_bytes();

// // let st6 = &st5[0..6];

// // println!("String length : {}", st6.len());

// // st5.clear();

// // println!("{}", st5);

// let st6=String::from("AbCdE ZyX");
// let st7=String::from(" QWERty op");
// let st8=st6+ &st7;
// println!("{}",st8);

// for char in st8.bytes(){
//     println!("{}", char);
// }

// }

// casting

// fn main(){

//     let int_u8:u8 = 5;
//     let int2_u8:u8 = 54;
//     let int3_u32:u32 = (int2_u8 as u32) + (int_u8 as u32);
//     print!("{}" ,  int3_u32);
// }

// Enums

// fn main() {
//     enum Day {
//         Monday,
//         Tuesday,
//         Wednesday,
//         Thrusday,
//         Friday,
//         Saturday,
//         Sunday,
//     }

//     impl Day {
//         fn is_weekend(&self) -> bool {
//             match self {
//                 Day::Saturday | Day::Sunday => true,
//                 _ => false,
//             }
//         }
//     }
//     let today: Day = Day::Monday;
//     match today {
//         Day::Monday => println!("Mondayyy"),
//         Day::Tuesday => println!("Tuessss"),
//         Day::Wednesday => println!("Wednesdayyyy"),
//         Day::Thrusday => println!("Thrusdayy"),
//         Day::Friday => println!("Fridayy"),
//         Day::Saturday => println!("SAturdasyy"),
//         Day::Sunday => println!("Sundayyy"),
//     }
// }

// //  Vectors

// fn main(){

//     let vec1: Vec<i32> = Vec::new();

//     let mut vec2 = vec![1,2,3,4];
//     vec2.push(5);
//     println!("1st :{}",vec2[0]);

//  let sec : &i32 = &vec2[1];

//  match vec2.get(1){
//     Some(sec)=>println!("2nd : {}",sec),
//     None =>println!("No 2nd value"),

//  };

// for  i in &mut vec2{
//     *i *=2;

// }

// for i in &vec2{
//     println!{"{}  ",i};

// }

// println!("Vec leng: {}",vec2.len());
// println!("Vec pop: {:?}",vec2.pop());

// }

// fn hello(x: i32,y: i32) -> (i32,i32){
//     // let z = x*y;
//     println!("Z: {}",x+y);
//     println!{"helloo"};

//     // x*y
//     return (x*y,x+y);
// }

// fn listt(x:&[i32]) ->i32{

//     let mut sum = 0;
//     for &val in  x.iter(){
//         sum+=&val;

//     }
//     sum
// }

// fn main(){

//     // let (val_1,val_2) = hello(3,4);
//     // println!("Nums : {} {}",val_1,val_2);

//     let num_list= vec![1,2,3,4,5];
//     println!("Sum of list - {} ", listt(&num_list));
// }

// // Generics

// use std::ops::Add;

// fn gen<T:Add<Output = T>>(x:T,y:T) -> T{
//     return x + y;

// }
// fn main(){

// println!("5 + 4 = {}",gen(5,4));
// println!("5 + 4 = {}",gen(7.98,4.33));

// }

// // Hashmap
// use std::collections::HashMap;

// fn main(){

//     let mut heros=HashMap::new();
//     heros.insert("Superman", "Clark kent");
//     // heros.insert("Batman", "bruce wayne");
//     heros.insert("iron", "MC ");

//     for (k,v)in heros.iter(){
//         println!("key: {}, value : {}",k,v);
//     }

//     println!("Length: {}",heros.len());

//     let the_batman= heros.get(&"Batman");

//     match the_batman{
//         Some(x)=> println!("Batman is a hero"),
//         None => println!("Batman is noob"),
//     };

// }

// // Structs
// // Like C and one other way
// fn main(){

//     const PI:f32 =3.14592;
// // struct Rec<T,U>{
// //     length: T,
// //     height: U,
// // }

// // let rec= Rec{length: 4, height: 10.5};

// trait Shape{

//     fn new(length: f32, width: f32)-> Self;
//     fn area(&self) ->f32;
// }

// struct  Rectangle {length: f32, width: f32};
// struct  Circle {length: f32, width: f32};

// impl Shape for Rectangle{
//     fn new(length: f32, width: f32)-> Rectangle{
//         return Rectangle{length,width};
//     }

//     fn area(&self) ->f32{
//         return self.length * self.width;
//     }
// }

// impl Shape for Circle{
//     fn new(length: f32, width: f32)-> Circle{
//         return Circle{length,width};
//     }

//     fn area(&self) ->f32{
//         return (self.length/2.0).powf(2.0)*PI;
//     }
// }

// let rec: Rectangle = Shape::new(10.0,10.0);
// let cir: Circle = Shape::new(10.0,10.0);

// println!("Rect : {}",rec.area());
// println!("Circ;e : {}",cir.area());

// }

// // modules

// mod restaurant;
// use crate::restaurant::order_food;
// fn main(){
//     order_food();
// }

// // error handling

// fn main(){
//     let path = "lines.txt";
//     let output = File::create(path);
//     let mut output=match output{
//         Ok(file)=>file,
//         Err(error)=> {panic!("problem creating file: {:?}",error);}
//     };
//     write!(output,"lorem ipsum \n uaeshfaEF").expect("Failed to write");

//     let input = File::open(path).unwrap();
//     let buffered= BufReader::new(input);

//     for line in buffered.lines(){
//         println!("{}",line.unwrap());

//     }

//     let output2 =File::create("abc.txt");
//     let output2 =match output2{
//         Ok(file) => file,
//         Err(error)=> match error.kind(){
//             ErrorKind::NotFound=> match File::create("abc.txt"){
//                 Ok(fc)=>fc,
//                 Err(e)=>panic!("Can't create file {:?}",error),
//             },
//             _other_error=>panic!("Prob opein file : {:?}",error),
//         },
//     };

// }

// iterators

// fn main(){

//     let mut arr_it = [1,2,3,4];

//     for val in arr_it.iter(){
//         println!("{}",val);
//     }

//     // arr_it.into_iter();

//     let mut iter1 = arr_it.iter();
//     println!("1st : {:?} ", iter1.next())
// }

//   closures

// let var_name = |parameters| -> return_type
// {Body}
// fn main(){
// let can_vote= |age:i32| {
//     age>= 18
// };

//     println!("Can vote : {}",can_vote(8));
// }
// fn  main(){
// let mut samp1=5;
// let print_var=|| println!("samp1 = {}",samp1);

// print_var();

// samp1 = 10;
// let mut change_var = || samp1 +=1;
// change_var();
// println!("samp = {}",samp1);
// samp1 = 10;
// println!("samp1 = {}", samp1);

// }

// fn main(){
//     fn use_func<T>(a:i32, b:i32, func:T)-> i32
//     where T: Fn(i32,i32)->i32{
//         func(a,b)

//     }
//     let sum = |a,b| a+b;
//     let prod = |a,b| a*b;
//     println!("5+4 = {}",use_func(5,4,sum));
//     println!("5*4 = {}",use_func(5,4,prod));
//     println!("5+4 = {}",sum(5,4));

// }

// // smart pointer

// fn main() {
//     //  BOX stores in heap

//     struct TreeNode<T> {
//         pub left: Option<Box<TreeNode<T>>>,
//         pub right: Option<Box<TreeNode<T>>>,
//         pub key: T,
//     }

//     impl<T> TreeNode<T> {
//         pub fn new(key: T) -> Self {
//             TreeNode {
//                 left: None,
//                 right: None,
//                 key,
//             }
//         }

//         pub fn left(mut self, node: TreeNode<T>) -> Self {
//             self.left = Some(Box::new(node));
//             self
//         }

//         pub fn right(mut self, node: TreeNode<T>) -> Self {
//             self.right = Some(Box::new(node));
//             self
//         }
//     }

//     let node1  =TreeNode::new(1).left(TreeNode::new(2)).right(TreeNode::new(3));
// }

// Concurrency

use std::thread;
use std::time::Duration;

// fn main(){

//    let thread1 =  thread::spawn(||{
//         for i in 1..25{
//             println!("Spawned thread: {}",i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..20{
//         println!("Main thread: {}",i);
//         thread::sleep(Duration::from_millis(1));
//     }
//     thread1.join().unwrap(); // to make sure that all the threads are completed
// }

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    pub struct Bank {
        balance: f32,
    }
    // fn withdraw(the_bank: &mut Bank, amt:f32){
    //     the_bank.balance-=amt;
    // }

    // let mut bank = Bank{balance: 100.0};
    // withdraw(&mut bank, 5.00);

    // fn customer(the_bank: &mut Bank){
    //     withdraw(the_bank,5.00);
    // }

    // thread::spawn(||{
    //     customer(&mut bank)

    // }).join().unwrap();

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!(
                "Current Balance : {} \n withdrawal a smaller amt",
                bank_ref.balance
            )
        } else {
            bank_ref.balance -= amt;
            println!("Customer withdraw {} Curent Balance", bank_ref.balance);
        }
    }

    fn customer(the_bank: &Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.0);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance: 20.00 }));
    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(move || customer(&bank_ref))
    });

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total {}", bank.lock().unwrap().balance);
}
