fn main() {
    let mut str1 = String::from("Hello world!");
    const STR2: &str = "foo bar";

    let result1 = find_first_world(&str1[..]);
    let result2 = find_first_world(STR2);

    // str1.clear(); // ERROR

    println!("空格前的单词是：{}", result1);
    println!("空格前的单词是：{}", result2);
}

fn find_first_world(string: &str) -> &str {
    let str_bytes_array = string.as_bytes();
    for (index, &str_byte) in str_bytes_array.iter().enumerate() {
        if str_byte == b' ' {
            return &string[..index];
        }
    }
    &string[..]
}
