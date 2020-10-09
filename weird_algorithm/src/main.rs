fn main () {
    let mut n = get_line_input().trim().parse::<i128>().unwrap();
    let mut res = String::new();
    res += &n.to_string();
    while n != 1 {
        if n % 2 == 0 {
            n = n/2;
        } else {
            n = n*3+1;
        }
        res += &(" ".to_owned() + &n.to_string());
    }
    println!("{}", res);
}

fn get_line_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}