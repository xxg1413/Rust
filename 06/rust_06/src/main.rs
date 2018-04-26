///06.计算退休时间
///示例输出
///What is your current age? 25
///At what age would you like to retire? 65
///You have 40 years left until you can retire.
///It's 2018, so you can retire in 2058.

/*
 * 2018.4.23 by Jimmy Xiang
 */

extern crate time;

use std::io;

fn main() {

    println!("What is your current age?");
    let mut current_age = String::new();
    io::stdin().read_line(&mut current_age).unwrap();

    let current_age:u32 = current_age.trim().parse()
    .ok()
    .expect("Please type a number");

    println!("At what age would you like to retire?");
    let mut retire_age = String::new();
    io::stdin().read_line(&mut retire_age).unwrap();

    let retire_age:u32 = retire_age.trim().parse()
    .ok()
    .expect("Please type a number");

    let left:i32  = retire_age as i32  - current_age as i32;

    let current_year = time::now().tm_year+1900;
    let retire_year = current_year + left;

    println!("It's {},so you can retire in {}",current_year,retire_year);

}
