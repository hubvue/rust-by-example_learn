// Option 和unwrap

// 在标准库（std）中有个叫做Option<T>的枚举类型，用于有”不存在“的可能性的情况。它表现为一下两个选项中的一个：
//  - Some(T): 找到一个属于T类型的元素
//  - None: 找不到相应元素

// 这些选项可以通过match显式地处理，或使用unwrap隐式地处理。隐式处理要么返回Some内部的元素，要么就panic。

// 请注意，手动使用expect方法自定义panic信息是可能的，但相比显式处理，unwrap的输出仍显得不太有意义。

fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck! I am throwing that snake in a fire."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No gift? Oh well."),
    }
}

fn give_princess(gift: Option<&str>) {
    // unwrap 在接受到None时将会返回panic。
    let inside = gift.unwrap();
    if inside == "snake" {
        panic!("AAAaaaa!!!!");
    }
    println!("I love {}s !!!!!", inside);
}

fn main() {
    let food = Some("chichen");
    let snake = Some("snke");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess(bird);
    give_princess(nothing);
}
