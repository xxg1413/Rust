/// 01.问好
/// 示例：
/// What is your name? Brian
/// Hello, Brian, nice to meet you!

/*
 * 2018.4.17 by Jimmy Xiang
 */

fn main() {

    use std::io;

    println!("What is your name?");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let message = "Hello,".to_string()+input.trim()+",nice to meet you!";

    println!("{}",message);
}
