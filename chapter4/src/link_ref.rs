fn main() {
    // let s1 = String::from("hello");
    //
    // let len = calculate_length(&s1);
    //
    // println!("The length of '{}' is {}.", s1, len);


    // let mut s = String::from("hello");
    //
    // change(&mut s);


    // let mut s = String::from("hello");
    //
    // let r1 = &mut s;
    // let r2 = &mut s; // мы не можем заимствовать s как изменяемые более одного раза в один момент
    //
    // println!("{}, {}", r1, r2);


    // никакой ошибки, так как ссылки находятьсся в разных облостях видимости
    // let mut s = String::from("hello");
    //
    // {
    //     let r1 = &mut s;
    // } // r1 goes out of scope here, so we can make a new reference with no problems.
    //
    // let r2 = &mut s;


    // let mut s = String::from("hello");
    //
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM не может быть изменяемой ссылки, пока у нас есть неизменяемая ссылка на то же значение
    //
    // println!("{}, {}, and {}", r1, r2, r3);


    // let mut s = String::from("hello");
    //
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{} and {}", r1, r2);
    // // variables r1 and r2 will not be used after this point
    //
    // let r3 = &mut s; // no problem
    // println!("{}", r3);


    let reference_to_nothing = dangle();
}

fn dangle() -> String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    s
    // &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()
// } // Here, s goes out of scope. But because it does not have ownership of what
// // it refers to, it is not dropped.