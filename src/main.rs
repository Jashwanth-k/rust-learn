// use std::cell::RefCell;
// use std::io;
// use std::rc::Rc;

// fn main_input() {
//     let input: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));
//     let input_data: Rc<RefCell<String>> = input.clone();
//     io::stdin().read_line(&mut input.borrow_mut()).expect("Reading input failed");
//     // let _ = input_data.borrow().replace(&input_data.clone().borrow().trim().to_string(), "");
//     let input_data_str: String =  input_data.borrow().trim().to_string();
//     println!("{}", input_data_str);
// }

// variables
// fn main_variables() {
//     let int_x: i32 = 46;
//     println!("value of x {}", int_x);

//     let int_x: &str = "some str";
//     // int_x.push_str(" abc");
//     println!("value of x {}", int_x);

//     let y: i32 = 64;
//     let mut z: &i32 = &y;
//     z = &12;
//     println!("values x : {}, y : {}", y, z);

//     const SUBSCRIBER_COUNT : i32 = 100000;

//     let mut int_arr: Vec<i32> = Vec::from([1, 2, 3]);
//     int_arr.push(12);
//     let int_arr_2: &[i32] = &[1, 2, 3];
//     println!("int arr value {:?}", int_arr);
//     println!("int arr 2 value {:?}", int_arr_2);
//     println!("subs count {}", SUBSCRIBER_COUNT);
// }

// types

// use std::vec::Vec;
// use sysinfo::System;

// fn main_types() {
//     let float: f32 = 101_22.01;
//     //    let bin : i32 = 0b1011;
//     let hex: i32 = 0xfff;
//     let oct: i32 = 0o77;
//     let e: u8 = b'A';
//     println!("hex number : {}", hex);
//     println!("e value : {}", e);
//     println!("oct value: {}", oct);
//     println!("float value : {}", float);

//     // compund types
//     let int_arr: [i32; 4] = [1, 2, 3, 4];
//     let byte_arr: [i32; 8] = [0; 8];
//     println!("int arr : {:?}", int_arr);
//     println!("byte arr : {:?}", byte_arr);

//     let tuple: (&str, [[i32; 2]; 1]) = ("some string", [[123, 12]]);
//     println!("logging tuple: {:?}", tuple);

//     let mut sys: System = System::new_all();
//     let mut large_vector : Vec<u128> = Vec::from([]);
//     // let mut index : i128 = 0;
//     println!("{}", 2u128.pow(127));
//     loop {
//         sys.refresh_all();
//         large_vector.push(2u128.pow(127));
//         println!("mem usage: {}", sys.used_memory());
//         // index += 1;
//     }
// }

// ownnership
// use std::mem::take;

// fn main_ownnership() {
//     let x: i32 = 10;
//     let y: i32 = x;

//     println!("x : {}, y : {}", x, y);

//     let str_lit: String = String::from("hello");
//     // let str_2 : String = str_lit;
//     // println!("str lit : {}", str_2);
//     take_onwnership(str_lit);
//     // println!("str lit value {}", str_lit);

//     let other_str: String = String::from("hello world");
//     // let a : &String = &other_str;
//     // let b : &String = &other_str;
//     // let mut_key : &mut String = &mut other_str;

//     // println!("mut other key reference: {}", mut_key);
//     let word: String = String::from("word 1");
//     let longest_word: &str;
//     let word2: String = String::from("word 2");
//     longest_word = longest(&word, &word2);
//     // other_str.clear();
//     println!("longest word: {}", longest_word);
// }

// fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
//     if a.len() > b.len() {
//         return a;
//     } else {
//         return b;
//     }
// }

// fn take_onwnership(mut some_str: String) {
//     some_str.push_str("other");
//     println!("{}", some_str)
// }

// fn dangle_pointer() -> String {
//     let new_str : String = String::from("some string");
//     return new_str;
// }

// fn get_first_word(s: &str) -> &str {
//     let bytes = s.bytes();
//     println!("{:?}", bytes);
//     for (i, item) in bytes.enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//         // println!("{}", item);
//     }
//     return s;
// }

// structs
// #[allow(dead_code)]
// #[derive(Debug)]
// struct User<'b> {
//     email: &'b str,
//     name: &'b str,
//     signin_count: u32,
//     active: bool,
// }

// impl<'b> User<'b> {
//     fn get_name(&self) -> &str {
//         return &self.name;
//     }
// }

// fn main_structs() {
//     let mut user = User {
//         email: "test@gmail.com",
//         name: "someone first name",
//         signin_count: 1,
//         active: true,
//     };

//     let mut user2 = User {
//         email: "some email",
//         ..user
//     };
//     user.name = "modified";
//     println!("{:?}", user);
//     println!("{:?}", user2);
//     let name: &str = user.get_name();
//     println!("{}", name);

//     let some_str: String = String::from("some string");
//     let result: &str = first_word(&some_str);
//     println!("{result}");
//     dbg!(&user);
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     return s;
// }

// use std::vec;

// // enums
// #[derive(Debug)]
// enum IpAddrKind<'a> {
//     V4(&'a str),
//     V6(i32),
// }

// #[derive(Debug)]
// struct IpAddr<'a> {
//     address: &'a str,
//     kind: IpAddrKind<'a>,
// }

// enum Coin {
//     RUPEE,
//     DOLLOR,
//     EURO(Country),
// }

// enum Country {
//     INDIA,
//     USA,
//     EU,
// }

// fn get_coin_value(coin: Coin) -> u8 {
//     match coin {
//         Coin::RUPEE => 1,
//         Coin::DOLLOR => 2,
//         Coin::EURO(country) => 3,
//         // Coin::EURO(Country::INDIA) => 4,
//         // Coin::EURO(Country::USA) => 5,
//     }
// }

// fn main_enums() {
//     let localhost = IpAddr {
//         kind: IpAddrKind::V4("127.0.0.1"),
//         address: "http://localhost",
//     };
//     println!("{:?} {}", localhost.kind, localhost.address);

//     // enum Option<T> {
//     //     Some(T),
//     //     None
//     // }
//     let some_number = Some(5);
//     let some_other_number = 6;
//     let total_sum = some_number.unwrap_or(0) + some_other_number;
//     let coin_euro = Coin::EURO(Country::INDIA);
//     let coin_type = get_coin_value(coin_euro);
//     println!("{coin_type}");
// }

// collections

// fn main() {
//     // vectors
//     let a: [i32; 3] = [1, 2, 3];

//     let mut vec_arr: Vec<u32> = Vec::new();
//     vec_arr.push(1);
//     vec_arr.push(2);
//     vec_arr.push(3);

//     match vec_arr.get(1) {
//         Some(n) => println!("{n}"),
//         None => (),
//     }

//     for i in vec_arr {
//         println!("{}", i)
//     }

//     enum Types {
//         Int(u32),
//         Float(f32),
//         String(String),
//     }
//     let vec_arr_types = vec![
//         Types::Int(1),
//         Types::Float(0.1),
//         Types::String(String::from("hello")),
//     ];
//     for i in vec_arr_types {
//         match i {
//             Types::Int(v) => println!("int found {}", v),
//             _ => (),
//         }
//     }

//     // strings
//     let str_1: String = String::new();
//     let str_2: &str = "hello";
//     let str_3: &str = "!";
//     let str_4: String = String::from("world");
//     let str_5 = str_2.to_string() + str_3;
//     println!("{str_5} {}", str_2);
//     // let str_6 = str_2 + str_3;

//     // let x: &str = "somestr";
//     let x = String::from("hello world");
//     if x == "somestr" {
//         let x = 12;
//     }
//     use std::any::Any;
//     // let any: Box<dyn Any> = Box::new(&x);
//     // let mut recovered: Box<String> = any.downcast().expect("something went wrong");
//     // recovered.make_ascii_uppercase();
//     // println!("any value {recovered}")

//     for i in x.bytes() {
//         print!("{:?} ", i)
//     }
//     for i in x.chars() {
//         print!("{:?} ", i)
//     }
//     println!("");
// }

// hashmaps
// fn main() {
//     use std::collections::HashMap;
//     let red = "red";
//     let green = "green";
//     let mut color_map = HashMap::new();
//     color_map.insert(red, 1);
//     color_map.insert(green, 2);
//     println!("{:?}", color_map);

//     for (key, value) in &color_map {
//         println!("{} {}", key, value);
//     }
//     let val = &color_map.entry("red");
//     let data = 1;

//     let text_str = "This is rust hello world, rust is good";
//     let mut freq_map: HashMap<&str, u32> = HashMap::new();
//     for word in text_str.split_whitespace() {
//         let count = freq_map.entry(word).or_insert(0);
//         *count += 1;
//     }
//     println!("{:?}", freq_map);

//     let mut w: u8 = 10;
//     let r = w.clone();
//     let mut q = &mut w;
//     *q = 20;
//     // w = 10;
//     // let r = &q;
//     let s = *q;
//     // w = 20;
//     // let s = *r;
//     // println!("{}:{}", q, s);
//     println!("w: {:?}, q: {}, s: {}", r, q, s)

// }

// Error Handling
// use core::panic;
// use std::fs::File;
// use std::io::{self, ErrorKind, Read};

// fn read_content_from_file(mut file: &File) -> Result<String, io::Error> {
//     let mut s = String::new();
//     file.read_to_string(&mut s)?;
//     return Ok(s);
//     // match file.read_to_string(&mut s) {
//     //     Ok(_) => Ok(s),
//     //     Err(e) => return Err(e),
//     // }
// }

// fn main() {
//     let file = File::open("fileCheck.txt");
//     let file: Option<File> = match file {
//         Ok(file) => Some(file),
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("fileCheck.txt") {
//                 Ok(file) => Some(file),
//                 Err(error) => {
//                     println!("some error: {}", error);
//                     return ();
//                 }
//             },
//             other => {
//                 println!("some error: {}", other);
//                 return ();
//             }
//         },
//     };

//     match file {
//         Some(mut file) => match read_content_from_file(&mut file) {
//             Ok(result) => println!("file : {:?}", result),
//             Err(e) => println!("error while extracting content {}", e),
//         },
//         None => println!("file not found"),
//     }
// }

// Generic Types

// #[derive(Debug)]
// struct Point<T, U> {
//     x : T,
//     y : U,
// }

// impl<T, U> Point<T, U> {
//     fn x(&self) -> &T {
//         &self.x
//     }

//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y : other.y,
//         }
//     }
// }

// impl Point<i32, i32> {
//     fn y(&self) -> &i32 {
//         &self.y
//     }
// }

// fn get_largest<T: PartialOrd + Copy>(arr: &[T]) -> T {
//     let mut largest = arr[0];
//     for el in arr {
//         if *el > largest {
//             largest = *el
//         }
//     }
//     return largest;
// }

// fn main() {
//     let numbers = [1,2,3,56,3,2,4,5];
//     let strings = ["a", "b", "za", "zz", "cz"];
//     let largest_number = get_largest(&numbers);
//     println!("{}", largest_number);
//     let largest_str = get_largest(&strings);
//     println!("{}", largest_str);

//     let point = Point { x : 1.0, y : "something"};
//     let point2 = Point { x : 1, y : "other"};
//     // point2.
//     // let res = Point::y(&point2);
//     let p3= point.mixup(point2);
//     println!("{:?}", p3);
// }

// traits

// use std::{cell::UnsafeCell, fmt::{format, Debug, Display}};

// struct Headline<'a> {
//     content: &'a str,
//     author: &'a str,
// }

// impl Summarise for Headline<'_> {
//     fn author_name(&self) -> &str {
//         self.author
//     }
//     // fn summarise(&self) -> String {
//     //     format!("{} {}", self.content, self.author)
//     // }
// }

// struct Tweet<'a> {
//     content: &'a str,
//     person: &'a str,
//     retweet: bool,
//     image: bool,
// }

// impl Summarise for Tweet<'_> {
//     fn author_name(&self) -> &str {
//         self.person
//     }

//     fn summarise(&self) -> String {
//         format!("{} {}", self.content, self.person)
//     }
// }

// trait Summarise {
//     fn author_name(&self) -> &str;
//     fn summarise(&self) -> String {
//         format!("Read more from {}", self.author_name())
//     }
// }

// fn return_trait(switch : bool) -> impl Summarise {
//     Headline {
//         author: "some other author",
//         content: "headline content",
//     }
// }

// // fn notify(obj: &impl Summarise) {
// //     println!("notified {}", obj.author_name())
// // }

// fn notify<T: Summarise> (obj: T) {
// }

// fn notify_2<T: Summarise> (obj1: T, obj2: T) {
// }

// fn complex<T: Summarise + Clone, U: Display> (t: T, u: U) {
// }

// fn complex_where<T, U> (t: T, u: U) 
//     where T: Summarise + Clone,
//         U: Display + Debug {
// }

// fn main() {
//     let tweet = Tweet {
//         content: "Tweet content",
//         person: "Tweet person",
//         retweet: true,
//         image: false,
//     };
//     let headline = Headline {
//         content: "Headline content",
//         author: "some author",
//     };
//     println!("{}", tweet.summarise());
//     println!("{}", headline.summarise());

//     #[derive(Debug)]
//     enum Option<T> {
//         Some(T),
//         None
//     }
//     let nul_pointer: Option<String> = Option::None;
//     let a= &nul_pointer;
//     println!("nul pointer: {:?}", *a);

//     // 1. Create a raw pointer with the value 0 (null address)
//     let null_ptr: *const i32 = 0 as *const i32;

//     unsafe {
//         // that `null_ptr` points to valid memory.
//         let _value = *null_ptr;
//         println!("Value at null pointer: {}", _value);
//     }

//     println!("Program continues (if it doesn't crash)");
//     // notify(&tweet);
// }

// unsafe fn test_iter() {
//         // this fails at compile-time in Rust
//         let mut v = Vec::from_iter((1..=10).map(|n| String::from(n.to_string())));
//         // using String instead of i32 so Rust uses ref instead of copy for the loop below

//         // let test_str=  String::from("123");
//         for mut s in v.iter() {
//             if s == "5" {
//                 v.push(String::from(s))
//             } 
//         }
// }


// lifetimes

// fn longest(x: &String, y: &String) -> String{
//     return String::from("test");
//     // if x.len() > y.len() {
//     //     x
//     // } else {
//     //     y
//     // }
// }

// fn main() {
//     let string1 = String::from("hellow world");
//     let mut result ;
//     {
//         let string2 = String::from("hello world long");
//         result = longest(&string1, &string2);
//     }
//     println!("result: {}", result);
// }

// iterators

// use std::{fmt::Debug, iter::Sum, vec};

// fn iter_check<T: Copy + Debug>(v1 : T, v2 : T, v3 : T, v4 : T) {
//     let mut vec_int: Vec<T> = Vec::new();
//     vec_int.push(v1);
//     vec_int.push(v2);
//     vec_int.push(v3);
//     vec_int.push(v4);
//     println!("{:?}", vec_int);
//     let mut vec_iter = vec_int.into_iter();
//     // for el in vec_int {
//     //     println!("{:?}", el);
//     // }
// }

// #[derive(Debug)]
// struct Counter {
//     size: u64
// }

// impl Counter {
//     fn new(size: u64) -> Counter {
//         Counter { size }
//     }
// }

// impl Iterator for Counter {
//     type Item = u64;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.size > 0 {
//             let val = self.size;
//             self.size -= 1;
//             Some(val)
//         } else {
//             None
//         }
//     }
// }

// fn main() {
//     let mut vec_int = Vec::from([1,2,3,4,5,6,7,8,9,10]);
//     let vec_int_clone = vec_int.clone();
//     let mut vec_iter = vec_int.iter_mut();
//     let mut vec_iter_val = vec_int_clone.clone().into_iter();

//     let mut vec_modified: Vec<_> = vec_int_clone.iter().filter(|x: &&i32| **x > 5).collect();
//     vec_modified[2] = &80;
//     println!("{:?}", vec_modified);
//     // let test = vec_int.iter();
//     // let val = vec_iter_val.next();
//     // let total_sum: u64 = vec_iter.sum();
//     // println!("{:?}", val);
//     // println!("{:?} {:?}", vec_iter, vec_iter_val);
//     // iter_check(1, 2, 3, 4);
//     for el in &mut vec_iter {
//         // vec_int.push(1);
//         if *el == 2 {
//             *el = 10;
//         }
//         print!("{el} ")
//     }
//     println!("{:?}", vec_int);
//     for mut el in vec_iter_val {
//         if el == 2 {
//             el = 10
//         }
//         print!("{el} ")
//     }
//     println!("{:?}", vec_int_clone);

//     // for el in vec_iter {
//     //     println!("{el}");
//     //     if *el == 5 {
//     //         // *el = 10;
//     //         println!("{el}")
//     //         // vec_int.push(20);
//     //     }
//     // }
//     // println!("{:?}", vec_int);
//     let mut counter_iter: Counter = Counter::new(2);
//     println!("{:?}", counter_iter.next());
//     println!("{:?}", counter_iter.next());
//     println!("{:?}", counter_iter.next());
// }

// smart pointers

// use std::cell::RefCell;
// use std::ops::Deref;

// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     None,
// }

// #[derive(Debug)]
// struct MyBox<T> {
//     value : T
// }

// impl<T> MyBox<T> {
//     fn from(value : T) -> MyBox<T> {
//         MyBox { value }
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.value
//     }
// }

// fn main_box() {
//     let cons_list = List::Cons(1, 
//         Box::new(List::Cons(2, 
//             Box::new(List::None
//         )))
//     );
//     // println!("{:?}", cons_list);
//     // let node2 = append_node();
//     // let mut my_list = List::Cons(1, &node2);

//     let cons_list_a = List::Cons(0, Box::new(cons_list));
//     println!("{:?}", cons_list_a);


//     let mut x = 5;
//     let y = MyBox::from(x);
//     // drop(y);
//     x = 120;
//     // assert_eq!(x, *y);
//     println!("{x}, {:?}", y)
// }

// use std::rc::Rc;
// #[derive(Debug)]
// enum ListRc {
//     Cons(i32, Rc<ListRc>),
//     None
// }

// #[derive(Debug)]
// enum ListCyclic {
//     Cons(i32, RefCell<Rc<ListCyclic>>),
//     None
// }

// fn main() {
//     let cons_list = Rc::new(ListRc::Cons(1, 
//         Rc::new(ListRc::Cons(2, 
//             Rc::new(ListRc::None
//         )))
//     ));

//     println!("{}", Rc::strong_count(&cons_list));
//     let cons_list_a = Rc::new(ListRc::Cons(0, Rc::clone(&cons_list)));
//     println!("{}", Rc::strong_count(&cons_list));
//     {
//         let cons_list_b = Rc::new(ListRc::Cons(-1, Rc::clone(&cons_list)));
//         println!("{}", Rc::strong_count(&cons_list));
//     }
//     println!("{:?}", cons_list_a);
//     println!("{}", Rc::strong_count(&cons_list));
//     // test();

//     let a = Rc::new(ListCyclic::Cons(5, RefCell::new(Rc::new(ListCyclic::None))));
//     println!("a-count {}", Rc::strong_count(&a));

//     let b = Rc::new(ListCyclic::Cons(10, RefCell::new(Rc::clone(&a))));


//     match a.as_ref() {
//         ListCyclic::Cons(_, list) => {
//             *list.borrow_mut() = Rc::clone(&b);
//         },
//         ListCyclic::None => ()
//     }
//     println!("a-count {}, b-count {}", Rc::strong_count(&a), Rc::strong_count(&b));
// }


// fn test() {
//     let a = RefCell::new(12);
//     let b: *mut i32 = a.as_ptr();
//     unsafe {
//         *b = 20;
//         let b = *b;
//     }
//     println!("a : {:?}, b : {:?}", a, b);
// }

// Threads
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("index {i} from secondary thread");
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 0..5 {
        println!("index {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    let new_vec = vec![1,2,3];

    let handle = thread::spawn(move || {
        println!("vec value {:?}", new_vec);
        // panic!("something went wrong");
    });

    handle.join().unwrap();
    // thread::sleep(Duration::from_millis(1000));
    inter_thread_communication();
}

fn inter_thread_communication() {
    let (tx, rx) = mpsc::channel::<&str>();
    // let tx = Rc::new(tx);
    let tx1 = tx.clone();

    let handle = thread::spawn(move || {
        let vec_str = vec!["1", "2", "3"];
        for curr_str in vec_str {
            let _val = tx.send(curr_str).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    // let _val = tx.send("msg from first thread").unwrap();
    thread::spawn(move || {
        let vec_str = vec!["4", "5", "6"];
        for curr_str in vec_str {
            let _val = tx1.send(curr_str).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // let received_data: Vec<&str> = rx.iter().collect();
    // println!("received data: {:?}", received_data);
    for received in rx {
        println!("data: {}", received)
    }
    // handle.join().unwrap();

    // MUTEX
    let n = Mutex::new(5);
    {
        let mut x = Mutex::lock(&n).unwrap();
        *x += 1;
    }
    println!("n value {:?}", n);

    let counter = Arc::new(Mutex::new(10));
    let mut thread_handles: Vec<JoinHandle<()>> = vec![];
    for i in 0..10 {
        let curr_arc_counter: Arc<Mutex<i32>> = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut curr_counter: std::sync::MutexGuard<'_, _> = Mutex::lock(&curr_arc_counter).unwrap();
            *curr_counter += 1;
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    println!("final counter : {:?}", counter);

    // deadlock
    let counter_1 = Arc::new(Mutex::new(5));
    let counter_2 = Arc::new(Mutex::new(5));
    let handle1 = thread::spawn({
        let counter_1_clone = Arc::clone(&counter_1);
        let counter_2_clone = Arc::clone(&counter_2);
        move || {
            let mut counter_1_ref = Mutex::lock(&counter_1_clone).unwrap();
            thread::sleep(Duration::from_millis(1));
            *counter_1_ref += 1;
            let mut counter_2_ref = Mutex::lock(&counter_2_clone).unwrap();
            *counter_2_ref += 1;
        }
    });
    let handle2 = thread::spawn({
        let counter_1_clone = Arc::clone(&counter_1);
        let counter_2_clone = Arc::clone(&counter_2);
        move || {
            let mut counter_2_ref = Mutex::lock(&counter_2_clone).unwrap();
            thread::sleep(Duration::from_millis(1));
            *counter_2_ref += 1;
            let mut counter_1_ref = Mutex::lock(&counter_1_clone).unwrap();
            *counter_1_ref += 1;
        }
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();

    // thread::sleep(Duration::from_secs(3));
    println!("counter1: {:?}, counter_2 {:?}", counter_1, counter_2);
}
