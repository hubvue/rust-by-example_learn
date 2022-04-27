// 消除重叠trait
// 一个类型可以实现许多不同的trait。如果两个trait都需要相同的名称怎么办？
// 例如，许多trait可能拥有get()的方法。他们甚至可能有不同的返回类型。

// 有个好消息：由于每个trait实现都有自己的impl块，因此很清楚要实现哪个trait的get方法。

// 何时需要调用这些方法？为了消除它们之间的歧义，我们必须使用完全限定预发。

trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u8;
}

struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "kim".to_owned(),
        age: 28,
    };

    // multiple applicable items in scope
    // println!("{}", form.get());

    let username = <Form as UsernameWidget>::get(&form);
    println!("username: {}", username);
    let age = <Form as AgeWidget>::get(&form);
    println!("age: {}", age);
}
