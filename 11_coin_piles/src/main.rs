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
        if x == 0 && y == 0 {
            println!("YES");
            continue;
        }
        if (x+y) % 3 == 0 {
            if x == 0 || y == 0 {
                println!("NO");
                continue;
            } 
            if x < y && x*2 < y {
                println!("NO");
                continue;
            }
            if x > y && y*2 < x {
                println!("NO");
                continue;
            }
            println!("YES");
            continue;
        } else {
            println!("NO");
            continue;
        }
    }
}

