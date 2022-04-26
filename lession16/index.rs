// trait
// trait类似面向对象语言中的interface，用来定义任何类型的方法集。

struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // 静态方法签名；Self表示实现者类型
    fn new(name: &'static str) -> Self;
    // 实例方法签名
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // trait 可以提供默认的方法定义。
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

// 往Sheep类型上添加方法
impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // 实现者可以使用它trait上的方法
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

// 对 Sheep 类型实现Animal trait
impl Animal for Sheep {
    // Self是实现者类型
    fn new(name: &'static str) -> Sheep {
        Sheep { name, naked: false }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaaah!"
        }
    }

    // 默认trait方法可以重载
    fn talk(&self) {
        println!("{} pauses briefly.... {}", self.name, self.noise());
    }
}

struct Dog {
    naked: bool,
    name: &'static str,
}

impl Animal for Dog {
    fn new(name: &'static str) -> Dog {
        Dog { name, naked: false }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn noise(&self) -> &'static str {
        self.name
    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");
    // let mut dolly = Sheep::new("kim");

    dolly.talk();
    dolly.shear();
    dolly.talk();

    let mut dog: Dog = Animal::new("Dog");
    dog.talk();
    println!("{}", dog.name());
}
