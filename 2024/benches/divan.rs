use advent_of_code_2024::{day22, day23};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day22_part2() {
    day22::part2();
}

#[divan::bench]
fn day23_part2() {
    day23::part2();
}
