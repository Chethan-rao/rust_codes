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

////////////////////////////////////////////////////////////////////////////////////

// use enum_dispatch::enum_dispatch;

// #[enum_dispatch]
// trait KnobControl {
//     fn get_value(&self) -> f64;
// }

// #[derive(Debug)]
// struct LinearKnob;

// impl KnobControl for LinearKnob {
//     fn get_value(&self) -> f64 {
//         1.0
//     }
// }

// #[derive(Debug)]
// struct LogarithmicKnob;

// impl KnobControl for LogarithmicKnob {
//     fn get_value(&self) -> f64 {
//         2.0
//     }
// }

// #[enum_dispatch(KnobControl)]
// #[derive(Debug)]

// enum Knob {
//     LinearKnob,
//     LogarithmicKnob,
// }

// fn main() {
//     let a = Knob::from(LinearKnob);

//     println!("{:?}", a)
// }

////////////////////////////////////////////////////////////////////////////////////

// TypeState pattern

// struct Settings {
//     db_password: String,
// }
// struct Decryptable<S: State> {
//     state: Settings,
//     marker: std::marker::PhantomData<S>,
// }

// /// decrypt valid only in Encrypted state.
// impl Decryptable<Encrypted> {
//     fn decrypt(self) -> Decryptable<Raw> {
//         Decryptable {
//             state: Settings {
//                 db_password: "decrypted".to_string(),
//             },
//             marker: std::marker::PhantomData,
//         }
//     }
// }

// impl<S: State> Decryptable<S> {
//     fn inner_value(self) -> Settings {
//         self.state
//     }
// }

// enum Encrypted {}
// enum Raw {}

// trait State {}
// impl State for Encrypted {}
// impl State for Raw {}

// fn main() {
//     // parsed as encrypted value
//     let parsed_config: Decryptable<Encrypted> = Decryptable {
//         state: Settings {
//             db_password: "encrypted".to_string(),
//         },
//         marker: std::marker::PhantomData,
//     };

//     let raw_decryptable = parsed_config.decrypt();

//     let decrypted_settings = raw_decryptable.inner_value();

//     println!("{:?}", decrypted_settings.db_password); // decrypted
// }

////////////////////////////////////////////////////////////////////////////////////

// use std::marker::PhantomData;

// trait EncryptionState {}
// #[derive(Clone, Debug)]

// struct Decrypted {}

// #[derive(Clone, Debug)]
// struct Encrypted {}

// impl EncryptionState for Decrypted {}
// impl EncryptionState for Encrypted {}

// // Placeholder KMS client
// struct KMSClient {}

// impl KMSClient {
//     fn decrypt(&self, _: String) -> String {
//         "decrypted".to_string()
//     }
// }

// #[derive(Debug, Clone)]
// struct Decryptable<T, S: EncryptionState> {
//     inner: T,
//     marker: PhantomData<S>,
// }

// impl<T> Decryptable<T, Encrypted> {
//     fn decrypt(mut self, decryptor_fn: impl FnOnce(T) -> T) -> Decryptable<T, Decrypted> {
//         self.inner = decryptor_fn(self.inner);
//         Decryptable {
//             inner: self.inner,
//             marker: PhantomData,
//         }
//     }
// }

// trait Decryption
// where
//     Self: Sized,
// {
//     fn decrypt(
//         value: Decryptable<Self, Encrypted>,
//         kms_client: &KMSClient,
//     ) -> Decryptable<Self, Decrypted>;
// }

// #[derive(Clone, Debug)]
// struct Database {
//     dbname: String,
//     password: String,
// }

// impl Decryption for Database {
//     fn decrypt(
//         value: Decryptable<Self, Encrypted>,
//         kms_client: &KMSClient,
//     ) -> Decryptable<Self, Decrypted> {
//         value.decrypt(|a| Database {
//             dbname: a.dbname,
//             password: kms_client.decrypt(a.password),
//         })
//     }
// }

// #[derive(Debug)]
// struct Settings<S: EncryptionState> {
//     db: Decryptable<Database, S>,
// }

// fn main() {
//     let db = Database {
//         dbname: "hyper".to_string(),
//         password: "encrypted".to_string(),
//     };

//     let decryptable_db: Decryptable<Database, Encrypted> = Decryptable {
//         inner: db.clone(),
//         marker: PhantomData,
//     };

//     let mut state = Settings {
//         db: decryptable_db.clone(),
//     };

//     let decrypted_val = Database::decrypt(decryptable_db, &KMSClient {});

//     let decrypted_settings = Settings { db: decrypted_val };
// }

////////////////////////////////////////////////////////////

// struct AwsKmsClient;

// impl AwsKmsClient {
//     fn encrypt(&self, data: String) {}

//     fn decrypt(&self, data: String) {}
// }

// struct HcVaultClient;

// impl HcVaultClient {
//     fn fetch(&self, data: String) {}
// }

// pub trait SecretsManagementInterface {
//     fn store_secret(&self, input: String);

//     fn get_secret(&self, input: String);
// }

// impl SecretsManagementInterface for AwsKmsClient {
//     fn store_secret(&self, input: String) {
//         self.encrypt(input)
//     }

//     fn get_secret(&self, input: String) {
//         self.decrypt(input)
//     }
// }

// impl SecretsManagementInterface for HcVaultClient {
//     fn store_secret(&self, input: String) {
//         unimplemented!()
//     }

//     fn get_secret(&self, input: String) {
//         self.fetch(input)
//     }
// }

// struct AppState {
//     pub secrets_manager_client: Box<dyn SecretsManagementInterface>,
// }

// fn main() {}

////////////////////////////////////////////////////////////////////////////////////////////////////

// use std::marker::PhantomData;

// #[derive(Debug)]
// struct Settings<S: State> {
//     db_password: String,
//     state: PhantomData<S>,
// }

// trait State {}

// #[derive(Debug)]
// struct Encrypted;
// impl State for Encrypted {}

// #[derive(Debug)]
// struct Decrypted;
// impl State for Decrypted {}

// impl Settings<Encrypted> {
//     fn decrypt(self) -> Settings<Decrypted> {
//         Settings {
//             db_password: self.db_password, // decrypt password and assign it
//             state: PhantomData::<Decrypted>,
//         }
//     }
// }

// impl<S: State + std::fmt::Debug> Settings<S> {
//     fn print(&self) {
//         println!("Settings: {:?}", self);
//     }
// }

// fn main() {
//     let config = Settings {
//         db_password: "db_pass".to_string(),
//         state: PhantomData::<Encrypted>,
//     };
//     config.print();

//     let raw_config = config.decrypt();
//     raw_config.print();
// }

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

// // 45. Tokio spawn
// #[tokio::main]
// async fn main() {
//     let ops = vec![1, 2, 3];
//     let mut tasks = Vec::with_capacity(ops.len());
//     for op in ops {
//         // This call will make them start running in the background
//         // immediately.
//         tasks.push(tokio::spawn(my_background_op(op)));
//     }

//     let mut outputs = Vec::with_capacity(tasks.len());
//     for task in tasks {
//         outputs.push(task.await.unwrap());
//     }
//     println!("{:?}", outputs);
// }

// async fn my_background_op(id: i32) -> String {
//     let s = format!("Starting background task {}.", id);
//     println!("{}", s);
//     s
// }

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

// 46. When to use Box smart pointer -
// - When u have variable with a trait type whose size cannot be computed at compile time, wrap it in Box
// - Recursive data types

// trait Vehicle {
//     fn drive(&self);
// }
// struct Truck;

// impl Vehicle for Truck {
//     fn drive(&self) {
//         println!("Truck is driving");
//     }
// }

// fn main() {
//     let truck: Box<dyn Vehicle>;

//     truck = Box::new(Truck);
//     truck.drive();
// }

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

// 47. When to use Rc smart pointer (Gives shared ownership to a value)-
// In case of normal reference (&), when owner goes out of scope, u can't use those reference anymore
// But in case of Rc, even though owner goes out of scope or moved, u can still use it until Rc count goes to 0 (refer below)
// Basically rust has rule like - only one owner can exists for a variable
// But in Rc, every variable is a owner of that value, and the value will only be deallocated when every variable goes out of scope.

// use std::rc::Rc;

// struct Truck {
//     cap: i32,
// }

// fn main() {
//     // No. of ref to this object - 1
//     let truck1 = Rc::new(Truck { cap: 5 });

// No. of ref incremented to +1 - 2
// let truck2 = Rc::clone(&truck1);
// println!("{:?}", Rc::strong_count(&truck1)); // prints 2. still will be 2 if u took strong count of &truck2
//                                              // Ownership is transferred still No. of ref remains same - 2
// let truck3 = truck1;
// // No. of ref is still 2. still will be 2 if u took strong count of &truck3.
// println!("{:?}", Rc::strong_count(&truck2)); // prints 2
//                                              // One reference got dropped so now no. of ref - 1
// std::mem::drop(truck3);
// // No. of ref is now decremented so - 1
// println!("{:?}", Rc::strong_count(&truck2)); // prints 1
// }

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

// 48. When to use Arc (Atomically referenced counter)
// Rc doesn't implement Send (safe to send value to another thread) nor Sync (safe to share value between threads) traits.
// Assume value `A` allocated in heap. thread1 holds RC ref to `A` and thread2 also holds RC ref to `A`.
// So currently count of reference to `A` is 2. if both threads try to clone using Rc::clone, there will be data race
// and instead of count becoming 4, it might become 3.
// So use Arc instead which implements both Send and Sync trait.

// But Arc doesn't support mutability of inner value as multiple owners shares this value
// So u can wrap the inner value with `Mutex` first and then with Arc. Example Arc::new(Mutex::new(5));
// Now, even though `5` can be shared with multiple owners, only one thread can mutate it at a time

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

// 49. Closure example

// Type which lives only in main function
// struct AppState {
//     db: String
// }

// impl AppState {
//     fn alter(&self) -> String {
//         format!("{}_innerval",self.db)
//     }
// }

// fn main() {
//     // Lives in main function
//     let state = AppState { db: "samn".to_string() };

//     // Piece of code which has to be executed in helper function, which requires state as argument
//     // but helper function doesn't take `state` as argument. So create a closure within main
//     // and write all code which has to be executed in helper function. Pass this closure as argument
//     // to helper function
//     let closure = || {
//         state.alter()
//     };

//     helper(closure);
// }

// fn helper<F>(f: F) where F: Fn() -> String {
//     let res = f();
//     println!("{res:?}");
// }

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

// 49. Closure example 2

// fn main() {
//     let mut abc = Abc { val: 19 };

//     // Executes map closure, if inner.val == 10
//     abc.mutate_if_taking_mut_sef(|inner| inner.val == 10, |inner| inner.val = 15);

//     println!("{abc:?}");

//     let abc = abc.mutate_if_without_mut_sef(
//         |inner| inner.val == 10,
//         |mut inner| {
//             inner.val = 15;
//             inner
//         },
//     );

//     println!("{abc:?}");

//     let abc = abc.mutate_if_in_one_closur(|mut inner| {
//         if inner.val == 10 {
//             inner.val = 15;
//         }
//         inner
//     });

//     println!("{abc:?}");
// }

// #[derive(Debug, Clone)]
// struct Abc {
//     val: i32,
// }

// impl Abc {
//     fn mutate_if_taking_mut_sef<F, G>(&mut self, func1: F, func2: G)
//     where
//         F: FnOnce(&Self) -> bool,
//         G: FnOnce(&mut Self),
//     {
//         if func1(self) {
//             func2(self);
//         }
//     }

//     fn mutate_if_without_mut_sef<F, G>(self, func1: F, func2: G) -> Self
//     where
//         F: FnOnce(&Self) -> bool,
//         G: FnOnce(Self) -> Self,
//     {
//         if func1(&self) {
//             return func2(self);
//         }
//         self
//     }

//     fn mutate_if_in_one_closur<F>(self, func1: F) -> Self
//     where
//         F: FnOnce(Self) -> Self,
//     {
//         func1(self)
//     }
// }

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

// 50. Custom Option methods impl

// #[allow(dead_code)]
// #[derive(Debug, PartialEq)]
// enum MyOption<T> {
//     Some(T),
//     None,
// }

// use MyOption::{None, Some};

// #[allow(dead_code)]
// impl<T> MyOption<T> {
//     fn is_some(&self) -> bool {
//         matches!(self, Some(_))
//     }

//     fn is_some_and<F>(self, func: F) -> bool
//     where
//         F: FnOnce(T) -> bool,
//     {
//         match self {
//             Some(inner) => func(inner),
//             None => false,
//         }
//     }

//     fn as_ref(&self) -> MyOption<&T> {
//         match self {
//             Some(inner) => Some(inner),
//             None => None,
//         }
//     }

//     fn unwrap_or(self, default: T) -> T {
//         match self {
//             Some(x) => x,
//             None => default,
//         }
//     }

//     fn unwrap_or_else<F>(self, func: F) -> T
//     where
//         F: FnOnce() -> T,
//     {
//         match self {
//             Some(x) => x,
//             None => func(),
//         }
//     }

//     fn unwrap_or_default(self) -> T
//     where
//         T: Default,
//     {
//         match self {
//             Some(x) => x,
//             None => T::default(),
//         }
//     }

//     fn map<U, F>(self, func: F) -> MyOption<U>
//     where
//         F: FnOnce(T) -> U,
//     {
//         match self {
//             Some(x) => Some(func(x)),
//             None => None,
//         }
//     }

//     fn inspect<F>(self, func: F) -> Self
//     where
//         F: FnOnce(&T),
//     {
//         if let Some(ref x) = self {
//             func(x);
//         }

//         self
//     }

//     fn map_or<U, F>(self, default: U, func: F) -> U
//     where
//         F: FnOnce(T) -> U,
//     {
//         match self {
//             Some(x) => func(x),
//             None => default,
//         }
//     }

//     fn map_or_else<U, F, G>(self, default: G, func: F) -> U
//     where
//         F: FnOnce(T) -> U,
//         G: FnOnce() -> U,
//     {
//         match self {
//             Some(x) => func(x),
//             None => default(),
//         }
//     }

//     fn ok_or<E>(self, err: E) -> Result<T, E> {
//         match self {
//             Some(x) => Ok(x),
//             None => Err(err),
//         }
//     }

//     fn ok_or_else<F, E>(self, func: F) -> Result<T, E>
//     where
//         F: FnOnce() -> E,
//     {
//         match self {
//             Some(x) => Ok(x),
//             None => Err(func()),
//         }
//     }

//     fn and<U>(self, next: MyOption<U>) -> MyOption<U> {
//         match self {
//             Some(_) => next,
//             None => None,
//         }
//     }

//     fn and_then<U, F>(self, next_func: F) -> MyOption<U>
//     where
//         F: FnOnce(T) -> MyOption<U>,
//     {
//         match self {
//             Some(x) => next_func(x),
//             None => None,
//         }
//     }

//     fn filter<P>(self, predicate: P) -> Self
//     where
//         P: FnOnce(&T) -> bool,
//     {
//         if let Some(x) = self {
//             if predicate(&x) {
//                 return Some(x);
//             }
//         }
//         None
//     }

//     fn or(self, default: Self) -> Self {
//         match self {
//             val @ Some(_) => val,
//             None => default,
//         }
//     }

//     fn or_else<F>(self, default: F) -> Self
//     where
//         F: FnOnce() -> Self,
//     {
//         match self {
//             val @ Some(_) => val,
//             None => default(),
//         }
//     }

//     fn xor(self, other: Self) -> Self {
//         match (self, other) {
//             (val @ Some(_), None) => val,
//             (None, val @ Some(_)) => val,
//             _ => None,
//         }
//     }

//     fn take(self) -> Self {
//         self
//     }
// }

// fn main() {
//     let x: MyOption<i32> = Some(5);
//     let y = x.take();
//     println!("{y:?}");
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////

// 51. Custom Result methods impl

// #[allow(dead_code)]
// #[derive(Debug)]
// pub enum MyResult<T, E> {
//     Ok(T),
//     Err(E),
// }

// #[allow(dead_code)]
// use MyResult::{Err, Ok};

// #[allow(dead_code)]
// impl<T, E> MyResult<T, E> {
//     fn is_ok(&self) -> bool {
//         matches!(self, Ok(_))
//     }

//     fn is_ok_and<F>(self, func: F) -> bool
//     where
//         F: FnOnce(T) -> bool,
//     {
//         match self {
//             Ok(x) => func(x),
//             Err(_) => false,
//         }
//     }

//     fn is_err_and<F>(self, func: F) -> bool
//     where
//         F: FnOnce(E) -> bool,
//     {
//         match self {
//             Ok(_) => false,
//             Err(e) => func(e),
//         }
//     }
//     fn ok(self) -> Option<T> {
//         match self {
//             Ok(x) => Some(x),
//             Err(_) => None,
//         }
//     }

//     fn as_ref(&self) -> MyResult<&T, &E> {
//         match *self {
//             Ok(ref x) => Ok(x),
//             Err(ref e) => Err(e),
//         }
//     }

//     fn map<F, U>(self, func: F) -> MyResult<U, E>
//     where
//         F: FnOnce(T) -> U,
//     {
//         match self {
//             Ok(x) => Ok(func(x)),
//             Err(e) => Err(e),
//         }
//     }

//     fn and_then<F, U>(self, func: F) -> MyResult<U, E>
//     where
//         F: FnOnce(T) -> MyResult<U, E>,
//     {
//         match self {
//             Ok(x) => func(x),
//             Err(e) => Err(e),
//         }
//     }
// }

// fn main() {
//     let res: MyResult<i32, i32> = Ok(5);

//     let x = res.and_then(|inner| Ok(inner + 1));

//     println!("{x:?}");
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////

// 52. Vector with at least single element

// use std::fmt::Debug;

// struct Vector<T> {
//     first: T,
//     rest: Vec<T>,
// }

// #[macro_export]
// macro_rules! vec {
//     ($first:expr) => {
//         Vector::new($first)
//     };

//     ($first:expr, $($rest:expr),*) => {{
//         let mut vector = Vector::new($first);
//         $(
//             vector.push($rest);
//         )*
//         vector
//     }};
// }

// impl<T> Vector<T> {
//     fn new(first: T) -> Self {
//         Self {
//             first,
//             rest: Vec::default(),
//         }
//     }

//     fn push(&mut self, element: T) {
//         self.rest.push(element);
//     }
// }

// impl<T: Debug + Clone> Debug for Vector<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let first = self.first.clone();
//         let mut vector = Vec::default();
//         vector.push(first);
//         vector.extend(self.rest.clone());

//         Debug::fmt(&vector, f)
//     }
// }

// fn main() {
//     let mut vecc = vec![1];
//     vecc.push(2);
//     vecc.push(3);
//     println!("{vecc:?}");

//     let vecc = vec![1, 2];
//     println!("{vecc:?}");

//     // Error
//     // let vecc = vec![];
//     // println!("{vecc:?}");
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////

// 53. Custom Secret impl

// use std::marker::PhantomData;

// fn main() {
//     let a = Example { a: Secret::new(4) };

//     println!("{a:?}");
// }

// #[derive(Debug)]
// struct Example {
//     a: Secret<i32>,
// }

// struct Secret<T, Strategy = WithType>
// where
//     Strategy: StrategyTrait,
// {
//     pub inner_secret: T,
//     pub strategy: PhantomData<Strategy>,
// }

// impl<T> Secret<T> {
//     fn new(ele: T) -> Self {
//         Secret {
//             inner_secret: ele,
//             strategy: PhantomData,
//         }
//     }
// }

// impl<T> std::fmt::Debug for Secret<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "** {} **", std::any::type_name::<T>())
//     }
// }

// trait StrategyTrait {}

// pub enum WithType {}

// impl StrategyTrait for WithType {}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////

// 54. PhantomData usecase

// #[derive(Debug)]
// struct MyType<T>
// where
//     T: SecuredState,
// {
//     field: i32,
//     type1: std::marker::PhantomData<T>,
// }

// impl<T> MyType<T>
// where
//     T: SecuredState,
// {
//     fn new(field: i32) -> Self {
//         Self {
//             field,
//             type1: std::marker::PhantomData,
//         }
//     }
// }

// trait SecuredState {}

// #[derive(Debug)]
// struct RawState;

// #[derive(Debug)]
// struct EncryptedState;

// impl SecuredState for RawState {}

// impl SecuredState for EncryptedState {}

// fn main() {
//     let mytype1: MyType = MyType::new(45);

//     println!("{mytype1:?}");
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////

// 55.

// fn main() {
//     let abc = Example {
//         f0: 5,
//         f1: None,
//         f2: None,
//     };

//     let res = serde_json::to_value(abc).unwrap();

//     println!("{res:?}");

//     let res1 = serde_json::from_value::<Example>(res).unwrap();
//     println!("{res1:?}");
// }

// trait ExampleTrait {
//     fn print();
// }

// mod private_module {
//     trait SealedTrait
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////

// struct Example<const N: i32 = 1> {
//     field: i32,
// }

// impl<const N: i32> Example<N> {
//     pub fn new(field: i32) -> Self {
//         let _ = <Self as AssertGt0>::VALID;

//         Self { field }
//     }
// }

// trait AssertGt0 {
//     const VALID: ();
// }

// impl<const N: i32> AssertGt0 for Example<N> {
//     const VALID: () = assert!(N > 0);
// }

// fn main() {
//     let a: Example<-1> = Example { field: 5 };
//     println!("{:?}", a.field);
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////

// Calling associated method

// #[derive(Debug)]
// struct Example(i32);

// trait ExampleTrait {
//     type AssociatedType;
//     fn associated_method();
// }

// impl ExampleTrait for Example {
//     type AssociatedType = String;

//     fn associated_method() {
//         println!("hello");
//     }
// }

// fn main() {
//     let example = Example(5);

//     <Example as ExampleTrait>::associated_method();
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////

// use std::{
//     collections::HashMap,
//     hash::{Hash, Hasher},
// };

// use rand::{prelude::Distribution, Rng, SeedableRng};

// fn main() {
//     use std::collections::hash_map;

//     use rand::distributions;
//     let mut res: HashMap<String, i32> = HashMap::default();

//     for _ in 0..20 {
//         let mut splits = vec![("braintree", 40), ("stripe", 10), ("adyen", 50)];

//         let weights = splits.iter().map(|(_, b)| b).collect::<Vec<_>>();

//         let rng_seed: Option<&str> = None;
//         let weighted_index = distributions::WeightedIndex::new(weights).unwrap();

//         let idx = if let Some(seed) = rng_seed {
//             let mut hasher = hash_map::DefaultHasher::new();
//             seed.hash(&mut hasher);
//             let hash = hasher.finish();

//             let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(hash);
//             weighted_index.sample(&mut rng)
//         } else {
//             let mut rng = rand::thread_rng();
//             weighted_index.sample(&mut rng)
//         };

//         splits.get(idx).unwrap();

//         // Panic Safety: We have performed a `get(idx)` operation just above which will
//         // ensure that the index is always present, else throw an error.
//         let removed = splits.remove(idx);
//         splits.insert(0, removed);

//         let cc = splits[0].0.to_string();
//         println!(">> {:?}", cc);

//         match res.get_mut(&cc) {
//             Some(val) => *val = *val + 1,
//             None => {
//                 res.insert(cc, 1);
//             }
//         };
//     }
//     println!(">> final {res:?}");
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////

// use common_utils::types::MinorUnit;
// use euclid::{
//     backend::{inputs, EuclidBackend, VirInterpreterBackend},
//     enums,
//     frontend::ast,
//     types::DummyOutput,
// };

// fn main() {
//     let program_str = r#"
//         default: ["stripe", "adyen"]

//         rule_1: ["stripe"]
//         {
//             payment_method = pay_later
//             or
//             amount = 34
//         }
//         "#;

//     let (_, program) = ast::parser::program::<DummyOutput>(program_str).expect("Program");
//     println!(">> {program:?}");
//     let inp = inputs::BackendInput {
//         metadata: None,
//         payment: inputs::PaymentInput {
//             amount: MinorUnit::new(32),
//             currency: enums::Currency::USD,
//             card_bin: None,
//             authentication_type: Some(enums::AuthenticationType::NoThreeDs),
//             capture_method: Some(enums::CaptureMethod::Automatic),
//             business_country: Some(enums::Country::UnitedStatesOfAmerica),
//             billing_country: Some(enums::Country::France),
//             business_label: None,
//             setup_future_usage: None,
//         },
//         payment_method: inputs::PaymentMethodInput {
//             payment_method: Some(enums::PaymentMethod::PayLater),
//             payment_method_type: Some(enums::PaymentMethodType::Affirm),
//             card_network: None,
//         },
//         mandate: inputs::MandateData {
//             mandate_acceptance_type: None,
//             mandate_type: None,
//             payment_type: None,
//         },
//     };

//     let backend = VirInterpreterBackend::<DummyOutput>::with_program(program).expect("Program");
//     let result = backend.execute(inp).expect("Execution");
//     println!(">> {result:?}");
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////

// use std::{
//     thread,
//     time::{self, Duration, Instant, SystemTime},
// };

// use serde_json::{json, Value};

// #[derive(Debug, serde::Deserialize, serde::Serialize)]
// struct LeakyBucket {
//     capacity: usize,
//     leak_rate: std::time::Duration,
//     level: usize,
//     last_leak: std::time::SystemTime,
// }

// impl LeakyBucket {
//     pub fn new(capacity: usize, leak_rate: Duration) -> Self {
//         Self {
//             capacity,
//             leak_rate,
//             level: 0,
//             last_leak: SystemTime::now(),
//         }
//     }

//     fn refresh(&mut self) {
//         let now = SystemTime::now();
//         let elapsed = now.duration_since(self.last_leak).unwrap();
//         let cl = elapsed.div_duration_f64(self.leak_rate).floor() as usize;

//         if cl > 0 {
//             self.last_leak = now;
//             self.level = self.level.saturating_sub(cl);
//         }
//     }

//     pub fn add(&mut self, amount: usize) -> bool {
//         self.refresh();

//         if self.level + amount <= self.capacity {
//             self.level += amount;
//             true
//         } else {
//             false
//         }
//     }
//     pub fn check_if_full(&mut self) -> bool {
//         self.refresh();
//         self.level >= self.capacity
//     }
// }
// fn main() {
//     let mut bucket = LeakyBucket::new(6, Duration::from_secs(6));

//     // for _ in 0..6 {
//     //     bucket.add(1);
//     // }

//     // thread::sleep(Duration::from_secs(6));
//     // bucket.refresh();
//     // dbg!(&bucket);
//     // for _ in 0..6 {
//     //     thread::sleep(Duration::from_secs(1));
//     //     bucket.refresh();
//     // }

//     // // dbg!(bucket);

//     println!("{}", serde_json::to_string(&bucket).unwrap());
// }

// pub(crate) fn get_current_time_in_secs() -> Duration {
//     std::time::SystemTime::now()
//         .duration_since(std::time::UNIX_EPOCH)
//         .unwrap()
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////

// use config::Config;
// use external_services::managers::secrets_management::SecretsManagementConfig;
// use hyperswitch_interfaces::secrets_interface::{
//     secret_handler::SecretsHandler,
//     secret_state::{RawSecret, SecretState, SecretStateContainer, SecuredSecret},
//     SecretManagementInterface, SecretsManagementError,
// };
// use masking::ExposeInterface;
// use serde::Deserialize;

// #[derive(Debug, Deserialize, Clone, Default)]
// #[serde(default)]
// pub struct Settings<S: SecretState> {
//     pub master_database: SecretStateContainer<Database, S>,
//     pub secrets_management: SecretsManagementConfig,
// }

// #[derive(Clone, Default, Deserialize, Debug)]
// struct Database {
//     password: String,
// }

// #[async_trait::async_trait]
// impl SecretsHandler for Database {
//     async fn convert_to_raw_secret(
//         value: SecretStateContainer<Self, SecuredSecret>,
//         secret_management_client: &dyn SecretManagementInterface,
//     ) -> error_stack::Result<SecretStateContainer<Self, RawSecret>, SecretsManagementError> {
//         let db = value.get_inner();
//         let db_password = secret_management_client
//             .get_secret(db.password.clone().into())
//             .await?;

//         Ok(value.transition_state(|db| Self {
//             password: db_password.expose(),
//             ..db
//         }))
//     }
// }

// #[tokio::main]
// async fn main() {
//     let settings = Config::builder()
//         .add_source(config::File::with_name(
//             "/Users/chethan.rao/playground/rust/config/dev.toml",
//         ))
//         .add_source(config::Environment::with_prefix("APP"))
//         .build()
//         .unwrap();

//     let secured_settings = settings
//         .try_deserialize::<Settings<SecuredSecret>>()
//         .unwrap();

//     let secret_management_client = secured_settings
//         .secrets_management
//         .get_secret_management_client()
//         .await
//         .unwrap();

//     let database = Database::convert_to_raw_secret(
//         secured_settings.master_database,
//         &*secret_management_client,
//     )
//     .await
//     .expect("Failed to decrypt database password");

//     println!(">> {database:?}");
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////

// // 56. MultiArmedBandit problem
// Run below commands if facing dependency issue
// python3 -m venv path/to/venv
// source path/to/venv/bin/activate
// python3 -m pip install pandas
// use rand::Rng;
// use std::io::Write;
// use std::process::Command;
// use std::{collections::HashMap, fs::File};

// #[derive(Debug)]
// struct ConnectorStats {
//     successful_txn: i64,
//     total_txn: u64,
// }

// struct MultiArmedBandit {
//     connectors: HashMap<String, ConnectorStats>,
//     total_pulls: u64,
//     exploration_constant: f64,
// }

// impl MultiArmedBandit {
//     fn new(exploration_constant: f64) -> Self {
//         MultiArmedBandit {
//             connectors: HashMap::new(),
//             total_pulls: 0,
//             exploration_constant,
//         }
//     }

//     fn add_connector(&mut self, name: &str) {
//         self.connectors.insert(
//             name.to_string(),
//             ConnectorStats {
//                 successful_txn: 0,
//                 total_txn: 0,
//             },
//         );
//     }

//     fn log_exploration_exploitation(
//         log_file: &mut File,
//         name: &str,
//         exploitation: f64,
//         exploration: f64,
//         txn_count: u64,
//     ) {
//         writeln!(
//             log_file,
//             "{},{},{},{}",
//             txn_count, name, exploitation, exploration
//         )
//         .expect("Failed to write to log file");
//     }

//     // fn log_selected_connector(log_file: &mut File, name: &str) {
//     //     writeln!(log_file, "selected_connector: {}", name).expect("Failed to write to log file");
//     // }

//     fn select_connector(&self, log_file: &mut File) -> Option<String> {
//         if self.total_pulls == 0 {
//             return self
//                 .connectors
//                 .iter()
//                 .filter(|(_, stats)| stats.total_txn == 0)
//                 .map(|(name, _)| name.clone())
//                 .next();
//         }

//         let connector = self
//             .connectors
//             .iter()
//             .map(|(name, stats)| {
//                 let exploitation = if stats.successful_txn == 0 && stats.total_txn == 0 {
//                     0.0
//                 } else {
//                     stats.successful_txn as f64 / stats.total_txn as f64
//                 };

//                 let exploration =
//                     (2.0 * (self.total_pulls as f64).ln() / stats.total_txn as f64).sqrt();
//                 let exploration = if exploration.is_nan() {
//                     0.0
//                 } else {
//                     let normalized_exploration = (exploration - 0.0) / (2.0 - 0.0);
//                     normalized_exploration * self.exploration_constant
//                 };

//                 let ucb = exploitation + exploration;
//                 println!(
//                     "{name}         - exploitation: {exploitation} exploration: {exploration}"
//                 );

//                 Self::log_exploration_exploitation(
//                     log_file,
//                     name,
//                     exploitation,
//                     exploration,
//                     self.total_pulls,
//                 );

//                 (name.clone(), ucb)
//             })
//             .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
//             .map(|(name, _)| name);

//         // if let Some(ref connector) = connector {
//         //     Self::log_selected_connector(log_file, &connector);
//         // }
//         connector
//     }

//     fn update(&mut self, connector_name: &str, success: bool) {
//         if let Some(stats) = self.connectors.get_mut(connector_name) {
//             stats.total_txn += 1;
//             self.total_pulls += 1;

//             let reward = if success { 1 } else { 0 };

//             stats.successful_txn += reward;
//         }
//     }

//     fn get_stats(&self) -> Vec<(String, i64, u64, f64)> {
//         self.connectors
//             .iter()
//             .map(|(name, stats)| {
//                 let average_reward = if stats.total_txn > 0 {
//                     stats.successful_txn as f64 / stats.total_txn as f64
//                 } else {
//                     0.0
//                 };
//                 (
//                     name.clone(),
//                     stats.successful_txn,
//                     stats.total_txn,
//                     average_reward,
//                 )
//             })
//             .collect()
//     }
// }
// fn simulate_connector_performance() {
//     let mut rng = rand::thread_rng();
//     let mut mab = MultiArmedBandit::new(1.0);

//     let mut log_file = File::create("multi_arm_bandit_files/exploration_exploitation_log.csv")
//         .expect("Failed to create log file");
//     writeln!(log_file, "Transaction,Connector,Exploitation,Exploration").unwrap();

//     mab.add_connector("adyen        ");
//     mab.add_connector("bankofamerica");
//     mab.add_connector("cybersource  ");

//     for _ in 0..300 {
//         if let Some(selected_connector) = mab.select_connector(&mut log_file) {
//             let rng_val = match selected_connector.as_str() {
//                 "adyen        " => rng.gen_range(0.9..0.95),
//                 "bankofamerica" => rng.gen_range(0.7..0.75),
//                 "cybersource  " => rng.gen_range(0.8..0.85),
//                 _ => rng.gen_range(0.8..1.0),
//             };

//             let success = rng.gen_bool(rng_val);
//             println!("-----------------------------------------------");
//             let status = if success { "success" } else { "failed" };
//             println!(">> selected_connector: {selected_connector}, status: {status}");
//             mab.update(&selected_connector, success);
//         }
//     }

//     println!("\nFinal Statistics 1:");
//     for (name, successful_txn, total_txn, avg_reward) in mab.get_stats() {
//         println!(
//             "Connector: {} , {}/{} , Average Reward: {:.4}",
//             name, successful_txn, total_txn, avg_reward,
//         );
//     }

//     println!("Log file written: exploration_exploitation_log.csv");
// }

// fn main() {
//     simulate_connector_performance();

//     Command::new("python3")
//         .arg("multi_arm_bandit_files/plot.py")
//         .output()
//         .expect("Failed to execute python script");
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////

// 57. Non stationary MAB

// use rand::Rng;
// use std::collections::VecDeque;

// // Gateway structure representing each payment gateway
// #[derive(Clone, Debug)]
// struct Gateway {
//     id: String,
//     success_window: VecDeque<bool>,
//     window_size: usize,
//     success_rate: f64,
// }

// impl Gateway {
//     fn new(id: String, window_size: usize) -> Self {
//         Gateway {
//             id,
//             success_window: VecDeque::with_capacity(window_size),
//             window_size,
//             success_rate: 0.0,
//         }
//     }

//     // Update gateway performance and recalculate success rate
//     fn update_performance(&mut self, is_success: bool) {
//         // Maintain sliding window
//         if self.success_window.len() == self.window_size {
//             self.success_window.pop_front();
//         }
//         self.success_window.push_back(is_success);

//         // Recalculate success rate
//         let successful_txns = self.success_window.iter().filter(|&&x| x).count();
//         self.success_rate = successful_txns as f64 / self.success_window.len() as f64;
//     }
// }

// // Dynamic Gateway Router
// struct DynamicGatewayRouter {
//     gateways: Vec<Gateway>,
//     window_size: usize,
//     hedging_factor: f64,
// }

// impl DynamicGatewayRouter {
//     fn new(gateways: Vec<Gateway>, window_size: usize, hedging_factor: f64) -> Self {
//         DynamicGatewayRouter {
//             gateways,
//             window_size,
//             hedging_factor,
//         }
//     }

//     // Select gateway based on success rate and hedging
//     fn select_gateway(&mut self) -> &mut Gateway {
//         let mut rng = rand::thread_rng();

//         // Decide whether to explore or exploit
//         if rng.gen::<f64>() < self.hedging_factor {
//             // Exploration: randomly select a gateway
//             let random_index = rng.gen_range(0..self.gateways.len());
//             &mut self.gateways[random_index]
//         } else {
//             // Exploitation: select gateway with highest success rate
//             self.gateways
//                 .iter_mut()
//                 .max_by(|a, b| a.success_rate.partial_cmp(&b.success_rate).unwrap())
//                 .unwrap()
//         }
//     }

//     // Simulate transaction routing
//     fn route_transaction(&mut self, is_successful: bool) {
//         let selected_gateway = self.select_gateway();

//         selected_gateway.update_performance(is_successful);
//     }

//     // Compute statistical significance of gateway performance difference
//     fn compute_performance_difference(&self) -> f64 {
//         if self.gateways.len() < 2 {
//             return 0.0;
//         }

//         let gateways: Vec<&Gateway> = self.gateways.iter().collect();
//         let (gw1, gw2) = (gateways[0], gateways[1]);

//         // Normalized performance difference
//         let mu_diff = gw2.success_rate - gw1.success_rate;
//         let variance = (gw1.success_rate * (1.0 - gw1.success_rate) / self.window_size as f64)
//             + (gw2.success_rate * (1.0 - gw2.success_rate) / self.window_size as f64);

//         mu_diff / variance.sqrt()
//     }
// }

// // Example usage and simulation
// fn main() {
//     // Initialize gateways with initial configurations
//     let gateways = vec![
//         Gateway::new("Gateway1".to_string(), 1000),
//         Gateway::new("Gateway2".to_string(), 1000),
//     ];

//     // Configure router with optimal parameters
//     let mut router = DynamicGatewayRouter::new(
//         gateways.clone(),
//         1119,   // Optimal window size from paper
//         0.1554, // Optimal hedging factor
//     );

//     // Simulate transactions
//     for _ in 0..10000 {
//         let is_successful = rand::thread_rng().gen_bool(0.8); // 80% success probability
//         router.route_transaction(is_successful);
//     }

//     // Print final gateway performance
//     for gateway in router.gateways.iter() {
//         println!(
//             "{} Success Rate: {:.2}%",
//             gateway.id,
//             gateway.success_rate * 100.0
//         );
//     }

//     // Compute performance difference
//     let perf_diff = router.compute_performance_difference();
//     println!("Performance Difference Significance: {:.4}", perf_diff);
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
// pub trait DynamicRouting {
//     type Response;

//     fn get_val(&self) -> Self::Response;
// }

// impl DynamicRouting for Example1 {
//     type Response = i32;

//     fn get_val(&self) -> Self::Response {
//         self.inner
//     }
// }

// impl DynamicRouting for Example2 {
//     type Response = String;

//     fn get_val(&self) -> Self::Response {
//         self.inner.clone()
//     }
// }

// impl DynamicRouting for Example3 {
//     type Response = i32;

//     fn get_val(&self) -> Self::Response {
//         self.inner1
//     }
// }

// #[derive(Debug)]
// struct Example1 {
//     inner: i32,
// }

// struct Example2 {
//     inner: String,
// }

// struct Example3 {
//     inner1: i32,
// }

// struct DynDispatch {
//     inner: Box<dyn DynamicRouting<Response = i32>>,
// }

// fn helper(val: i32) -> Box<dyn DynamicRouting<Response = i32>> {
//     if val == 1 {
//         Box::new(Example1 { inner: 5 })
//     } else {
//         Box::new(Example3 { inner1: 10 })
//     }
// }

// fn main() {
//     let example = DynDispatch { inner: helper(3) };
//     println!(">> {:?}", example.inner.get_val());
// }

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
