mod benchmark;

mod day_16;
mod day_18;
mod day_19;
mod day_20;
mod day_21;

use crate::benchmark::{benchmark_run, print_day, print_header};

fn main() {
    benchmark_all!(
        day_16,
        day_18,
        day_19,
        day_20,
        day_21
    );
}