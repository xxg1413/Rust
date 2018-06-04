///11.货币兑换
///示例输出
///How mang euros are you exchanging? 81
///What is the exchange rate? 137.51
///81 euros at an exchange rate of 137.51 is
///111.38 U.S. dollars.

/*
 * 2018.5.23 by Jimmy Xiang
 */

use std::io;

fn main() {

    println!("How mang euros are you exchanging?");
    let mut raw_amount_from= String::new();
    io::stdin().read_line(&mut raw_amount_from).unwrap();

    let amount_from :f32 = raw_amount_from.trim().parse()
    .ok()
    .expect("Please type a number");

    println!("What is the exchange rate?");
    let mut raw_rate_from = String::new();
    io::stdin().read_line(&mut raw_rate_from).unwrap();

    let rate_from:f32 = raw_rate_from.trim().parse()
    .ok()
    .expect("Please type a number");

    const RATE_TO: f32 = 100.00;

    let amount_to = (amount_from * rate_from ).ceil() / RATE_TO;

    println!("{} euros at an exchange rate of {} is\n{} U.S. dollars.",amount_from, rate_from, amount_to);

}

