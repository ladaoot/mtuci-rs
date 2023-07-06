

fn main() {
    // объявление экземпляра структуры User
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com"); // без mut в объявлении переменной нельзя менять значения полей

    // user2 имеет другое значение для email, но с тем же значением для полей username, active и sign_in_count из user1
    // мы больше не можем использовать user1 после создания user2
    let user2 = User {
        // active: user1.active,
        // username: user1.username,
        // email: String::from("another@example.com"),
        // sign_in_count: user1.sign_in_count,
        email: String::from("another@example.com"),
        ..user1 //оставшиеся поля устанавливаются неявно и должны иметь значения из указанного экземпляра
    };

    // объявление экземпляров кортежных структур
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // объявление структуры без полей
    let object = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,//: username,
        email,//: email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String, // если использовать &str то код не скомпелируеться, так как это ссылка и нет уверености в том что она будет существовать пока существует структура
    email: String,
    sign_in_count: u64,
}

// кортежные структуры
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// структурf не имеющие полей (единично-подобными структура)
struct AlwaysEqual;