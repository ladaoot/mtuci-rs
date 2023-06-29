fn main() {
    // пример области видимости
    // let s = "sok";
    // {                      // s is not valid here, it’s not yet declared
    //     let s = "hello";   // s is valid from this point forward
    //
    //     println!("{}", s);
    // }                      // this scope is now over, and s is no longer valid
    //
    // println!("{}", s);


    // let mut s = String::from("hello");
    //
    // s.push_str(", world!"); // push_str() appends a literal to a String
    //
    // println!("{}", s); // This will print `hello, world!`


    // пример ошибки компеляции, так как после создания s2  s1 является недействительной
    // было использовано 'поверхностное' копирование
    // let s1 = String::from("hello");
    // let s2 = s1;
    //
    // println!("{}, world!", s1);


    // код работает, так мы 'глубоко' скопировали строку  s1, тоесть s1 и s2 ссылается на разные части памяти в куче
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    //
    // println!("s1 = {}, s2 = {}", s1, s2);


    //  let s = String::from("hello");  // s comes into scope
    //
    // takes_ownership(s);             // s's value moves into the function...
    //                                // ... and so is no longer valid here
    // // println!("{}", s);  ошибка компеляции, покольку данные из кучи были удалены
    //
    // let x = 5;                      // x comes into scope
    //
    // makes_copy(x);                  // x would move into the function,


    // let s1 = gives_ownership();         // gives_ownership moves its return
    // // value into s1
    //
    // let s2 = String::from("hello");     // s2 comes into scope
    //
    // let s3 = takes_and_gives_back(s2);  // s2 is moved into
    //                                                     // takes_and_gives_back, which also
    //                                                     // moves its return value into s3
    // println!("{}", s3);
    // // println!("{}", s2); ошибка компеляции, так как s2 больше недействительна
    // println!("{}", s1);


    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    // println!("{}", s1); ошибка компеляции так как s1 более недествительна

    println!("The length of '{}' is {}.", s2, len);

} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.


// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.
//
// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// }

// fn gives_ownership() -> String {             // gives_ownership will move its
//                                              // return value into the function
//                                              // that calls it
//
//     let some_string = String::from("yours"); // some_string comes into scope
//
//     some_string                              // some_string is returned and
//                                              // moves out to the calling
//                                              // function
// }
//
// // This function takes a String and returns one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//                                                       // scope
//
//     a_string  // a_string is returned and moves out to the calling function
// }

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}