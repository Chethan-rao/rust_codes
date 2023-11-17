// fn main() {
//     println!("Hello");
// }

// 1. Variables
// let apples = 5; // immutable - default type i32
// let mut bananas = 5; // mutable

// Explicit type conversion

// let a: i32 = 5;    // i8 i32 i64 i128
// let a: u32 = 5;
// let a = 5i32;
// let a = 5_i32;
// let a = 4 as i32
// let z = a + (b as i32);

// u32 : 0 - 2^8 - 1 (0-255)
// i32 : -2^7 - 2^7 - 1 (-127 - 127)

// let floating_point: f32 = 10.8

// let true_or_false: bool = true (or 1)

// let letter: char = 'a'

// let string_var: &str = "chethan";

// let x = i32::MAX;

// "/" of 2 same type will result in that type itself i8/i8 = i8

// fn main() {
//     // let x = 5;  implicit type conversion
//     let mut x:i 32  = 5; // exclicit type conversion
//     println!("before x = {}",x);
//     x = 6;
//     println!("after x = {}",x);

//     // another way of making variable mutable, use let again to re declare
//     let y = 6;
//     println!("before y = {}",y);
//     let y = 7;
//     println!("after x = {}",y);

//     let x: u8 = 5;
//     let y = x; // y is u8 type now;
// }

//  2. Constant

// fn main() {
//     const SECONDS_IN_MINUTE: u32 = 60;  // you have to specify type when declaring constant
//     const SECONDS_IN_MINUTE: u32 = 50; // unlike normal variables, you can re-declare constant
//     println!("{}",SECONDS_IN_MINUTE);
// }

// 3. Tuple - immutable

// fn main() {
//     let tup1: (i32,&str,char) = (1,"chethan",'A');
//     let mut tup2 = (2,"hello"); // This is mutable
//     tup2.0 = 4;
//     println!("{}",tup1.1); //use . to access elements
//     println!("{}",tup2.0);
// }

// 4. Arrays

// fn main() {
//     let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
//     arr[0] = 6; // using square bracket
//     println!("{}",arr[0]);

//     for i in 0..arr.len(){
//         print!("{} ",arr[i]);
//     }
//     println!();

//     let arr2 = [5; 5];

//     for ele in arr2 {
//         print!("{} ",ele);
//     }
//     println!();

//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");
// }

// 5. Taking input

// use std::io;  // std - crate and io - module and  :: path separator operator
// fn main() {
//     let mut input = String::new(); // declare empty string variable
//     io::stdin()  // stdin is specific type for taking inputs
//         .read_line(&mut input)  // read_line function and storing user typed result in input variable
//             .expect("failed to read line"); // error handling
//     println!("{}",input);
// }

// Taking array of list as input
// use std::io;
// fn main() {
//     let mut arr = [0; 5];
//     for i in 0..arr.len() {
//         let mut ele = String::new();
//         io::stdin().read_line(&mut ele).expect("failed");
//         arr[i] = ele.trim().parse().unwrap();
//     }

//     for i in arr {
//         println!("{}",i);
//     }
// }

// 6. String to Int

// use std::io;
// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("failed");
//     let int_input: i64 = input.trim().parse().unwrap();
//     // trim removes leading and trailing whitespace
//     // parse returns converted string to another type mentioned
//     // unwrap is for unwrapping the value. this will either give us parsed value or panic value if error occurs
//     println!("{}",int_input + 4);
// }

// 7. Conditions - ! && ||

// fn main() {
//     let food = "d";
//     if food == "cookie" {
//         println!("I like cookie");
//     } else if food == "bun"{
//         println!("I like bun");
//     } else{
//         println!("Too bad");
//     }
// }

// 8. Expression and statement

// fn main() {
//     let num1 = 6; // statement;
//     let num2 = {5}; // expression
//     let num3 = {
//         let temp = 7;  //expression
//         temp + 1  // dont put semicolon at last
//     };
//     println!("num1 = {}, num2 = {}, num3 = {}",num1,num2,num3);
// }

// 9. function

// fn main() {
//     let sum = add(6,5);
//     println!("{}",sum);
// }

// fn add(x:i32, y:i32) -> i32 {   // necessary to mention type of parameter and return type
//     return x + y;
//     // x + y // don't put semicolon, because this is default return statement
// }

// 10.Ownership

// fn main(){
//     let x = String::from("hello"); // taking owner_ship
//     let y = fun(x); // borrowing as well as taking owner_ship;
//     // println!("{}",x); // gives error as x is borrowed so it doesn't live in this scope anymore
//     println!("{}",y);
// }
// fn fun(x: String) -> String{
//     return x; // borrowing
// }

// 11. Referencing

// fn main(){
//     let x = String::from("hello");
//     let y = fun(&x); // passing reference of x
//     println!("{}",x); // doesn't give error as x is not borrowed
//     println!("{}",y);
// }
// fn fun(x: &String) -> &String{ // borrowing but not taking ownership
//     return x;
// }

// In order to make references mutable pass the mutable string
// fn main(){
//     let mut x = String::from("hello");
//     change(&mut x);
//     println!("{}",x);
// }

// fn change(x: &mut String){
//     x.push_str(" world");
// }

// You can create only one mutable reference

// fn main(){
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s; // error

// To get rid of that make s immutable,
//     let s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s; // error

//     println!("{} , {}",r1,r2);
// }

// 12. Passing by reference
// fn main(){
//     let mut x = 5;
//     fun(&mut x);
//     println!("{}",x);
// }
// fn fun(x:&mut i32){
//     *x = *x + 1;
// }

// Even to change a value x, make it mutable equivalent to y
// fn main(){
//     let mut x = String::from("hello");
//     let y = &mut x;
//     y.push_str(" world");
//     println!("{}",y);
// }

// 13. Slicing
// fn main() {
//     // let mut s = String::from("hello");
//     // let hello = &s[0..5];  // [..5]
//     // let world = &s[6..11]; // [6.. ]

//     let mut a = [1,2,3,4,5];
//     let mut slice = &mut a[0..2];
//     slice[0] = 3;
//     for i in slice {
//         println!("{}",i);
//     }

//     for i in a{
//         println!("{}",i);
//     }
// }

// 14. Structs

//Example 1
// struct User {
//     user_name: String,
//     age: i32,
//     active: bool
// }

// fn main() {
//     let mut user1 = User {
//         user_name: String::from("Chethan"),
//         age: 21,
//         active: true
//     };

//     let mut name = user1.user_name; // Chethan
//     user1.user_name = String::from("Rao");  // Rao
//     println!("{}",name);                    // Chethan

//     let user2 = build_user(String::from("Yashoda"), 45, true);

//     let user3 = User {
//         user_name: String::from("Shankar"),
//         ..user2
//     };
// }

// fn build_user(name: String, age: i32, active: bool) -> User {
//     User {
//         user_name: name,
//         age:age,
//         active: active
//     }
// }

// Example 2

// #[derive(Debug)] // To print struct
// struct Rectangle {
//     width: i32,
//     height: i32
// }

// impl Rectangle {   // bind the related methods to current structure it itself using impl
//     fn cal_area(&self) -> i32{
//         self.width*self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// impl Rectangle {  // Associated Function
//     fn square(size: i32) -> Rectangle {   // no self
//         Rectangle { width: size, height: size }
//     }
// }

// fn main() {
//     let rect = Rectangle {
//         width: 5,
//         height: 10
//     };

//     println!("rect: {:#?}",rect); // To print struct

//     let rect2 =  Rectangle {
//         width: 4,
//         height: 6
//     };

//     println!("Area of rec = {}",rect.cal_area());

//     println!("Rect1 can hold rect2?  : {}",rect.can_hold(&rect2));

//     let rect3 = Rectangle::square(6); // No need to create an object
//     println!("rect3: {:#?}",rect3);
// }

// 15. Enums and pattern matching

// enum IpAddrKind {  // Group related named values
//     V4(String),
//     V6(String)
// }

// enum  Message {
//     Quit,
//     Move {x: i32, y:i32},
//     Write(String),
//     ChangeColor(i32, i32, i32)
// }

// impl Message {
//     fn some_fun () {
//         println!("Inside enum fun");
//     }
// }

// struct IpAddr {   // Group related data fields
//     kind: IpAddrKind,
//     address : String
// }

// fn main() {
//     // let four = IpAddrKind::V4;
//     // let six = IpAddrKind::V6;

//     // let localhost = IpAddr {
//     //     kind: four,
//     //     address: String::from("127.0.0.1")
//     // };

//     // Simplified
//     let localhost = IpAddrKind::V4(String::from("127.0.0.1"));

//     Message::some_fun();
// }

// Ex - 2 Option enum

// enum Option<T> {
//     Some(T),
//     None
// }
// fn main() {
//     let x = 5;
//     let y = Some(6); // Default Option enum which handles both None and value
//     // let sum = x + y; // error because x & y are both different types

//     let sum = x + y.unwrap_or(0);
// }

// Ex - 3 Match Expression

// enum Coin {
//     Ten,
//     Hundred,
//     Thousand,
//     Lakh,
// }

// impl Coin {
//     fn value_in_numbers(&self) -> i32 {
//         match self {
//             Coin::Ten => 10,
//             Coin::Hundred => 100,
//             Coin::Thousand => 1000,
//             Coin::Lakh => 100000
//         }
//     }
// }

// fn main() {
//     let coin = Coin::Thousand;
//     println!("{}",coin.value_in_numbers());
// }

// Ex - 4 , Combine match and option enum

// fn main() {
//     let five = Some(5);

//     let six = plus_one(five);
//     println!("{}",six.unwrap_or(0));

//     let none = plus_one(None);
//     println!("{:?}",none);
// }

// fn plus_one(x:Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1)
//     }
// }

// Ex - 5, another example

// fn main(){
//     let some_value = Some(3);
//     match some_value {
//         Some(3) => println!("Three"), // If 3 print three
//         _ => ()  //   // Else if anything, dont do anything
//     }

//     // Alternate

//     if let Some(5) = some_value {
//         println!("Five");
//     }
// }

// 16. Modules

// mod lib;

// use lib::restuarent;

// fn main() {
//     let mut meal = restuarent::Breakfast::summer(String::from("Chapathi"),String::from("Apple"));

//     meal.print_meal();
// }

// 17. Collections

//vectors - its a macro just like println

// fn main() {
//     let a = [1,2,3];
//     let mut v1:Vec<i32> = Vec::new(); // Empty vector
//     v1.push(1);
//     v1.push(2);
//     v1.push(3);

//     let v2 = vec![1,2,3,4,5];  // Vector initialization
//     let mut third = v2[2];
//     third = third + 1;
//     println!("{}",third); // 4  but v2[2] = 3

//     let mut v2 = vec![1,2,3,4,5];
//     let third = &mut v2[2];   // Another way to change v[2] by referecing it to third va
//     *third += 1;
//     println!("{}", v2[2]); // prints 4 and third = 3

//     // Iterating
//     for ele in v2 {
//         println!("{}",ele);
//     }

//     // 2d vector
//     let rows = 3;
//     let cols = 4;
//     let mut matrix = vec![vec![0; cols]; rows];

//     matrix[1][2] = 5;

//     for row in &matrix {
//         for col in row {
//             print!("{} ", col);
//         }
//         println!();
//     }

// }

// String -- collection of UTF-8 encoded

// fn main() {
//     // let s1 = String::new();
//     // let s2 = "hello";
//     // let s3 = s2.to_string();
//     // let s4 = String::from("hello");

//     let mut s = String::from("hello");
//     s.push_str(" world"); //  append string
//     s.push('!');   // append character
//     println!("{}",s);

//     let s1 = String::from("hello");
//     let s2 = String::from(" world");
//     let s3 = s1 + &s2; // concatenate using +
//     println!("{}",s3);

//     // iterating though string
//     let s = String::from("hello world");
//     for char in s.chars(){
//         println!("{}",char);
//     }

//     let c = s.chars().nth(1).unwrap(); // To access particular index character
//     println!("{}",c);
// }

// HashMap

// use std::collections::HashMap;
// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert("Team 1", 10);
//     scores.insert("Team 2", 50);

//     let team_A_score = scores.get("Team 1");
//     println!("{}",team_A_score.unwrap_or(&-1));

//     //iterating over hashmap
//     for (key, val) in scores {
//         println!("key: {}, val: {}",key,val);
//     }

//     // Creating frequency of words
//     let text = "hello world wonderful world";
//     let mut map = HashMap::new();
// map.contains_key(key) // return boolean
//     for word in text.split_whitespace() {
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }
//     print!("{:?}",map);
// }

// 18. Error handling

// fn main() {
//     panic!("Error");
// }

// 19. Generics

// Ex - 1
// fn main() {
//       let v1 = vec![1,2,3,4];
//       println!("{}",find_largest(v1));

//       let v2 = vec![1.3,4.2,3.2];
//       println!("{}",find_largest(v2));

// }

// fn find_largest<T: PartialOrd + Copy>(v1: Vec<T>) -> T {
//       let mut largest = v1[0];
//       for ele in v1 {
//             if ele > largest {
//                   largest = ele;
//             }
//       }
//       largest
// }

// Ex - 2

// struct Point<T> {
//       x: T,
//       y: T
// }

// impl<T> Point<T> {
//       fn funx(&self) -> &T {
//             &self.x
//       }
// }
//  fn main() {
//       let p1 = Point{ x: 5, y: 10};
//       println!("{}",p1.funx());
//  }

// 20. Traits (interfaces in java)

// pub struct Movie {
//       name: String,
//       year: i32
// }

// impl Summary for Movie {
//       fn summarize(&self) {
//           println!("Move name: {}, released in: {}",self.name,self.year);
//       }
// }
// pub struct Bike {
//       bikename: String,
//       year: i32
// }

// impl Summary for Bike {
//       fn summarize(&self)  {
//           println!("Move name: {}, released in: {}",self.bikename,self.year);
//       }
// }

// pub trait Summary {   // Both struct can implement this trait (shared)
//     fn summarize(&self); // other structs should implement this
//     fn default_fun(&self) {  // If implemented, then this is overridden
//       println!("Default trait");
//     }
// }

// fn main() {
//       let mov1 = Movie {name: String::from("Kabzaa"), year: 2023};
//       let bike1 = Bike {bikename: String::from("Triump"), year: 2012};

//       mov1.summarize();
//       bike1.summarize();
// }

// 21. Lifetime

// fn main () {
//       let s1 = "hello";
//       let s2 = "hello world";

//       let result = longest(&s1,&s2);
//       println!("{}",result);
// }

// fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
//       if s1.len()>s2.len(){
//             s1
//       }
//       else{
//             s2
//       }
// }

// 22. Binary Search
// use std::io;
// fn main() {
//       let arr = [2,3,5,7,8,9];
//       let mut key = String::new();
//       println!("Enter the key to find: ");
//       io::stdin().read_line(&mut key).expect("Failed");

//       let key = key.trim().parse().unwrap();
//       let result = bin_search(&arr, key);
//       match result {
//             Some(x) => println!("key found at {}",x),
//             None => println!("Key not found")
//       }
// }

// fn bin_search(arr: &[i32;6], x: i32) -> Option<i32>{
//       let mut l:i32 = 0;
//       let mut h:i32 = (arr.len() as i32) - 1 ;
//       while l <= h {
//             let m = (l + h) / 2;
//             if arr[m as usize] == x {
//                   return Some(m);
//             }
//             else if arr[m as usize] < x {
//                   l = m + 1;
//             }
//             else {
//                   h = m -1;
//             }
//       }
//       None
// }

// 23. Frequency of characters

// use std::collections::HashMap;
// use std::io;
// fn main() {

//     let mut input = String::new();
//     println!("Enter string: ");
//     io::stdin().read_line(&mut input).expect("Failed");

//     let mut map:HashMap<char, i32> = HashMap::new();

//     for c in input.trim().chars() {
//         let count = map.entry(c).or_insert(0);
//         *count = *count + 1;
//     }

//     for (key, val) in map.iter() {
//         println!("{} - {}",key,val);
//     }
// }

// 24.

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];
//     let mut first = &mut v[0];
//     *first = 6;
//     println!("{}",v[0]);
// }

// 25. Raw string literal - To escape double quotes inside string (Syntax -> r#"String"#)

// fn main() {
//     let res = "Hello \"world\"";
//     println!("{res}");

//     // Instead use raw string literal
//     let res = r#"Hello "world" "#;
//     println!("{res}");
// }

// 26. Binary, Octal, Hexa printing
// fn main() {
//     let num = 15;
//     println!("{:b}", num); // Binary
//     println!("{:o}", num); // Octal
//     println!("{:x}", num); // Hexa
// }

// 27. Dyn dispatch vs static dispatch
// trait Drive {
//     fn drive(&self);
// }
// struct Sedan;

// impl Drive for Sedan {
//     fn drive(&self) {
//         println!("Sedan is driving");
//     }
// }

// struct SUV;

// impl Drive for SUV {
//     fn drive(&self) {
//         println!("SUV is driving");
//     }
// }

// // Static dispatch
// fn road_trip(car: &impl Drive) {
//     car.drive();
// }

// // Dyn dispatch
// fn road_trip(car: Box<dyn Drive>) {
//     car.drive();
// }

// struct Abc;

// fn main() {
//     let sedan = Sedan;
//     let suv = SUV;

//     road_trip(&sedan);
//     road_trip(&suv);
// }

// 32. Dynamic dispatch 2
// use std::fmt::Debug;

// trait Car {
//     fn drive(&self);
// }

// #[derive(Debug)]
// struct Bmw;

// impl Bmw {
//     fn get_prize(&self) -> i32 {
//         100
//     }
// }

// #[derive(Debug)]
// struct Audi;

// impl Audi {
//     fn get_prize(&self) -> i32 {
//         999
//     }
// }

// impl Car for Bmw {
//     fn drive(&self) {
//         println!("Driving BMW");
//     }
// }

// impl Car for Audi {
//     fn drive(&self) {
//         println!("Driving Audi");
//     }
// }

// fn main() {
//     let new_bmw = helper1(1);
//     let new_audi = helper1(2);
// }

// fn helper1(item: i32) -> &'static dyn Car {
//     if item == 1 {
//         return &Bmw;
//     }
//     return &Audi;
// }

// 28. From and into -> Both are same, If u implement From trait for a type, then Into will be automatically implemented as well
// #[derive(Debug)]
// struct Number {
//     num: i32,
// }

// impl From<i32> for Number {
//     fn from(value: i32) -> Self {
//         Self { num: value }
//     }
// }

// fn main() {
//     // In "from", u use from()
//     let num1_from = Number::from(12);

//     // In "into", u use into() by explicitly specifying the type
//     let num2_into: Number = 34.into();

//     let res: i32 = num1_from.into();

//     println!("{num1_from:?}");
//     println!("{num2_into:?}");
// }

// // 29. Vec implements From<array>, that's y u can call into trait on array by specifying Vec as type
// fn main() {
//     let arr = [1, 3, 4];
//     let vec1 = Vec::from(arr);
//     let vec2: Vec<_> = arr.into();
//     println!("{vec1:?}");
//     println!("{vec2:?}");
// }

// use std::collections::HashMap;

// fn main() -> () {
//     let hm = HashMap::from([("abc", HashMap::from([("123", vec![1, 2, 3])]))]);

//     let res = hm.get("abc");
//     let boolean = {
//         if let Some(res) = res {
//             let inner_hm = res.get("123");
//             if let Some(inner_hm) = inner_hm {
//                 inner_hm.contains(&1)
//             } else {
//                 false
//             }
//         } else {
//             false
//         }
//     };

//     let boolean2 = hm
//         .get("abc")
//         .and_then(|hm| hm.get("123"))
//         .map(|inner_hm| inner_hm.contains(&4))
//         .unwrap_or(false);
//     println!("{boolean}");

//     println!("{boolean2}");
// }

// 30. change_context()

// use error_stack::{IntoReport, Result, ResultExt};
// use thiserror::Error;

// #[derive(Debug, Error)]
// enum CustomError {
//     #[error("could not parse value")]
//     ParseError,
//     #[error("could not add vec values")]
//     SummationError,
//     #[error("Error unknown")]
//     UnknownError,
// }
// fn main() {
//     let vec = vec!["1", "2", "-8"];
//     let sum = summation(vec).change_context(CustomError::SummationError);
//     println!("{sum:?}");
// }

// fn summation(vec: Vec<&str>) -> Result<i32, CustomError> {
//     let mut sum = 0;
//     for &ele in vec.iter() {
//         let int_val = parse_str(ele)?;
//         sum += int_val;
//     }
//     if sum > 0 {
//         Ok(sum)
//     } else {
//         Err(CustomError::UnknownError.into()) // Converts CustomError into Report<CustomError>
//     }
// }

// fn parse_str(ele: &str) -> Result<i32, CustomError> {
//     ele.parse::<i32>()
//         .into_report()
//         .change_context(CustomError::ParseError)
//         .attach_printable_lazy(|| format!("Pass integer value to parse"))
// }

// 31. serde_json

// use serde::{Deserialize, Serialize};
// #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
// struct Student {
//     id: i32,
//     name: String,
//     branch: Branch,
// }

// #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
// enum Branch {
//     CSE,
//     EC,
//     IT,
// }
// fn main() {
//     let stud1 = Student {
//         id: 1,
//         name: "Chethan".to_string(),
//         branch: Branch::CSE,
//     };

//     let json_val = serde_json::to_value(stud1.clone()).unwrap();
//     println!("{json_val:?}");

//     let original_struct = serde_json::from_value::<Student>(json_val.clone()).unwrap();
//     println!("{:#?}", original_struct);
//     assert_eq!(stud1, original_struct);

//     let str = r#"{"id": 2,"name": "Chandana", "branch": "IT"}"#;
//     let val = serde_json::from_str(str).unwrap();
//     let stud2 = serde_json::from_value::<Student>(val).unwrap();
//     println!("{:#?}", stud2);
// }

// 32. Insert values of different types inside vec

// use std::vec;

// trait Shape {
//     fn calculate_area(&self) -> i32;
// }
// struct Circle {
//     radius: i32,
// }

// impl Shape for Circle {
//     fn calculate_area(&self) -> i32 {
//         return 3 * self.radius * self.radius;
//     }
// }

// struct Rec {
//     h: i32,
//     b: i32,
// }

// impl Shape for Rec {
//     fn calculate_area(&self) -> i32 {
//         return self.h * self.b;
//     }
// }
// fn main() {
//     let cir1 = Box::new(Circle { radius: 3 });
//     let rec1 = Box::new(Rec { h: 4, b: 3 });
//     let vec: Vec<Box<dyn Shape>> = vec![cir1, rec1];
//     for item in vec.iter() {
//         let val = item.calculate_area();
//         println!("{val}");
//     }
// }

// 33. Implement max_Value() for Vec<T> where T can be any type

// trait MaxValue {
//     type Item;
//     fn max_value(&self) -> Option<&Self::Item>;
// }

// impl<T: PartialOrd + Ord> MaxValue for Vec<T> {
//     type Item = T;
//     fn max_value(&self) -> Option<&Self::Item> {
//         self.iter().max()
//     }
// }
// fn main() {
//     let vec1 = vec![1, 2, 3];
//     let vec2 = vec!["apple", "banana", "cherry"];

//     println!("{:?}", vec1.max_value());
//     println!("{:?}", vec2.max_value());
// }
// }

// impl Shape for Circle {
//     fn calculate_area(&self) -> i32 {
//         return 3 * self.radius * self.radius;
//     }
// }

// struct Rec {
//     h: i32,
//     b: i32,
// }

// impl Shape for Rec {
//     fn calculate_area(&self) -> i32 {
//         return self.h * self.b;
//     }
// }
// fn main() {
//     let cir1 = Box::new(Circle { radius: 3 });
//     let rec1 = Box::new(Rec { h: 4, b: 3 });
//     let vec: Vec<Box<dyn Shape>> = vec![cir1, rec1];
//     for item in vec.iter() {
//         let val = item.calculate_area();
//         println!("{val}");
//     }
// }

// 33. Implement max_Value() for Vec<T> where T can be any type

// trait MaxValue {
//     type Item;
//     fn max_value(&self) -> Option<&Self::Item>;
// }

// impl<T: PartialOrd + Ord> MaxValue for Vec<T> {
//     type Item = T;
//     fn max_value(&self) -> Option<&Self::Item> {
//         self.iter().max()
//     }
// }
// fn main() {
//     let vec1 = vec![1, 2, 3];
//     let vec2 = vec!["apple", "banana", "cherry"];

//     println!("{:?}", vec1.max_value());
//     println!("{:?}", vec2.max_value());
// }

// 34. dynamic dispatch makes a type to forget its own implemented function
// trait Animal {
//     fn shout(&self);
// }

// #[derive(Debug)]
// struct Dog;

// impl Dog {
//     fn walk(&self) {
//         println!("Dog is walking");
//     }
// }
// #[derive(Debug)]
// struct Fish;

// impl Fish {
//     fn swim(&self) {
//         println!("Fish is swimming");
//     }
// }

// impl Animal for Dog {
//     fn shout(&self) {
//         println!("Dog is shouting");
//     }
// }
// impl Animal for Fish {
//     fn shout(&self) {
//         println!("Fish is shouting");
//     }
// }

// fn main() {
//     let animal = return_an(1);
//     animal.shout();
// When Dog and Fish turn into Animal, they all forget how to walk and swim respoectively, only remember how to shout.
// So, the code below will cause an error.
//     // animal.walk; // This is type "&dyn Animal", in vtable, there will only be trait methods implemented on this, not the walk() or swim()
// }

// fn return_an(i: i32) -> &'static dyn Animal {
//     if i == 1 {
//         &Dog
//     } else {
//         &Fish
//     }
// }

// 35. Deref
// use std::ops::Deref;

// /* Make it work with least amount of changes*/
// fn main() {
//     let animal = Animal {
//         cat: Cat {
//             name: "abc".to_string(),
//             color: "red".to_string(),
//         },
//     };
//     let name = &animal.name;
//     println!("{name:?}");
// }

// struct Animal {
//     cat: Cat,
// }

// impl Deref for Animal {
//     type Target = Cat;
//     fn deref(&self) -> &Self::Target {
//         &self.cat
//     }
// }

// struct Cat {
//     name: String,
//     color: String,
// }

// 36. Deref - 2
// use std::ops::Deref;

// struct Abc {
//     x: i32,
// }
// fn main() {
//     let a = Abc { x: 5 };
//     println!("{}", *a);
// }

// impl Deref for Abc {
//     type Target = i32;
//     fn deref(&self) -> &Self::Target {
//         &self.x
//     }
// }

// 38. serde - attribute - rename_all (PascalCase, snake_case, camelCase, SCREAMING_SNAKE_CASE, lowercase, UPPERCASE)
// The rename_all attribute allows you to specify how field names should be transformed during serialization and deserialization.

// use serde::{Deserialize, Serialize};
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// struct Person {
//     first_name: String,
//     last_name: String,
//     age: u32,
// }

// fn main() {
//     let chethan = Person {
//         first_name: "Chethan".to_string(),
//         last_name: "Rao".to_string(),
//         age: 21,
//     };
//     let serialized_chethan = serde_json::to_value(chethan).unwrap(); // Serialization
//     println!("{serialized_chethan:?}");

//     let deserialized_chethan = serde_json::from_value::<Person>(serialized_chethan).unwrap(); // Deserialization
//     println!("{deserialized_chethan:?}");
// }

// 39. serde - attribute - flatten()
// The flatten attribute allows you to flatten nested structures during serialization and deserialization.

// use serde::{Deserialize, Serialize};
// #[derive(Serialize, Deserialize, Debug)]
// struct Address {
//     street: String,
//     city: String,
//     country: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Person {
//     name: String,
//     age: u32,
//     #[serde(flatten)]
//     address: Address,
// }

// fn main() {
//     let chethan = Person {
//         name: "Chethan".to_string(),
//         age: 21,
//         address: Address {
//             street: "1st cross".to_string(),
//             city: "Bangalore".to_string(),
//             country: "India".to_string(),
//         },
//     };

//     let serialized_chethan = serde_json::to_value(chethan).unwrap(); // Serialization
//     println!("{serialized_chethan:?}");

//     let deserialized_chethan = serde_json::from_value::<Person>(serialized_chethan).unwrap(); // Deserialization
//     println!("{deserialized_chethan:?}");
// }

// 40. serde - rename - particular variant to another specified value during ser and deser. It over rides the global attribute as below
// use serde::{Deserialize, Serialize};
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// struct Person {
//     #[serde(rename = "f_name")]
//     first_name: String,
// }

// fn main() {
//     let chethan = Person {
//         first_name: "Chethan".to_string(),
//     };
//     let serialized_chethan = serde_json::to_value(chethan).unwrap(); // Serialization
//     println!("{serialized_chethan:?}");

//     let deserialized_chethan = serde_json::from_value::<Person>(serialized_chethan).unwrap(); // Deserialization
//     println!("{deserialized_chethan:?}");
// }

// 41. serde - tag and untagged (only for enums)
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// // #[serde(tag = "type")]  ->  Will give enum name a type. i.e., {type: Wallet, amount: 4}
// // #[serde(untagged)]  ->  Will not include the enum Name in serialized Value.
// enum Payment {
//     Wallet { amount: i32 },
//     Card { amount: i32 },
// }
// fn main() {
//     let pay = Payment::Wallet { amount: 4 };
//     let ser_pay = serde_json::to_value(pay).unwrap();
//     println!("{ser_pay:?}");
// }

// 37. Strum
// use std::str::FromStr;

// #[derive(
//     // strum::Display -> enables to_string() on enum variants
//     // strum::EnumString -> enables from_str() on enum variants
//     Debug,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
//     strum::Display,
//     strum::EnumString,
// )]
// // #[strum(serialize_all = "case_style")] attribute can be used to change the case used when serializing to and deserializing from "strings".
// #[strum(serialize_all = "snake_case")]
// #[strum(ascii_case_insensitive)] // Supports 'Success' or 'sUCCesS'
// pub enum RefundType {
//     // matches on both, if anyone is satisfied, InstantRefund will be returned
//     #[strum(serialize = "InstantRefund", serialize = "instant_refund")]
//     InstantRefund,
//     #[default]
//     RegularRefund,
//     RetryRefund,
//     Success,
// }

// fn main() {
//     let a1 = RefundType::from_str("instant_refund").unwrap();
//     let a2 = RefundType::from_str("InstantRefund").unwrap();
//     println!("{a1:?} {a2:?}"); // Both r same

//     let b = RefundType::from_str("regular_refund").unwrap();
//     println!("{b:?}");

//     let c = RefundType::RetryRefund.to_string(); // We get in snake_case
//     println!("{c}");
// }

// 42 .

// TODO: remove this when you're done with your implementation.
// #![allow(unused_imports, unused_variables, dead_code)]

// pub trait Widget {
//      Natural width of `self`.
//     fn width(&self) -> usize;

//      Draw the widget into a buffer.
//     fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

//      Draw the widget on standard output.
//     fn draw(&self) {
//         let mut buffer = String::new();
//         self.draw_into(&mut buffer);
//         println!("{buffer}");
//     }
// }

// pub struct Label {
//     label: String,
// }

// impl Label {
//     fn new(label: &str) -> Label {
//         Label {
//             label: label.to_owned(),
//         }
//     }
// }

// pub struct Button {
//     label: Label,
//     callback: Box<dyn FnMut()>,
// }

// impl Button {
//     fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
//         Button {
//             label: Label::new(label),
//             callback,
//         }
//     }
// }

// pub struct Window {
//     title: String,
//     widgets: Vec<Box<dyn Widget>>,
// }

// impl Window {
//     fn new(title: &str) -> Window {
//         Window {
//             title: title.to_owned(),
//             widgets: Vec::new(),
//         }
//     }

//     fn add_widget(&mut self, widget: Box<dyn Widget>) {
//         self.widgets.push(widget);
//     }

//     fn inner_width(&self) -> usize {
//         std::cmp::max(
//             self.title.chars().count(),
//             self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
//         )
//     }
// }

// impl Widget for Label {
//     fn width(&self) -> usize {
//         unimplemented!()
//     }

//     fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
//         unimplemented!()
//     }
// }

// impl Widget for Button {
//     fn width(&self) -> usize {
//         unimplemented!()
//     }

//     fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
//         unimplemented!()
//     }
// }

// impl Widget for Window {
//     fn width(&self) -> usize {
//         unimplemented!()
//     }

//     fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
//         unimplemented!()
//     }
// }

// fn main() {
//     let mut window = Window::new("Rust GUI Demo 1.23");
//     window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
//     window.add_widget(Box::new(Button::new(
//         "Click me!",
//         Box::new(|| println!("You clicked the button!")),
//     )));
//     window.draw();
// }

// fn main() {
//     let s = RefundType {
//         shipping: ShippingAddress {
//             address: "ban".to_string()
//         }
//     };

// }

// #[derive(
//     // strum::Display -> enables to_string() on enum variants
//     // strum::EnumString -> enables from_str() on enum variants
//     Debug,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
// )]

// pub struct RefundType {
//     // matches on both, if anyone is satisfied, InstantRefund will be returned
//     shipping: ShippingAddress
// }

// #[derive(
//     // strum::Display -> enables to_string() on enum variants
//     // strum::EnumString -> enables from_str() on enum variants
//     Debug,
//     Default,
//     serde::Serialize,
//     serde::Deserialize,
// )]
// struct ShippingAddress {
//     address: String
// }

//  43. Pass closure as parameter to another function
// fn main() {
//     fun2(fun);
// }

// fn fun() {
//     println!("Hello");
// }

// fn fun2<F>(fun: F)
// where
//     F: Fn(),
// {
//     fun();
// }

// 44. Pass closure which accepts parameters to another function
// fn main() {
//     fun(5, |a, b| println!("Hello {a} {b}"));
// }

// fn fun<F>(a: i32, func: F)
// where
//     F: Fn(i32,i32),
// {
//     func(a,a)
// }

// fn main() {
//     let a = None;
//     let b = Some(6);
//     let res = a.unwrap_or(b.unwrap());
//     println!("{res:?}");
// }

// fn main() {
//     let mut a = vec![1, 2, 3]; // Your vector a
//     let max = 5; // The desired length of vector a
//     let b = vec![2, 3, 4, 5, 6, 7]; // Your vector b

//     // Filter out unique elements from b and add them to a until a.len() == max
//     let unique_elements = b.iter().filter(|&x| !a.contains(x)).take(max - a.len());

//     a.extend(unique_elements);

//     println!("{:?}", a);
// }

// struct A {
//     a: i32,
//     b: i32,
// }

// struct B {
//     a: i32,
// }

// impl From<B> for A {
//     fn from(value: B) -> Self {
//         Self { a: value.a, b: 0 }
//     }
// }
// fn main() {
//     let b = B { a: 4 };
//     let a = A::from(b);
//     let c = B::from(a);
// }

// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize)]
// struct InnerJson {
//     #[serde(rename = "inner_field.field1")]
//     field1: String,
//     #[serde(rename = "inner_field.field2")]
//     field2: i32,
// }

// #[derive(Serialize, Deserialize)]
// struct OuterJson {
//     outer_field: String,
//     #[serde(flatten)]
//     inner_field: InnerJson,
// }

// fn main() {
//     // Create an instance of InnerJson
//     let inner = InnerJson {
//         field1: "value1".to_string(),
//         field2: 42,
//     };

//     // Create an instance of OuterJson with the inner JSON struct
//     let outer = OuterJson {
//         outer_field: "outer_value".to_string(),
//         inner_field: inner,
//     };

//     // Serialize the outer JSON object to a JSON string
//     let json_str = serde_json::to_string(&outer).expect("Failed to serialize to JSON");

//     println!("{}", json_str);
// }

// In OSS
// struct Oss;

// // In VAS
// struct Vas;

// // In OSS
// trait ExtraThings {
//     fn extra_fun();
// }

// // In OSS
// impl ExtraThings for Oss {
//     fn extra_fun() {
//         println!("OSS");
//     }
// }

// // In VAS
// impl ExtraThings for Vas {
//     fn extra_fun() {
//         println!("VAS");
//     }
// }

// // In OSS
// fn make_pm_data<Ctx: ExtraThings>() {
//     Ctx::extra_fun();
// }

// fn main() {
//     make_pm_data::<Vas>();
// }

fn main() {}
