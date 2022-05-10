// 更改或自定义 Key 的类型

// 任何实现了Eq和Hash trait 的类型都可以充当HashMap的键。这包括：
// - bool
// - int、unit，以及其他整型
// - String 和 &str （如果使用String作为键来创建HashMap，则可以将&str作为散列表的.get()方法的参数获取值）

// f32 和 f64 没有实现Hash，这很大程度上是由于若使用浮点数作为 散列表的键，浮点精度误差会很容易导致错误。

// 对于所有的集合类，如果它们包含的类型都分别实现了Eq和Hash，那么这些集合类也就实现了Eq和Hash。例如 若 T实现了Hash，则Vec<T>也实现了Hash

// 对自定义类型可以通过派生宏来实现Eq和Hash， #[derive(PartialEq, Eq, Hash)]，也可以手动实现

// 下面做一个非常简易的用户登录系统：

use std::collections::HashMap;

// Eq 要求对此类型推导PartialEq
#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("Userame: {username}");
    println!("Password: {password}");
    println!("Attempting logon....");

    let logon = Account { username, password };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successful login!");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
        }
        _ => println!("Login failed!"),
    }
}

pub fn custom_key() {
    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "kim",
        password: "123456",
    };

    let account_info = AccountInfo {
        name: "kim",
        email: "1191340528@qq.com",
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "kim", "123456");
    try_logon(&accounts, "kim", "123456");
}
