///05.简单的数学处理
///示例输出
///What is the first number? 10
///What is the second number? 5
///10 + 5 = 15
///10 - 5 = 5
///10 * 5 = 50
///10 / 5 = 2

/*
 * 2018.4.23 by Jimmy Xiang
 */

use std::io;

fn main() {

    println!("What is the first number?");
    let mut first = String::new();
    io::stdin().read_line(&mut first).unwrap();

    let first:u32 = first.trim().parse()
    .ok()
    .expect("Please type a number");

    println!("What is the second number?");
    let mut second = String::new();
    io::stdin().read_line(&mut second).unwrap();

    let second:u32 = second.trim().parse()
    .ok()
    .expect("Please type a number");


    let add = first + second;
    let sub = first - second;
    let plus = first * second;
    let div = first / second;

    println!("{}+{}={}\n{}-{}={}\n{}*{}={}\n{}/{}={}",first,second,add,first,second,sub,first,second,plus,first,second,div);
}

