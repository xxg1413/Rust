///04.疯狂填词
///示例输出
///
///Enter a noun: dog
///Enter a verb: walk
///Enter an adjective: blue
///Enter an adverb: quickly
///Do you walk your blue dog quickly? That's hilarious!

/*
 * 2018.4.21 by Jimmy Xiang
 */

fn main() {
    use std::io;

    println!("Enter a noun:");
    let mut noun = String::new();
    io::stdin().read_line(&mut noun).unwrap();

    println!("Enter a verb:");
    let mut verb = String::new();
    io::stdin().read_line(&mut verb).unwrap();

    println!("Enter a adjective:");
    let mut adjective = String::new();
    io::stdin().read_line(&mut adjective).unwrap();

    println!("Enter a adverb:");
    let mut adverb = String::new();
    io::stdin().read_line(&mut adverb).unwrap();

    let sentence = "Do you $verb your $adjective $noun $adverb?That's hilarious!";

    let output = sentence.replace("$noun",noun.trim()).replace("$verb",verb.trim()).replace("$adjective",adjective.trim()).replace("$adverb",adverb.trim());

    println!("{}",output);
}

