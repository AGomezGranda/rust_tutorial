use std::{io, cmp::Ordering, char};
use rand::Rng;

fn main() {
    println!("What is your name");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    // io::stdin().read_line(&mut name).expect("Didn't recive input");
    println!("Hello {}! {}", name.trim_end(), greeting);

    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.14159;
    let age = "20";
    let mut age: u32 = age.trim().parse()
        .expect("Age was not a number");
    age = age + 1;
    println!("I'm {} years old and I want {}, btw pi number is {}", age, ONE_MIL, PI);

    //Unsigned integers: u8, u16, u32, u64, u128
    //Signed integers: i8, i16, i32, i64, i128

    println!("Max u32 {}", u32::MAX);
    println!("Max u64 {}", u64::MAX);
    println!("Max usize {}", usize::MAX);
    println!("Max f32 {}", f32::MAX);
    println!("Max f64 {}", f64::MAX);

    let is_true: bool = true;
    let is_false: bool = false;
    println!("is_true: {}, is_false: {}", is_true, is_false);

    let num1: f32 = 10.0;
    let num2: f32 = 20.222222;
    println!("result: {}", num1 + num2);

    let random_num = rand::thread_rng().gen_range(5..10);
    println!("random number: {}", random_num);

    let age = 8;
    if age >= 18 {
        println!("You can vote");
    } else {
        println!("You can't vote");
    }
    if (age >= 1) && (age <= 18) {
        println!("Go to school");
    } else if (age > 18) && (age <= 25) {
        println!("Go to college");
    } else {
        println!("Do what you want");
    } 
    
    if (age == 18) || (age == 21) {
        println!("You get a special birthday");
    }
    else if age > 50 {
        println!("You are not special");
    }

    let mut my_age = 47;
    let can_vote = if my_age >= 18 {true} else {false};
    println!("Can vote: {}", can_vote);

    let age2 = 200;
    match age2 {
        1..=5 => println!("Go to kindergarten"),
        6..=18 => println!("Go to school"),
        65..=i32::MAX => println!("Go to retirement"),
        _ => println!("Work!!!"),
    }

    let my_age = 21;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("Can vote this year"),
        
    }

    let arr_1 = [1,2,3, 4];
    println!("1st: {}", arr_1[0]);
    println!("Length: {}", arr_1.len());

    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0{
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    while loop_idx < arr_2.len(){
        println!("Arr: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    for val in arr_2.iter() {
        println!("Val: {}", val);
    }

    //Tuple

    let my_tuple = ("Brad".to_string(), 37, 50_000_000);
    println!("Name: {}", my_tuple.0);
    let(v1, v2, v3) = my_tuple;
    println!("Age: {}", v2);

    let mut st1 = String::new();
    st1.push('A');
    st1.push_str("word");
    for word in st1.split_whitespace(){
        println!("{}", word);
    }
    let st2 = st1.replace("word", "another");
    println!("{}", st2);

    let mut st3 = String::from("x r t b h k k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for c in v1{
        println!("{}", c);
    }
    let st4 = "Random string";
    let mut st5 = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("Len {}", st6.len());
    st5.clear();
    let st6 = String::from("Brad");
    let st7 = String::from("My name is ");
    let st8 = st7 + &st6;

    for char in st8.bytes(){
        println!("{}", char);
    }

}
