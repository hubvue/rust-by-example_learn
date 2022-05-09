// Rust中有两种字符串类型：String和 &str

// String被存储为由字节组成的vector(Vec<u8>) ，但保证了它一定是一个有效的UTF-8序列。
// String是堆分配的，可增长，且不是零结尾的
// String的结构
// pub struct String {
//     vec: Vec<u8>
// }

// &str是一个总是指向有效UTF-8序列的切片(&[u8])，并可用来查看String的内容，就如同&[T]是Vec<T>的全部或部分引用。
// &str实际上就是String的一个借用切片
pub fn string_fn() {
    // 一个对只读内存中分配的字符串的引用
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Prangram: {pangram}");

    // 逆序迭代单词，这里并没有分配新的字符串
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {word}");
    }

    // 复制字符到一个vector排序并移除重复值
    let mut chars: Vec<char> = pangram.chars().collect();
    // 排序
    chars.sort();
    // 去重
    chars.dedup();

    println!("chars: {chars:?}");
    // 创建一个空的且可增长的String
    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }
    println!("string: {string}");

    // 这个缩短的字符串是源字符串的一个切片，所以没有执行新的分配操作
    let chars_to_trim: &[char] = &[',', ' '];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("used characters: {trimmed_str}");

    // 堆分配一个字符串
    let alice = String::from("I like dogs");
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {alice}");
    println!("Bob says: {bob}");
}
