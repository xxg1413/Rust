///08.比萨聚会
///示例输出
///How mang people? 8
///How mang pizzas do you hava? 2
///In each pizza, how many slices are there? 8
///
///8 people with 2 pizzas, each pizza 8 slices
///Each person gets 2 pieces of pizza.
///There are 0 leftover pieces.

/*
 * 2018.5.8 by Jimmy Xiang
 */

use std::io;

fn main() {

    println!("How mang people?");
    let mut raw_num_people = String::new();
    io::stdin().read_line(&mut raw_num_people).unwrap();

    let num_people :i32 = raw_num_people.trim().parse()
    .ok()
    .expect("Please type a number");

    println!("How mang pizzas do you hava?");
    let mut raw_num_pizzas = String::new();
    io::stdin().read_line(&mut raw_num_pizzas).unwrap();

    let num_pizzas:i32 = raw_num_pizzas.trim().parse()
    .ok()
    .expect("Please type a number");

    println!("In each pizza, how many slices are there?");
    let mut raw_num_slices = String::new();
    io::stdin().read_line(&mut raw_num_slices).unwrap();

    let num_slices:i32 = raw_num_slices.trim().parse()
    .ok()
    .expect("Please type a number");

    let num_get = num_pizzas * num_slices / num_people;
    let num_left = num_pizzas * num_slices % num_people;

    println!("{} people with {} pizzas, eacho pizzas {}  slices.",num_people, num_pizzas, num_slices);
    println!("Each person gets {} pieces of pizza.", num_get);
    println!("There are {} leftover pieces", num_left);
}

