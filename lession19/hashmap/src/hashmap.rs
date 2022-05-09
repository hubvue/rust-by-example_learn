// 散列表 HashMap

// HashMap是key-value结构，HashMap的键可以是布尔型、整型、字符串、或任意实现了Eq和Hash trait的其他类型。

// 和vector类似，HashMap也是可增长的，但HashMap在占据了多余空间时还可以缩小自己。
// 可以使用HashMap::with_capacity(unit)创建具有一定初始容量的HashMap，也可以使用HashMap::new()来获得一个带有默认初始容量的HashMap

use std::collections::HashMap;
#[allow(dead_code)]
fn call(number: &str) -> &str {
    match number {
        "789-1364" => {
            "We're sorry, the call cannot be completed as dialed. 
    Please hang up and try again."
        }

        "645-7689" => {
            "Hello, this is Mr. Awesome's Pizza. My name is Fred.
          What can I get for you today?"
        }

        _ => "Hi! Who is this again?",
    }
}

#[allow(dead_code)]
pub fn hashmap_fn() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // 接受一个引用并返回Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}, {}", call(number), number),
        _ => println!("Do not have Daniel is number."),
    }

    // 如果被插入的key为新内容，那么HashMap::insert 返回None，
    // 否则返回Some(oldValue)
    println!("{:?}", contacts.insert("Daniel", "154-6743"));
    println!("{:?}", contacts.insert("kim", "154-6743"));

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Do not have Daniel is number."),
    }

    contacts.remove(&"Ashley");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Do not have Daniel is number."),
    }

    // HashMap::iter() 返回一个迭代器， 迭代器以任意顺序举出key-value对
    for (contact, &number) in contacts.iter() {
        println!("Calling {contact}: {number}");
    }
}
