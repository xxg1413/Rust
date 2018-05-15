///10.自助结账
///示例输出
///Enter the price of item 1: 25 
///Enter the quantity of item 1: 2
///Enter the price of item 2: 10
///Enter the quantity of item 2: 1
///Enter the price of item 3: 4
///Enter the quantity of item 3: 1
///Subtotal: $64.00
///Tax:$3.52 
///Total: $67.52

/*
 * 2018.5.1 by Jimmy Xiang
 */

use std::io;

fn main() {

    println!("Enter the price of item 1:");
    let mut raw_price_1 = String::new();
    io::stdin().read_line(&mut raw_price_1).unwrap();

    let price_1 :f32 = raw_price_1.trim().parse()
    .ok()
    .expect("Please type a number");

    println!("Enter the quantity of item 1:");
    let mut raw_item_num_1 = String::new();
    io::stdin().read_line(&mut raw_item_num_1).unwrap();
    let item_num_1 :f32 = raw_item_num_1.trim().parse()
    .ok()
    .expect("Please type a number");



    println!("Enter the price of item 2:");
    let mut raw_price_2 = String::new();
    io::stdin().read_line(&mut raw_price_2).unwrap();

    let price_2 :f32 = raw_price_2.trim().parse()
    .ok()
    .expect("Please type a number");

    println!("Enter the quantity of item 2:");
    let mut raw_item_num_2 = String::new();
    io::stdin().read_line(&mut raw_item_num_2).unwrap();
    let item_num_2 :f32 = raw_item_num_2.trim().parse()
    .ok()
    .expect("Please type a number");


    println!("Enter the price of item 3:");
    let mut raw_price_3 = String::new();
    io::stdin().read_line(&mut raw_price_3).unwrap();

    let price_3 :f32 = raw_price_3.trim().parse()
    .ok()
    .expect("Please type a number");

    println!("Enter the quantity of item 3:");
    let mut raw_item_num_3 = String::new();
    io::stdin().read_line(&mut raw_item_num_3).unwrap();
    let item_num_3 :f32 = raw_item_num_3.trim().parse()
    .ok()
    .expect("Please type a number");


    let subtotal:f32 = price_1 * item_num_1 + price_2 * item_num_2 + price_3 * item_num_3;
    let tax: f32 = subtotal * 0.055;
    let total:f32 = subtotal + tax;

    println!("Subtotal: ${:.2}\nTax: ${:.2}\nTotal: ${:.2}",subtotal,tax,total);

}