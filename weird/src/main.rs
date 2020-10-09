use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let mut i: u64 = line.trim().parse().unwrap();
    print!("{}", i);

    while i != 1 {
        if i % 2 == 0 {
            i = i/2;
        } else {
            i = i*3 + 1;
        }

        print!(" {}", i);
    }
}
