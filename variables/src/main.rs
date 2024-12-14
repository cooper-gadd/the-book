// use std::io;

fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }
    // println!("The value of x is: {x}");

    // let spaces = "   ";
    // let spaces = spaces.len();

    // let guess: u32 = "42".parse().expect("Not a number!");

    // let x = 2.0; // f64
    // let y: f32 = 3.0; // f32

    // let sum = 5 + 10;
    // let difference = 95.5 - 4.3;
    // let product = 4 * 30;
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3;
    // let remainder = 43 % 5;

    // let t = true;
    // let f: bool = false;

    // let c = 'z';
    // let z = 'ℤ';
    // let heart_eyed_cat = '😻';

    //let tup: (i32, f64, u8) = (500, 6.4, 1);

    //let tup = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("The value of y is: {y}");

    // let x = (500, 6.4, 1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;

    // let a = [1, 2, 3, 4, 5];

    // let months = [
    //     "January",
    //     "February",
    //     "March",
    //     "April",
    //     "May",
    //     "June",
    //     "July",
    //     "August",
    //     "September",
    //     "October",
    //     "November",
    //     "December",
    // ];

    // let a: [i32; 5] = [1, 2, 3, 4, 5];

    // let a = [3; 5];

    // let a = [1, 2, 3, 4, 5];
    // let first = a[0];
    // let second = a[1];

    // let a = [1, 2, 3, 4, 5];

    // println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}");

    // println!("Hello, world!");
    // another_function();

    // another_function(5)

    // print_labeled_measurement(5, 'h');

    // let y = 6;
    // let x = (let y = 6);

    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    // print!("The value of y is: {y}");

    // let x = five();
    // print!("The value of x is: {x}");

    // let x = plus_one(5);
    // print!("The value of x is: {x}");

    // let number = 3;
    // let number = 7;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    // let number = 3;

    // if number {
    //     println!("number was three");
    // }

    // let number = 3;

    // if number != 0 {
    //     println!("number was something other than zero");
    // }

    // let number = 6;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    // let condition = true;
    // let number = if condition { 5 } else { 6 };

    // println!("The value of number is: {number}");

    // let condition = true;

    // let number = if condition { 5 } else { "six" };

    // println!("The value of number is: {number}");

    // loop {
    //     println!("again!");
    // }

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");

    // let mut number = 3;

    // while number != 0 {
    //     println!("{number}!");

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    // let a = [10, 20, 30, 40, 50];

    // for element in a {
    //     println!("the value is: {element}");
    // }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

// fn another_function() {
//     println!("Another function.");
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     print!("The measurement is: {value} {unit_label}");
// }

// fn five() -> i32 {
//     5
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }
