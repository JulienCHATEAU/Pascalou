use std::io;

fn main() {
    let mut dna = String::new();
    io::stdin().read_line(&mut dna).expect("Can't read line !");

    let mut count = 0;
    let mut max = 0;
    let mut curr_letter = 'z';
    for letter in dna.chars() {
        if curr_letter != letter {
            if count > max {
                max = count;
            }
            count = 1;
            curr_letter = letter;
        } else {
            count += 1;
        }
    }

    println!("{}", max);
}
