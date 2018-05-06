///07.矩形房间的面积
///示例输出
///What is the length of the room in feet? 15
///What is the width of the room in feet? 20
///You entered dimensions of 15 feet by 20 feet.
///The area is
///300 square feet
///27.871 square meters

/*
 * 2018.5.5 by Jimmy Xiang
 */

use std::io;

fn main() {

   let cf:f32 =  0.09290304;

    println!("What is the length of the room in feet?");
    let mut raw_length = String::new();
    io::stdin().read_line(&mut raw_length).unwrap();

    let length :f32 = raw_length.trim().parse()
    .ok()
    .expect("Please type a number");

    println!("What is the width of the room in feet?");
    let mut raw_width = String::new();
    io::stdin().read_line(&mut raw_width).unwrap();

    let width:f32 = raw_width.trim().parse()
    .ok()
    .expect("Please type a number");

    let area_feet = length * width;

    let area_meter =  area_feet * cf;

    println!("You entered dimensions of {} feet by {} feet.",length, width);
    println!("The area is\n{} square feet\n{:.3} square meters", area_feet,area_meter);

}

