// 组合算子：and_then

// map()以链式调用的方式来简化match语句。然而，如果返回类型是Option<T>的函数最为map()的参数，会导致出现嵌套形式Option<Option<T>>（不解构的情况下）
// 这样多层串联调用就会变得混乱。所以有必要引入and_then()，在某些语言中它叫做flatmap。

// and_then() 使用被Option包好的值来调用输入函数并返回结果。如果Option是None，那么它返回None。

// 在下面例子中，cookable_v2()会产生一个Option<Food>。如果在这里使用map()而不是and_then()将会得到Option<Option<Food>>，这对eat()来说是一个无效类型。

#![allow(dead_code)]

#[derive(Debug)]
enum Food {
    CordonBleu,
    Steak,
    Sushi,
}

#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

// 我们没有制作寿司所需的原材料（有其他材料）
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

// 我们拥有全部食物的食谱，除了法国蓝带猪排的
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

// 使用match的方式来做一份菜
fn cookable_v1(food: Food) -> Option<Food> {
    match have_ingredients(food) {
        None => None,
        Some(food) => match have_recipe(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

// 也可以使用and_then()把上面的逻辑改写
fn cookable_v2(food: Food) -> Option<Food> {
    have_ingredients(food).and_then(have_recipe)
}

fn eat(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

fn main() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);
    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}
