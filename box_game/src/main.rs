fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Could not read input");
    let t: u32 = buffer.trim().parse().expect("Could not convert to int");

    let mut values: Vec<u32> = Vec::new();

    for _ in 0..t {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).expect("Could not read input");
        values.push(buffer.trim().parse().expect("Could not convert"));
    }

    for e in values {
        if e % 2 == 0 {
            println!("ALICE");
        } else {
            println!("BOB");
        }
    }
}

// 1 L
// 2 W
// 3 L
// 4 W
// 5 W
// 6 W 