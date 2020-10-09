use std;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Could not read input");
    let t: u32 = buffer.trim().parse().expect("Could not convert to int");

    let mut pairs: Vec<Vec<u128>> = Vec::new();

    for _ in 0..t {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).expect("Could not read input");
        pairs.push(buffer.trim().split_whitespace().map(|s| s.parse().expect("parse error")).collect());
    }

    let mut res:Vec<u128> = Vec::new();
    for e in pairs {
        let y: u128 = e[0];
        let x: u128 = e[1];
        if y == x {
            res.push(x*x - x + 1);
        } else if x > y {
            if x % 2 == 0 {
                res.push((x-1)*(x-1)+y);
            } else {
                res.push(x*x - y +1);
            }
        } else {
            if y % 2 == 1 {
                res.push((y-1).pow(2) + x);
            } else {
                res.push(y*y - x  + 1);
            }

        }
    }

    for e in res {
        println!("{}", e);
    }
}
