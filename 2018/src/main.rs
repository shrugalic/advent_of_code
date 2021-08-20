use crate::day01::{cumulate_frequency_adjustments, find_first_repeated_frequency};
use crate::day02::{differing_letters_of_correct_boxes, product_of_2_and_3_counts};
use crate::day03::{id_of_non_overlapping_claim, overlapping_claim_count};
use crate::day04::{strategy_one, strategy_two};
use crate::day05::{length_of_shortest_possible_polymer, remaining_units_after_reaction};
use crate::day06::{
    size_of_area_with_max_total_distance_to_all_coords, size_of_largest_finite_area,
};
use crate::day07::{count_seconds, order_of_steps};
use crate::day08::{input_metadata_sum, input_value};
use crate::day09::high_score;
use crate::day10::message;
use crate::day11::{largest_total_power_3x3_square, largest_total_power_variable_size_square};
use crate::day12::{number_of_plants_after_20_gens, number_of_plants_after_generations};
use crate::day13::{location_of_first_crash, location_of_last_cart};
use crate::day14::{recipe_count_until_this_score_appears, score_of_10_recipes_after};
use crate::day15::Grid;
use crate::day16::{
    figure_out_op_code_numbers_and_run_program, number_of_samples_matching_3_or_more_opcodes,
};
use crate::day17::Ground;
use crate::day18::LumberCollectionArea;
use crate::day19::sum_of_divisors;
use crate::day20::Base;
use crate::day21::reversed_day21program;
use crate::day22::full_cave;
use crate::day23::{
    count_nanobots_in_signal_range, distance_to_origin_from_point_within_range_of_most_nanobots,
};
use crate::device::Device;
use line_reader::read_file_to_lines;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod device;
mod opcode;

fn main() {
    day01();
    day02();
    day03();
    day04();
    day05();
    day06();
    day07();
    day08();
    day09();
    day10();
    day11();
    day12();
    day13();
    day14();
    day15();
    day16();
    day17();
    day18();
    day19();
    day20();
    day21();
    day22();
    day23();
}

fn day01() {
    assert_eq!(
        cumulate_frequency_adjustments(&read_file_to_lines("input/day01.txt")),
        454
    );
    assert_eq!(
        find_first_repeated_frequency(&read_file_to_lines("input/day01.txt")),
        566
    );
}

fn day02() {
    assert_eq!(
        product_of_2_and_3_counts(&read_file_to_lines("input/day02.txt")),
        7936
    );
    assert_eq!(
        differing_letters_of_correct_boxes(&read_file_to_lines("input/day02.txt")),
        "lnfqdscwjyteorambzuchrgpx"
    );
}

fn day03() {
    assert_eq!(
        overlapping_claim_count(&read_file_to_lines("input/day03.txt")),
        113576
    );
    assert_eq!(
        id_of_non_overlapping_claim(&read_file_to_lines("input/day03.txt")),
        825
    );
}

fn day04() {
    assert_eq!(65489, strategy_one(&read_file_to_lines("input/day04.txt")));
    assert_eq!(3852, strategy_two(&read_file_to_lines("input/day04.txt")));
}

fn day05() {
    assert_eq!(
        9462,
        remaining_units_after_reaction(&read_file_to_lines("input/day05.txt")[0])
    );
    assert_eq!(
        4952,
        length_of_shortest_possible_polymer(&read_file_to_lines("input/day05.txt")[0])
    );
}

fn day06() {
    assert_eq!(
        4589,
        size_of_largest_finite_area(read_file_to_lines("input/day06.txt"))
    );
    assert_eq!(
        40252,
        size_of_area_with_max_total_distance_to_all_coords(
            read_file_to_lines("input/day06.txt"),
            10_000
        )
    );
}

fn day07() {
    assert_eq!(
        "JNOIKSYABEQRUVWXGTZFDMHLPC",
        order_of_steps(&read_file_to_lines("input/day07.txt"))
    );
    assert_eq!(
        1099,
        count_seconds(&read_file_to_lines("input/day07.txt"), 5, 60)
    );
}

fn day08() {
    assert_eq!(
        42146,
        input_metadata_sum(&read_file_to_lines("input/day08.txt")[0])
    );
    assert_eq!(
        26753,
        input_value(&read_file_to_lines("input/day08.txt")[0])
    );
}

fn day09() {
    assert_eq!(374690, high_score(477, 70851));
    assert_eq!(3_009_951_158, high_score(477, 7_085_100));
}

fn day10() {
    let input = &read_file_to_lines("input/day10.txt");
    assert_eq!(10511, message(input).1);
}

fn day11() {
    assert_eq!((28, 235, 87), largest_total_power_3x3_square(8199));
    assert_eq!(
        (119, 234, 272, 18),
        largest_total_power_variable_size_square(8199)
    );
}

fn day12() {
    assert_eq!(
        2063,
        number_of_plants_after_20_gens(&read_file_to_lines("input/day12.txt"))
    );
    assert_eq!(
        1_600_000_000_328,
        number_of_plants_after_generations(&read_file_to_lines("input/day12.txt"), 50_000_000_000)
    );
}

fn day13() {
    assert_eq!(
        (102, 114),
        location_of_first_crash(&read_file_to_lines("input/day13.txt"))
    );
    assert_eq!(
        (146, 87),
        location_of_last_cart(&read_file_to_lines("input/day13.txt"))
    );
}

fn day14() {
    assert_eq!("1411383621", score_of_10_recipes_after(760_221));
    assert_eq!(20177474, recipe_count_until_this_score_appears("760_221"));
}

fn day15() {
    assert_eq!(
        207_059,
        Grid::from(&read_file_to_lines("input/day15.txt")).play_until_no_enemies_remain()
    );
    assert_eq!(
        49_120,
        Grid::from(&read_file_to_lines("input/day15.txt"))
            .play_with_increasing_elf_attack_power_until_elves_win_without_a_single_loss()
    );
}

fn day16() {
    assert_eq!(
        605,
        number_of_samples_matching_3_or_more_opcodes(&read_file_to_lines("input/day16.txt"))
    );
    assert_eq!(
        653,
        figure_out_op_code_numbers_and_run_program(&read_file_to_lines("input/day16.txt"))
    );
}

fn day17() {
    let mut ground = Ground::from(read_file_to_lines("input/day17.txt").as_slice());
    let tiles_reachable_by_water = ground.tiles_reachable_by_water();
    assert_eq!(31949, tiles_reachable_by_water);

    let mut ground = Ground::from(read_file_to_lines("input/day17.txt").as_slice());
    let retained_water_count = ground.water_retained_when_spring_runs_dry();
    assert_eq!(26384, retained_water_count);
}

fn day18() {
    let mut grid = LumberCollectionArea::from(read_file_to_lines("input/day18.txt").as_slice());
    grid.run(10);
    let (trees, lumberyards) = grid.tree_and_lumberyard_count();
    assert_eq!(605_154, trees * lumberyards);

    let mut grid = LumberCollectionArea::from(read_file_to_lines("input/day18.txt").as_slice());
    grid.run(1_000_000_000);
    let (trees, lumberyards) = grid.tree_and_lumberyard_count();
    assert_eq!(200_364, trees * lumberyards);
}

fn day19() {
    let program = read_file_to_lines("input/day19.txt");
    assert_eq!(1872, Device::default().run_program(&program));
    assert_eq!(
        18_992_592, // 1 + 2 + 5 + 10 + 1_055_143 + 2_110_286 + 5_275_715 + 10_551_430,
        sum_of_divisors(10_551_430)
    );
}

fn day20() {
    let base = Base::from(read_file_to_lines("input/day20.txt")[0].as_str());
    assert_eq!(4360, base.furthest_room_from_start());

    let base = Base::from(read_file_to_lines("input/day20.txt")[0].as_str());
    assert_eq!(8509, base.number_of_rooms_at_least_1000_doors_away());
}

fn day21() {
    let halting_value = *reversed_day21program(1).first().unwrap();
    assert_eq!(103548, halting_value);

    let halting_values = reversed_day21program(usize::MAX);
    assert_eq!(14256686, *halting_values.last().unwrap());
}

fn day22() {
    let mut cave = full_cave();
    assert_eq!(10115, cave.risk_level());

    let mut cave = full_cave();
    assert_eq!(990, cave.shortest_path_len());
}

fn day23() {
    assert_eq!(
        417,
        count_nanobots_in_signal_range(read_file_to_lines("input/day23.txt"))
    );

    assert_eq!(
        112997634,
        distance_to_origin_from_point_within_range_of_most_nanobots(read_file_to_lines(
            "input/day23.txt"
        ))
    );
}
