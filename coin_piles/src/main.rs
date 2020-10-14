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

    for e in pairs {
        if (e[0] == 0 && e[1] == 0) || ((e[0]+e[1]) % 3 == 0 && std::cmp::max(e[0], e[1]) <= 2*std::cmp::min(e[0], e[1]) && e[0] != 0 && e[1] != 0) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}