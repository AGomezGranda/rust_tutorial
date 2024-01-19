use std::cmp::Ordering;
use rand::Rng;
use std::ops::Add;

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

    // Strings
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

    // Casting
    let int_u8: u8 = 5;
    let unt2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (unt2_u8 as u32);

    //Enums
    enum Days{
        Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
    }
    impl Days{
        fn is_weekday(&self) -> bool{
            match self {
                Days::Saturday | Days::Sunday => true,
                _ => false
            }
        }
    }

    let today:Days = Days::Friday;
    match today {
        Days::Monday => println!("It's Monday"),
        Days::Tuesday => println!("It's Tuesday"),
        Days::Wednesday => println!("It's Wednesday"),
        Days::Thursday => println!("It's Thursday"),
        Days::Friday => println!("It's Friday"),
        Days::Saturday => println!("It's Saturday"),
        Days::Sunday => println!("It's Sunday"),
    }
    println!("Is weekend: {}", today.is_weekday());

    //Vectors, only same type
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4, 5];
    vec2.push(5);
    println!("Vec2: {:?}", vec2[3]);
    let second: &i32 = &vec2[1];
    match vec2.get(1){
        Some(second)=> println!("Second: {}", second),
        None => println!("None"),
    }
    for i in &mut vec2{
        *i *= 2;
    }
    for i in &vec2{
        println!("Vec2: {}", i);
    }
    println!("Vec length {:?}", vec2.len());
    println!("Pop {:?}", vec2.pop());

    say_hello();
    println!("{:?}", get_2(5));

    let (val_1, val_2) = get_2(3);
    println!("Nums: {}, {}", val_1, val_2);

    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum of list = {}", sum_list(&num_list));

    //Generics
    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5.2 + 4.5 = {}", get_sum_gen(5.2, 4.5));

    //Ownerships
    // Stack: fast, last in first out, fixed size
    // Heap: slower, dynamic size, can grow in any direction

    let str1 = String::from("World");
    //let str2 = str1;
    let str2 = str1.clone();
    // Not possible to do: println!("Hello{}", str1);
    println!("Hello {}", str1);

    let str1 = String::from("Hello");
    let str2 = str1.clone();
    let str3 = print_return_str(str1);
    println!("{}", str3);
}

fn say_hello(){
    println!("Hello");
}

fn get_2(x: i32) -> (i32, i32){
    return(x+1, x+2)
}

fn sum_list(list: &[i32]) -> i32{
    let mut sum = 0;
    for i in list{
        sum += i;
    }
    return sum;
}

fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T{
    return x + y;
}

fn print_str(x: String){
    println!("{}", x);
}

fn print_return_str(x: String) -> String{
    return x;
}

fn change_string(name: &mut String){
    name.push_str("is happy");
    println!("Message: {}", name);
}