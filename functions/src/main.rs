fn main() {
    // print_labeled_measurement(5, 'h');

    //операторы и вырожения
    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    //
    // println!("The value of y is: {y}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

// функция
// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

fn plus_one(x: i32) -> i32 {
    x + 1
}