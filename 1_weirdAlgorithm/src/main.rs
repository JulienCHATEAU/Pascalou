use std::io; 

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line !");
    let mut number: i64 = input.trim().parse().expect("Please type a number !");

    let mut res = format!("{}", &number);
    loop {
        if number == 1 {
            break
        }
        if number & 1 == 0 {
            number = number / 2;
        } else {
            number = number * 3 + 1;
        }
        res = format!("{} {}", &res, &number);
    } 
    println!("{}", res);
}
