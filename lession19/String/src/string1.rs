// 字面量和转义字符

// 书写含有特殊字符的字符串字面量有很多方法。它们都会产生类似的&str类型，所以是最好，最方便的写法。
// 类似的，字符串字面量也有很多中欧冠写法，它们都会产生&[u8: N]类型。

// 通常特殊字符是使用反斜杠字符 \ 来转义的，这样就可以在字符串中写入各种各样的字符，甚至是不可打印的字符以及你不知道如何输入的字符。
// 如果需要反斜杠字符，再用另一个反斜杠来转义它就可以。 \\
pub fn string_fn() {
    // 通过转义，可以用十六进制值来表示字节。
    let byte_eascpe = "I am writing \x52\x75\x73\x74";
    println!("What are you doing \x3F (\\x3F means ?) {byte_eascpe}");

    // 也可以使用Unicode码位来表示。
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!("Unicode character {unicode_codepoint} (U+211D) is called {character_name}");

    let long_string = "String literals
                             can span multiple lines.
                             The linebreak and indentation here ->\
                             <- can be escaped too!";

    println!("{long_string}");

    // 有时候会有太多需要转义的字符，或者是直接原样写出会更便利。这时可以使用原始字符串
    let raw_str = r"Escapes don not work here: \x3F \u{211D}";
    println!("{raw_str}");

    // 如果需要在原始字符串中写引号，则需要在两边加上 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果字符串中需要写#，那就再定界符中使用更多的 #
    // 可使用的 # 的数目没有限制
    let longer_delimiter = r####"A string whit "# in it. And even "##！""####;
    println!("{longer_delimiter}");

    // 想要非UTF-8字符串（&str和String都必须是合法的UTF-8序列），或者需要一个字节数组，请使用字节串。

    // 这不是字符串，而是字节串
    let bytestring: &[u8; 20] = b"this is a bytestring";
    // 字节串没有实现Display
    println!("A bytestring: {bytestring:?}");

    // 字节串可以使用单字节的转义字符, 但不能使用Unicode 转义字符
    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("{escaped:?}");
    // let escaped = b"\u{211D}";

    // 原始字节串和原始字符串的写法一致
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{raw_bytestring:?}");

    // 把字节串转为&str，可能失败
    use std::str;
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{my_str}'");
    }

    // 字节串可以不使用UTF-8编码
    let shift_jis = b"\x82\xe6\x82\xaa8\x82\xb1\x82";
    println!("{shift_jis:?}");

    // 当不使用UTF-8编码时就无法转为 &str
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: {my_str}"),
        Err(e) => println!("Conversion failed: {e:?}"),
    }
}
