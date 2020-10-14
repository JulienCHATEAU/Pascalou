use std::io;

fn sign(n: i64) -> i64 {
    if n == 0 {
        return 0;
    } else if n > 0 {
        return 1;
    } else {
        return -1;
    }
}

fn multiple_count(n: i64) -> i64 {
    let mut sum = 0;
    for i in 0..(n-1) {
        sum += 1 - sign(i % (n-1));
    }
    return sum;
}

fn main() {
    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("Can't read line !");
    let count: i64 = count.trim().parse().expect("Give a number !");

    for _i in 0..count {
        let mut coord = String::new();
        io::stdin().read_line(&mut coord).expect("Can't read line !");
        let split: Vec<&str> = coord.split(" ").collect();
        let n: i64 = split[0].trim().parse().expect("Give a number !");
        if n % 2 == 1 {
            println!("BOB");
        } else {
            println!("ALICE");
        }
    }
}

