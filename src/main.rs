use std::env;
use lazy_static::lazy_static;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    lazy_static! {
        static ref DAYS: [fn(); 4] = [
            day1::solve,
            day2::solve,
            day3::solve,
            day4::solve,
        ];
    }

    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        for (ix, solve) in DAYS.iter().enumerate() {
            println!("Day {}", ix + 1);
            solve();
            println!();
        }
    } else {
        for arg in args {
            let day = arg.parse::<usize>().unwrap();
            println!("Day {}", day);
            DAYS[day - 1]();
        }
    }
}
