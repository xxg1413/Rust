///09.涂料计算程序
///示例输出
///What is length? 30
///What is width? 12 
///You will need to purchase 2 gallons of
///paint to conver 360 square feet.

/*
 * 2018.5.10 by Jimmy Xiang
 */

use std::io;

fn main() {

    println!("What is length?");
    let mut raw_length = String::new();
    io::stdin().read_line(&mut raw_length).unwrap();

    let length :f32 = raw_length.trim().parse()
    .ok()
    .expect("Please type a number");

    println!("What is width?");
    let mut raw_width = String::new();
    io::stdin().read_line(&mut raw_width).unwrap();

    let width:f32 = raw_width.trim().parse()
    .ok()
    .expect("Please type a number");


    let area:f32 = (width * length)/ 350.0;

    let num_gallon:f32 = area.ceil();
    

    println!("You will need to purchase {} gallons of\npaint to conver 360 square feet.",num_gallon);

}

