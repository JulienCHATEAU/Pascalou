use std::io;

fn main() {
    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("Can't read line !");
    let count: i64 = count.trim().parse().expect("Give a number !");

    let mut res = count * (count+1) / 2;
    let mut list = String::new();
    io::stdin().read_line(&mut list).expect("Can't read line !");
    let split = list.split(" ");

    for i in split {
        let number: i64 = i.trim().parse().expect("You gave something else than a number !"); 
        res -= number
    }

    println!("{}", res);
}
