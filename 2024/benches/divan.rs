use advent_of_code_2024::day22;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day22_part2() {
    day22::part2();
}
