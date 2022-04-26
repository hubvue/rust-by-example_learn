// 运算符重载
// 在Rust中，很多运算符可以通过trait来重载。也就是说，这些运算符可以根据根据它们的输入参数来完成不同的任务。
// 这之所以可行，是因为运算符就是方法调用的语法糖。
// 例如：a + b是的+运算符会调用add方法，也就是a.add(b)。
// 这个add方法是Addtrait的一部分。
// 因此 + 运算符可以被任何Add trait的实现者使用。

// 可以重载的运算符都在std::ops这个包里

use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// std::ops::Add trait用来指明+的功能，这里我们实现Add<Bar>，它是用于把对象和Bar类型的右操作（RHS）加起来的trait。
// 下面代码块实现了Foo + Bar = FooBar这样的运算
impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");
        FooBar
    }
}

impl ops::Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");
        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}
