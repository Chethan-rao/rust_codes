// ok() -> It will convert Result<T,E> to Option<T> ignoring err E
// fn main() {
//     let a = vec!["1", "4.5", "3", "chethan"];
//     let b = a
//         .iter()
//         .filter_map(|&val| val.parse::<i32>().ok())
//         .collect::<Vec<_>>();
//     println!("{b:?}");
// }

// err() -> It will convert Result<T,E> to Option<E> ignoring Ok(T)

// ok_or(err) -> Converts Option<T> into Result<T,Err>. If Some() value is found, then it is mapped to Ok(), if None, then Err is passed as an argument inside ok_or(Err)
// fn main() {
//     let a:Option<i32> = Some(5);
//     let res = a.ok_or("Value Not found");
//     println!("{res:?}");
// }

// ok_or_else(closure) -> Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and None to Err(err()).
// fn main() {
//     let x = Some("foo");
//     assert_eq!(x.ok_or_else(|| 0), Ok("foo"));

//     let x: Option<&str> = None;
//     assert_eq!(x.ok_or_else(|| 0), Err(0));
// }

// iter() -> If applied on Vector 'a', then u can reuse 'a' after applying iter() also
// fn main() {
//     let numbers = vec![1, 2, 3, 4, 5, 6];
//     let even_numbers = numbers.iter();
//     println!("{numbers:?}");
// }

// into_iter() -> Consumes on which it is called. If applied on Vector 'a', then u can not reuse 'a' after applyling into_iter()
// fn main() {
//     let numbers = vec![1, 2, 3, 4, 5, 6];
//     let even_numbers = numbers.into_iter();
//     println!("{numbers:?}"); // Error - Moved value
// }

// filter() -> Filter values based on condition
// fn main () {
//     let numbers = vec![1, 2, 3, 4, 5, 6];
//     let even_numbers = numbers.into_iter().filter(|&x| x % 2 == 0).collect::<Vec<_>>();
// // Result: even_numbers = [2, 4, 6]
// }

// filter_map() -> The filter_map method is similar to filter, but it additionally allows transforming the elements before filtering. Returns an option
// fn square_root(x: f64) -> Option<f64> {
//     if x >= 0.0 {
//         Some(x.sqrt())
//     } else {
//         None
//     }
// }
// fn main() {
//     let numbers = vec![4.0, 9.0, -1.0, 16.0];
//     let roots = numbers
//         .into_iter()
//         .filter_map(|x| square_root(x))
//         .collect::<Vec<_>>();
//     println!("{roots:?}");
// }

// filter_map() v/s filter. filter will actually filter data based on condition. But filter_map does some thing on data and then converts it into
// option and then takes only Some() varient.
// fn main() {
//     let a = vec!["1.4","A+","0.6","3.6","B-"];
//     let res = a.iter().filter_map(|&val| val.parse::<f32>().ok()).collect::<Vec<_>>();
//     println!("{res:?}");   // [1,3]
// }

// extend()
// fn main() {
//     let a = vec![1,2,3];
//     let mut another = vec![4,5,6];
//     another.extend(a);
//     println!("{another:?}");   // [4,5,6,1,2,3]
// }

// flatten()
// fn main() {
//     let a = vec![Some(1),None,Some(3)];
//     let res = a.into_iter().flatten().collect::<Vec<i32>>();
//     println!("{res:?}");   // [1,3]
// }

// is_ok() -> If result is Ok() then returns true else returns false
// fn main() {
//     let a = vec!["1.4","A+","0.6","3.6","B-"];
//     let res = a.iter().filter(|&val| val.parse::<f32>().is_ok()).collect::<Vec<_>>();
//     println!("{res:?}");   // [1,3]
// }

// unwrap_or(default: T) -> Returns Some value, else if None, returns default value. It takes the default value directly as an argument.
// fn main() {
//     assert_eq!(Some(4).unwrap_or(5), 4);
//     assert_eq!(None.unwrap_or(5), 5);
// }

// unwrap_or_else(closure) -> Takes closure - Returns Some value, else if None, returns default value
// fn main() {
//     assert_eq!(Some(4).unwrap_or_else(|| 5), 4);
//     assert_eq!(None.unwrap_or_else(|| 5), 5);
// }

// In summary, unwrap_or() takes the default value directly as an argument, while unwrap_or_else() takes a closure that provides
// the default value if needed. unwrap_or_else() allows for more complex logic or computations to determine the default value when
// unwrapping fails, making it more flexible in certain scenarios.

// unwrap_or_default() -> Returns Ok(T) or if its Err then returns default implementation of that type
// #[derive(Default, Debug)]
// struct User {
//     id: i32,
//     name: String,
// }
// impl User {
//     fn new() -> Result<Self,String> {
//         // Ok(Self {
//         //     id: 1,
//         //     name: "chethan".to_owned()
//         // })
//         Err("error".to_owned())
//     }
// }
// fn main() {
//     let user1 = User::new().unwrap_or_default();
//     println!("{user1:?}");   // User { id: 0, name: "" }

//     let a: Option<i32> = None;
//     let b = a.unwrap_or_default();
//     println!("{b}");   // 0
// }

// map(closure) -> Converts Option<T> to Option<U> consuming Some value
// fn main() {
//     let str = Some("abc");
//     let len = str.map(|val| val.len());
//     assert_eq!(len, Some(3));
// }

// map_or(default_value, closure) -> Returns the provided default result (if none), or applies a closure to the contained value (if any).
// fn main() {
//     let x = Some("foo");
//     assert_eq!(x.map_or(42, |v| v.len()), 3);

//     let x: Option<&str> = None;
//     assert_eq!(x.map_or(42, |v| v.len()), 42);
// }

// map_or_else(default_closure, closure) -> Computes a default function result (if none), or applies a different closure to the contained value (if any).
// fn main() {
//     let k = 21;

//     let x = Some("foo");
//     assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);

//     let x: Option<&str> = None;
//     assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);
// }

// or(Option<T>) -> Option1.or(Option2) -> Returns the option if it contains a Some(value), otherwise returns Option2.
// fn main() {
//     let op1 = Some(4);
//     let op2 = Some(5);
//     assert_eq!(op1.or(op2), Some(4));

//     let op1 = None;
//     let op2 = Some(5);
//     assert_eq!(op1.or(op2), Some(5));
// }

// or_else(closure) where closure should return an Option
// fn main() {
//     assert_eq!(Some(4).or_else(|| Some(5)), Some(4));
//     assert_eq!(None.or_else(|| Some(5)), Some(5));
// }

// cloned() -> Maps an Option<&T> to an Option<T> by cloning the contents of the option.
// fn main() {
//     let x = 12;
//     let opt_x = Some(&x);
//     assert_eq!(opt_x, Some(&12));
//     let cloned = opt_x.cloned();
//     assert_eq!(cloned, Some(12));
// }

// chain() -> Takes 2 iterator and appends it
// fn main() {
//     let v1 = vec![1, 2, 3];
//     let v2 = vec![4, 5, 6];
//     for ele in v1.iter().chain(v2.iter()) {
//         println!("{ele}");     // 1 2 3 4 5 6
//     }
// }
