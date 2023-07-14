fn main() {
    // создание пустого вектора
    // let v: Vec<i32> = Vec::new();

    // создание не пустого вектора
    // let v = vec![1, 2, 3];

    // let mut v = Vec::new();
    //
    // // добавление записей в вектор
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // let v = vec![1, 2, 3, 4, 5];
    // // получение записи через индекс
    // let third: &i32 = &v[2];
    // println!("The third element is {third}");
    // // получение записей при помощи метода get
    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // }

    // let v = vec![1, 2, 3, 4, 5];
    // // строка ниже вызывает панику, поскольку нет такого значения в векторе
    // // let does_not_exist = &v[100];
    // // ниже паника не вызывается так как get возвращает Option
    // let does_not_exist = v.get(100);

    // let mut v = vec![1, 2, 3, 4, 5];
    //
    // let first = &v[0];
    //
    // /* векторы размещают значения в памяти друг за другом,
    //  добавление нового элемента в конец вектора может потребовать выделения новой памяти
    //  и копирования старых элементов в новое пространство, если нет достаточного места,
    //  чтобы разместить все элементы друг за другом там, где в данный момент хранится вектор.
    //  В этом случае ссылка на первый элемент будет указывать на освобождённую память */
    // // v.push(6);
    //
    // println!("The first element is: {first}");

    // let mut  v = vec![100, 32, 57];
    //
    // // изменение значений вектора
    // for i in &mut v {
    //     *i += 50;
    // }
    //
    // // перебор значений вектора
    // for i in &v {
    //     println!("{i}");
    // }

    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }
    //
    // // вектор, зачениями которого являются варианты перечисления
    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}
