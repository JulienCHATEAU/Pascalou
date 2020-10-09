use std;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Could not read input");
    let n: u128 = buffer.trim().parse().expect("Could not convert");
    
    let mut res = Vec::new();

    if (2..=4).contains(&n) {
        match n {
            2 => println!("NO SOLUTION"),
            3 => println!("NO SOLUTION"),
            4 => println!("3 1 4 2"),
            _ => ()
        }
        return;
    }

    for i in (1..n+1).step_by(2) {
        res.push(i);
    }
    for i in (2..n+1).step_by(2) {
        res.push(i);
    }

    for e in res {
        print!("{} ", e);
    }
}
