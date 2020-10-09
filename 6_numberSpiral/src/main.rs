use std::io;

fn main() {
    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("Can't read line !");
    let count: u128 = count.trim().parse().expect("Give a number !");

    for _i in 0..count {
        let mut coord = String::new();
        io::stdin().read_line(&mut coord).expect("Can't read line !");
        let split: Vec<&str> = coord.split(" ").collect();
        let y: u128 = split[0].trim().parse().expect("Give a number !");
        let x: u128 = split[1].trim().parse().expect("Give a number !");
        if y > x {
            if y % 2 == 1 {
                let res = (y*y) - (y-1)*2 + x - 1;
                println!("{}", res);
            } else {
                let res = (y*y) - x + 1;
                println!("{}", res);
            }
        } else  {
            if x % 2 == 0 {
                let res = (x*x) - (x-1)*2 + y - 1;
                println!("{}", res);
            } else {
                let res = (x*x) - y + 1;
                println!("{}", res);
            }
        }
    }
}
