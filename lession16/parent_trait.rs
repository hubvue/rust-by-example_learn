// 父 trait
// Rust 没有继承，但是可以将一个trait定义为另一个trait的超集（即父trait）。例如：

trait Person {
    fn name(&self) -> String;
}

// Person 是Student 的父trait
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

#[derive(Debug)]
struct Kim {
    name: &'static str,
}
impl Person for Kim {
    fn name(&self) -> String {
        self.name.to_string()
    }
}

impl Student for Kim {
    fn university(&self) -> String {
        self.name.to_string()
    }
}

impl Programmer for Kim {
    fn fav_language(&self) -> String {
        "chinese".to_string()
    }
}

impl CompSciStudent for Kim {
    fn git_username(&self) -> String {
        self.name.to_string()
    }
}

fn comp_sci_student_greeting(student: impl CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main() {
    let kim = Kim { name: "kim" };
    println!("{}", comp_sci_student_greeting(kim));
    // println!("{:?}", kim);
}
