use std::io;

fn main() {
    // объявление констант
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // обозначение переменной
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // затеняемость
    // let x = 5;
    //
    // let x = x + 1;
    //
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }
    //
    // println!("The value of x is: {x}");

    // числовые операции
    // addition
    // let sum = 5 + 10;
    // println!("{}",sum);
    //
    // // subtraction
    // let difference = 95.5 - 4.3;
    // println!("{}",difference);
    //
    // // multiplication
    // let product = 4 * 30;
    // println!("{}",product);
    //
    // // division
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3; // Results in -1
    // println!("{}",quotient);
    // println!("{}",truncated);
    //
    // // remainder остаток
    // let remainder = 43 % 5;
    // println!("{}",remainder);

    // символы
    // let c = 'z';
    // let z: char = 'ℤ'; // with explicit type annotation
    // let heart_eyed_cat = '😻';
    // println!("{}\n{}\n{}", c, z, heart_eyed_cat);

    // кортежи
    // let k = (19, 45.44, 'c');
    // let emp = ();
    // println!("{:?}",k);
    // println!("{:?}",emp);
    // println!("{}",k.2);
    // //ошибка println!("{}",emp.0);

    //массивы
    // код, вызывает панику при вводе числа больше или равно 5, так как длинна массива всего 5
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);
}