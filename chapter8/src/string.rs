fn main() {
    // объявление пустой строки
    // let mut s = String::new();

    // объявление не пустой строки
    // let data = "initial contents";
    //
    // let s = data.to_string();
    //
    // // the method also works on a literal directly:
    // let s = "initial contents".to_string();
    //
    // let s = String::from("initial contents");

    // обявление строк спользуя разные языки
    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");

    // присоединение к строке другой строки
    // let mut s = String::from("foo");
    // s.push_str("bar");
    //
    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {s2}");

    // присоединение к строке символа
    // let mut s = String::from("lo");
    // s.push('l');

    // объединение строк при помощи оператора +
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    //
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    //
    // // let s = s1 + "-" + &s2 + "-" + &s3;
    // let s = format!("{s1}-{s2}-{s3}");// тоже самое что выше, только более читаемо и используется макрос format!

    // чтобы предотвратить возврат непредвиденного значения,
    // вызывающего ошибки которые не могут быть сразу обнаружены,
    // Rust просто не компилирует такой код
    // и предотвращает недопонимание на ранних этапах процесса разработки
    // let s1 = "hello";
    // let h = &s1[0];

    // let hello = "Здравствуйте";
    //
    // let s = &hello[0..4];// если бы вместо 4 стояла 1 то произошла бы ошибка, так как буквы кодируются в 2 байта
    // println!("{}", s)


    // перебор символов в строке
    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }
}