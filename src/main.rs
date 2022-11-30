use std::io;
fn main() {
    println!("Hello, world!");
    let x: u32 = 972;
    let tup: (i32, bool, char);
    tup = (1, true, 's');
    println!("{}", tup.1);
    let arr = [1, 2, 3, 4, 5];
    arr[4];
    println!("{}", arr[4]);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input + 2);
    let x: u8 = 9; // 0-255
    let y: i8 = 8; //-128-127
    let z = x + y as u8;
    println!("{}", z);
    let x: u8 = 252; // 0-255
    let y: u8 = 1; //-128-127
    let z = x + y;
    println!("{}", z);
    let x: f64 = 255.0; // 0-255
    let y: f64 = 1.0; //-128-127
    let z = x / y;
    println!("{}", z);
    let x = 127_000i64; // 0-255
    let y = 10_i64; //-128-127
    let z = x / y;
    println!("{}", z);
    let x = 127_000 as i64; // 0-255
    let y = 10_i32; //-128-127
    let z = x / y as i64;
    println!("{}", z);
    let cound = 2 as f32 <= 2.2;
    let cond2 = true && !cound; // &&  || !
    println!("{}", cond2);
    //control flow
    let food = "that";
    if food == "cookie" {
        println!("I like it")
    } else if food == "fruit" {
        println!("all right!")
    } else if food == "banana" {
        println!("all right! boring!")
    } else {
        println!("that's too bad!")
    }
    test();
    add_numbers(20, 35);

    let number = {
        let x = 3;
        x + 1 // no ; or failed
    };
    println!("{}", number);

    let result = add_othernumbers(2, 3);
    println!("{}", result);
    let result = add_moreothernumbers(3, 5);
    println!("{}", result);
    let x = 2;
    let y = x;
    add(x, y);
}

fn test() {
    println!("Test has been called...")
}

fn add_numbers(x: i32, y: i32) {
    println!("The sum is: {}", x + y)
}
fn add_othernumbers(x: i32, y: i32) -> i32 {
    return x + y;
}
fn add_moreothernumbers(x: i32, y: i32) -> i32 {
    let result = x + y;
    if result > 10 {
        return result - 10;
    }
    result
}
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
