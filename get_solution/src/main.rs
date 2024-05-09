use std::process::exit;

const LAST_REDUCING_NUMBER: i32 = 987654321;

fn is_reducing_number(number: i32) -> bool {
    let str = number.to_string();
    str.chars()
        .fold((':', true), |(prev, result), current| {
            (current, result && prev > current)
        })
        .1
}

fn main() {
    for i in 0..=LAST_REDUCING_NUMBER {
        if i % 1000000 == 0 {
            eprintln!("{} / {}", i, LAST_REDUCING_NUMBER);
        }

        if i > LAST_REDUCING_NUMBER {
            exit(0);
        }
        if is_reducing_number(i) {
            println!("{},", i);
        }
    }
}
