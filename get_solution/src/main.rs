use std::process::exit;

const MAX_N: u32 = 1000000;
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
    let mut i = 0;

    for _ in 0..=MAX_N {
        loop {
            if i % 1000000 == 0 {
                eprintln!("{} / {}", i, LAST_REDUCING_NUMBER);
            }

            if i > LAST_REDUCING_NUMBER {
                exit(0);
            }
            if is_reducing_number(i) {
                println!("{},", i);

                i += 1;
                break;
            }
            i += 1;
        }
    }
}
