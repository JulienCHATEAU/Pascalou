use std::io;

fn main() {
    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("Can't read line !");
    let count: i64 = count.trim().parse().expect("Give a number !");

    if count < 5 {
        match count {
            1 => println!("1"),
            2 => println!("NO SOLUTION"),
            3 => println!("NO SOLUTION"),
            4 => println!("2 4 1 3"),
            _ => println!(""),
        }
        return;
    }

    let mut vec = Vec::new();

    for i in 1..count+1 {
        if i % 2 == 0 {
            vec.push(i);
        }
    }
    for i in 1..count+1 {
        if i % 2 == 1 {
            vec.push(i);
        }
    }

    for i in vec {
        print!("{} ", i);
    }
}
