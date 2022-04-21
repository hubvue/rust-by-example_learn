// 可变性
// 可变数据使用&mut T进行可变借用。这叫做可变引用，它使借用者可以读写数据。相反，&T通过不可变引用拉借用数据，借用这可以读数据不能更改数据。
// 相反，&T通过不可变引用来借用数据，借用者可以读数据二不能更改数据。

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // &'static str 是一个对分配在只读内存区的字符串的引用
    author: &'static str,
    title: &'static str,
    year: u32,
}

// 此函数接受一个对Book类型的引用
fn borrow_book(book: &Book) {
    println!(
        "I immutabley borrowed {} - {} edition",
        book.title, book.year
    );
}

// 此函数接受一个可变借用
fn new_edition(book: &mut Book) {
    book.year = 2022;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    let immutablebook = Book {
        // 字符串字面量拥有 &'static str 类型
        author: "Douglas Hofstader",
        title: "Godel, Escher, Bach",
        year: 1979,
    };

    // 创建一个 immutablebook 的可变copy
    let mut mutabook = immutablebook;
    let mut mutabook1 = immutablebook;

    // 不可变借用一个不可变对象
    borrow_book(&immutablebook);

    // 不可变借用一个可变对象, 也是可以的
    borrow_book(&mutabook);

    // 可变借用一个可变对象
    new_edition(&mut mutabook);

    new_edition(&mut mutabook1);

    // 可变借用一个不可变对象  报错
    // new_edition(&mut immutablebook);

    // let arr = vec![1];
    // let total = sum(&arr);
    // let v = max(&arr);
}
