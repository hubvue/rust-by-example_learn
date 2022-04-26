// 使用dyn返回trait

// Rust编译器需要知道每个函数的返回类型需要多少空间。这意味着所有函数都必须返回一个具体类型。
// 与其他语言不同，如果你有个像Animal那样的trait，则不能编写返回Animal的函数，因为其不同的实现将需要不同的内存量。

// 但是，有一个简单的解决方法。相比于直接返回一个trait对象，我们的函数可以返回一个包含Animal的Box。
// box只是对堆中某些内存的引用（box是一个指向堆空间的指针，是固定大小的）。
// 因为引用的大小是静态已知的，并且编译器可以保证引用指向已分配的堆Animal，于是就可以从函数中返回trait

// 每当在堆上分配内存时，Rust都会尝试尽可能明确。
// 因此如果你的函数以这种方式返回指向堆的trait指针，则需要使用dyn关键字编写返回类型，例如Box<dyn Animal>

struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaaaah!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "mooooooo!"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn animal_noise(animal: Box<dyn Animal>) {
    println!("animal noise: {}", animal.noise());
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("{}", animal.noise());

    animal_noise(Box::new(Sheep {}));
    animal_noise(Box::new(Cow {}));
}
