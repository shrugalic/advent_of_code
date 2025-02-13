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

#[divan::bench]
fn day23_part2_andre() {
    day23::solve_part2_andre_optimized(include_str!("../../2024/input/day23.txt"));
}

#[divan::bench]
fn day23_part2_andre_orig() {
    day23::solve_part2_andre_orig(include_str!("../../2024/input/day23.txt"));
}
