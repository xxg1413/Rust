///02.计算字符数
///示例：
///What is the input string? Homer
///Homer has 5 characters.

/*
 * 2018.4.18 by Jimmy Xiang
 */

fn main() {

    use std::io;

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let  characters = input.trim();

    println!("{} has {} characters.", characters, characters.len());
}

