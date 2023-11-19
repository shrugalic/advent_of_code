use crate::day01::{day01_input, find_two};
use crate::day02::{day02_read_file, is_valid_for_part_1, is_valid_for_part_2};
use crate::day03::{traverse_map, Slope, PART2_SLOPES};
use crate::day04::{
    count_entries_with_required_fields, count_entries_with_required_fields_and_valid_values,
};
use crate::day05::{row_and_col, seat_id};
use crate::day06::{sum_of_unique_yes_answers_per_group, sum_of_unique_yes_answers_per_group2};
use crate::day07::{number_of_bags_within_bag_of, number_of_possible_bags_that_can_hold};
use crate::day08::{fix_program_until_no_more_infinite_loop, run_program_until_infinite_loop};
use crate::day09::{find_encryrption_weakness, first_invalid_digit};
use crate::day10::{adapter_combination_count, product_of_joltage_diff_1_and_3_counts};
use crate::day11::{
    count_occupied_seats_after_seating_process_became_stable, part1_seat_selection_strategy,
    part2_seat_selection_strategy,
};
use crate::day12::{
    distance_from_origin_after_following_instructions,
    distance_from_origin_after_following_instructions_part2,
};
use crate::day13::{day13_part1impl, day13_part2impl};
use crate::day14::{day14_part1impl, day14_part2impl};
use crate::day15::{the_2020th_number_spoken, the_30_000_000th_number_spoken};
use crate::day16::{invalid_error_sum, multiply_departure_fields};
use crate::day17::pocket_dimension_map_4d::PocketDimensionMap4D;
use crate::day17::pocket_dimension_vec::PocketDimensionVec;
use crate::day17::{day17_input, ExecutableCycle, PocketDimension};
use crate::day18::{evaluate_day18_part1, evaluate_day18_part2};
use crate::day19::alternate_number_of_messages_matching_rule_0;
use crate::day20::{count_hashes_not_part_of_sea_monsters, product_of_corner_tile_ids};
use crate::day21::{
    allergen_free_ingredient_appearance_count, canonical_dangerous_ingredient_list,
};
use crate::day22::{winning_players_score, winning_recursive_combat_players_score};
use crate::day23::{
    input_to_vec, label_part1, label_part2, play, Label, DAY23_PUZZLE_INPUT, DAY23_ROUND_COUNT,
};
use crate::day24::{black_tile_count, iterate_for_given_number_of_days};
use crate::day25::{find_encryption_key, DAY_25_PUZZLE_INPUT};
use crate::line_reader::read_file_to_lines;

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
mod day24;
mod day25;
mod line_reader;

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
    day24();
    day25();
}

fn day01() {
    let input = day01_input();
    let (a, b) = find_two(&2020, &input).unwrap();
    assert_eq!(317 * 1703, a * b);

    for a in input.iter() {
        let diff = 2020 - a;
        if let Some((b, c)) = find_two(&diff, &input) {
            assert_eq!(1081 * 315 * 624, a * b * c);
            assert_eq!(315 * 1081 * 624, a * b * c);
            assert_eq!(624 * 1081 * 315, a * b * c);
        }
    }
}

fn day02() {
    let tuples = day02_read_file("input/day02.txt");
    let count = tuples
        .iter()
        .filter(|(range, letter, password)| is_valid_for_part_1(range, letter, password))
        .count();
    assert_eq!(454, count);

    let count = tuples
        .iter()
        .filter(|(range, letter, password)| is_valid_for_part_2(range, letter, password))
        .count();
    assert_eq!(649, count);
}

fn day03() {
    let map = read_file_to_lines("input/day03.txt");
    assert_eq!(traverse_map(&map, &Slope { right: 3, down: 1 }), 244);

    let product = PART2_SLOPES
        .iter()
        .map(|slope| traverse_map(&map, slope))
        .reduce(|a, b| a * b)
        .unwrap();

    assert_eq!(product, 9406609920);
}

fn day04() {
    assert_eq!(
        count_entries_with_required_fields(&read_file_to_lines("input/day04.txt")),
        226
    );

    assert_eq!(
        count_entries_with_required_fields_and_valid_values(&read_file_to_lines("input/day04.txt")),
        160
    );
}

fn day05() {
    let lines = read_file_to_lines("input/day05.txt");
    let max_seat_id = lines
        .iter()
        .map(|line| seat_id(row_and_col(line)))
        .max()
        .expect("Empty list?");
    assert_eq!(max_seat_id, 970);

    let lines = read_file_to_lines("input/day05.txt");
    let mut seat_ids: Vec<usize> = lines
        .iter()
        .map(|line| seat_id(row_and_col(line)))
        .collect();
    seat_ids.sort_unstable();
    let neighbors: Vec<usize> = seat_ids
        .iter()
        .zip(seat_ids.iter().skip(1))
        .zip(seat_ids.iter().skip(2))
        .filter_map(|((&a, &b), &c)| {
            // println!("{} < {} < {}", a, b, c);
            if a != b - 1 || b + 1 != c {
                Some(b)
            } else {
                None
            }
            // } else
        })
        .collect();
    assert_eq!(neighbors.len(), 2);
    assert_eq!((neighbors[0] + neighbors[1]) / 2, 587);
}

fn day06() {
    assert_eq!(
        sum_of_unique_yes_answers_per_group(&read_file_to_lines("input/day06.txt")),
        6437
    );

    assert_eq!(
        sum_of_unique_yes_answers_per_group2(&read_file_to_lines("input/day06.txt")),
        3229
    );
}

fn day07() {
    assert_eq!(
        number_of_possible_bags_that_can_hold(
            &"shiny gold",
            &read_file_to_lines("input/day07.txt")
        ),
        192
    );

    assert_eq!(
        number_of_bags_within_bag_of(&"shiny gold", &read_file_to_lines("input/day07.txt")),
        12128
    );
}

fn day08() {
    assert_eq!(
        run_program_until_infinite_loop(&read_file_to_lines("input/day08.txt")),
        (1810, true)
    );

    assert_eq!(
        fix_program_until_no_more_infinite_loop(&read_file_to_lines("input/day08.txt")),
        (969, false)
    );
}

fn day09() {
    assert_eq!(
        first_invalid_digit(&read_file_to_lines("input/day09.txt"), 25),
        258585477
    );

    assert_eq!(
        find_encryrption_weakness(&read_file_to_lines("input/day09.txt"), 258585477),
        36981213
    );
}

fn day10() {
    assert_eq!(
        product_of_joltage_diff_1_and_3_counts(&read_file_to_lines("input/day10.txt")),
        2080
    );

    assert_eq!(
        adapter_combination_count(&read_file_to_lines("input/day10.txt")),
        6908379398144
    );
}

fn day11() {
    assert_eq!(
        count_occupied_seats_after_seating_process_became_stable(
            &read_file_to_lines("input/day11.txt"),
            &part1_seat_selection_strategy
        ),
        2481
    );

    assert_eq!(
        count_occupied_seats_after_seating_process_became_stable(
            &read_file_to_lines("input/day11.txt"),
            &part2_seat_selection_strategy
        ),
        2227
    );
}

fn day12() {
    assert_eq!(
        distance_from_origin_after_following_instructions(&read_file_to_lines("input/day12.txt")),
        923
    );

    assert_eq!(
        distance_from_origin_after_following_instructions_part2(&read_file_to_lines(
            "input/day12.txt"
        )),
        24769
    );
}

fn day13() {
    assert_eq!(
        day13_part1impl(&read_file_to_lines("input/day13.txt")),
        3269
    );

    assert_eq!(
        day13_part2impl(&read_file_to_lines("input/day13.txt"),),
        672754131923874
    );
}

fn day14() {
    assert_eq!(
        day14_part1impl(&read_file_to_lines("input/day14.txt")),
        6317049172545
    );

    assert_eq!(
        day14_part2impl(&read_file_to_lines("input/day14.txt"),),
        3434009980379
    );
}

fn day15() {
    assert_eq!(the_2020th_number_spoken(&[0, 6, 1, 7, 2, 19, 20]), 706);

    assert_eq!(
        the_30_000_000th_number_spoken(&[0, 6, 1, 7, 2, 19, 20]),
        19331
    );
}

fn day16() {
    assert_eq!(
        invalid_error_sum(&read_file_to_lines("input/day16.txt")),
        19240
    );

    assert_eq!(
        multiply_departure_fields(&read_file_to_lines("input/day16.txt")),
        21095351239483
    );
}

fn day17() {
    let initial = PocketDimensionVec::from(&day17_input());
    let next = initial
        .execute_cycle()
        .execute_cycle()
        .execute_cycle()
        .execute_cycle()
        .execute_cycle()
        .execute_cycle();
    assert_eq!(next.active_cube_count(), 291);

    let initial = PocketDimensionMap4D::from(&day17_input());
    let next = initial
        .execute_cycle()
        .execute_cycle()
        .execute_cycle()
        .execute_cycle()
        .execute_cycle()
        .execute_cycle();
    assert_eq!(next.active_cube_count(), 1524);
}

fn day18() {
    assert_eq!(
        evaluate_day18_part1(&read_file_to_lines("input/day18.txt")),
        3348222486398
    );

    assert_eq!(
        evaluate_day18_part2(&read_file_to_lines("input/day18.txt")),
        43423343619505
    );
}

fn day19() {
    assert_eq!(
        alternate_number_of_messages_matching_rule_0(&read_file_to_lines("input/day19.txt")),
        156
    );

    assert_eq!(
        alternate_number_of_messages_matching_rule_0(&read_file_to_lines("input/day19_2.txt")),
        363
    );
}

fn day20() {
    assert_eq!(
        product_of_corner_tile_ids(&read_file_to_lines("input/day20.txt")),
        60145080587029
    );

    assert_eq!(
        count_hashes_not_part_of_sea_monsters(&read_file_to_lines("input/day20.txt")),
        1901
    );
}

fn day21() {
    assert_eq!(
        allergen_free_ingredient_appearance_count(&read_file_to_lines("input/day21.txt")),
        2517
    );

    assert_eq!(
        canonical_dangerous_ingredient_list(&read_file_to_lines("input/day21.txt")),
        "rhvbn,mmcpg,kjf,fvk,lbmt,jgtb,hcbdb,zrb".to_string()
    );
}

fn day22() {
    assert_eq!(
        winning_players_score(&read_file_to_lines("input/day22.txt")),
        35299
    );

    assert_eq!(
        winning_recursive_combat_players_score(&read_file_to_lines("input/day22.txt")),
        33266
    );
}

fn day23() {
    assert_eq!(
        label_part1(play(&mut input_to_vec(DAY23_PUZZLE_INPUT), 100)),
        input_to_vec("82934675")
    );

    let mut cups: Vec<Label> = (1..=1_000_000).into_iter().collect();
    input_to_vec(DAY23_PUZZLE_INPUT)
        .iter()
        .enumerate()
        .for_each(|(i, v)| cups[i] = *v);

    assert_eq!(
        label_part2(play(&mut cups, DAY23_ROUND_COUNT)),
        749102 * 633559
    );
}

fn day24() {
    assert_eq!(
        black_tile_count(&read_file_to_lines("input/day24.txt")),
        287
    );

    assert_eq!(
        iterate_for_given_number_of_days(&read_file_to_lines("input/day24.txt"), 100),
        3636
    );
}

fn day25() {
    assert_eq!(
        find_encryption_key(DAY_25_PUZZLE_INPUT.0, DAY_25_PUZZLE_INPUT.1),
        12929
    );
}
