use std::io;

fn main() {
    println!("Введите темтературу для конвертации Фаренгейт к Цельсию");

    let mut str = String::new();

    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");

    let x: i32 = str
        .trim()
        .parse()
        .expect("Index entered was not a number");

    println!("В Целсиях это - {}", far_to_cel(x));

    println!("Введите номер числа Фибоначе, которое нужно сгенерировать");

    let mut str = String::new();

    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");

    let n: i32 = str
        .trim()
        .parse()
        .expect("Index entered was not a number");

    println!("{}-ое число Фибоначи - {}", n , fib(n));
}

fn far_to_cel(x: i32) -> i32{
    (x - 32)*5/9
}

fn fib (x: i32) -> i32{
    if x == 1 {
         1
    } else if x == 2 {
         1
    } else if x == 3 {
         2
    } else {
        let a =fib(x - 1);
        let b = fib(x - 2);
        a+b
    }
}