///03.打印引语
///示例输出
///What is the quote? These aren't the droids you're looking for.
///Who said it? Obi-Wan Kenobi
///Obi-Wan Kenobi says, "These aren't the droids you're looking for."
/*
 * 2018.4.19 by Jimmy Xiang
 */

fn main() {

    use std::io;

    println!("What is the quote?");

    let mut quote = String::new();
    let mut author = String::new();

    io::stdin().read_line(&mut quote).unwrap();

    println!("Who said it?");

    io::stdin().read_line(&mut author).unwrap();

    let output = author.trim().to_string() +" says, \""+quote.trim()+"\"";

    println!("{}",output);
}



