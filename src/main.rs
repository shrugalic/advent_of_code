#[macro_use]
extern crate lazy_static;

use crate::day01::{day01input, sum_of_fuel_needed_for};
use crate::day02::process_noun_and_verb;
use crate::day03::{combined_step_to_closest_intersection, distance_of_closest_intersection};
use crate::day04::IsValidPassword;
use crate::day05::{day05_puzzle_input, process_int_code_with_input};
use crate::day06::{count_orbit_transfers, count_orbits, day06_puzzle_input, OrbitCount};
use crate::day07::{
    day07_puzzle_input, max_thrust_in_feedback_loop_mode, max_thrust_in_serial_mode,
};
use crate::day08::{day08_puzzle_input, SpaceImageFormat};
use crate::day09::{day09_puzzle_input, IntCodeComputer};
use crate::day10::{day10_puzzle_input, MonitoringStation, Point};
use crate::day11::{day11_puzzle_input, Robot};
use crate::day12::{day12_puzzle_input, lcm, Jupiter};
use crate::day13::{day_13_puzzle_input, stats, ArcadeCabinet, Tile};
use crate::day14::{day14_puzzle_input, Chemical, NanoFactory};
use crate::day16::{day_16_puzzle_input, FlawedFrequencyTransmission};
use rayon::prelude::*;
use std::cmp::Ordering;

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
mod day16;

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
    day16();
}

fn day01() {
    assert_eq!(sum_of_fuel_needed_for(&day01input()), 4943994);
}

fn day02() {
    assert_eq!(process_noun_and_verb(12, 2), 3516593);

    for noun in 0..=99 {
        for verb in 0..=99 {
            if process_noun_and_verb(noun, verb) == 19690720 {
                assert_eq!(7749, 100 * noun + verb);
            }
        }
    }
}

fn day03() {
    assert_eq!(
        distance_of_closest_intersection(
            "R1004,D53,L10,U126,R130,U533,R48,D185,L768,U786,L445,U694,L659,D237,R432,U147,R590,U200,R878,D970,L308,D134,R617,U431,L631,D548,L300,D509,R660,U698,L958,U170,R572,U514,R387,D385,L670,D374,R898,U870,L545,D262,L699,D110,R58,D84,R77,D58,L891,U9,R320,D914,L161,D148,L266,D334,R442,D855,R349,D618,R272,U514,R584,D269,R608,U542,L335,U855,L646,D678,R720,U325,L792,U60,L828,D915,L487,D253,L911,U907,R392,D981,R965,D725,R308,D574,L997,D332,L927,D855,R122,D5,L875,D336,L395,U697,R806,U420,R718,D575,L824,U397,L308,D988,L855,U332,R838,U853,L91,U778,R265,U549,L847,D665,L804,D768,L736,D201,L825,U87,L747,D375,L162,U336,R375,U754,R468,U507,R256,D107,L79,U871,L155,D667,L448,D847,L193,U263,R154,U859,R696,D222,R189,D307,R332,U522,L345,D961,L161,U274,L122,U931,L812,D852,R906,D269,R612,D723,L304,U944,R64,D20,R401,D260,L95,U278,R128,U637,L554,D650,L116,D720,R12,D434,R514,U379,L899,D359,R815,D843,L994,U775,R63,D942,R655,D91,L236,U175,L813,D572,R520,U812,L657,D935,L886,D178,R618,U260,R7,D953,L158,D471,R309,D858,R25,U746,R40,U832,L544,D311,R122,D224,L281,D699,R147,D310,R659,D662,L990,U160,L969,D335,L923,U201,R336,D643,R226,D91,R88,U350,L303,U20,L157,U987,L305,U766,R253,D790,R977,U482,R283,U793,R785,D799,L511,D757,L689,D841,L233,U742,L551,D466,R66,U579,L18,U838,R554,D143,L996,U557,L783,D799,R36,D563,L244,U440,L8,D945,L346,D747,L769,U661,L485,U965,L569,U952,R57,U773,L267,U453,R424,U66,R763,U105,R285,D870,L179,U548,L46,U914,L251,U194,L559,U736,R768,D917,R617,D55,R185,D464,L244",
            "L1005,D527,R864,D622,R482,D647,R29,U459,R430,D942,R550,D163,L898,U890,L271,D216,L52,U731,R715,U925,L614,U19,R687,D832,L381,U192,L293,D946,L642,D2,L124,U66,R492,U281,R181,U624,R294,U767,R443,U424,R241,D225,R432,D419,L647,U290,L647,D985,L694,D777,L382,D231,R809,D467,L917,D217,R422,U490,L873,D537,R176,U856,L944,D875,L485,D49,R333,D220,L354,U789,R256,D73,R905,U146,R798,D429,R111,D585,L275,D471,R220,D619,L680,U757,R580,U497,L620,U753,R58,U574,L882,U484,R297,D899,L95,D186,R619,D622,R65,U714,L402,U950,R647,D60,L659,U101,L917,D736,L531,U398,R26,U134,R837,U294,R364,D55,R254,D999,R868,U978,R434,U661,R362,D158,L50,D576,L146,D249,L562,D433,R206,D376,L650,U285,L427,D406,L526,D597,R557,U554,L463,D157,L811,U961,R648,D184,L962,U695,R138,U661,L999,U806,L413,U54,L865,U931,L319,U235,L794,D12,L456,D918,L456,U214,L739,D772,R90,D478,R23,D658,R919,D990,L307,D534,L40,D324,L4,U805,L605,U534,R727,U452,R733,D416,L451,U598,R215,D545,L563,D222,L295,D669,R706,U11,R44,D392,L518,D437,L634,U874,L641,U240,L11,D279,L153,U601,L238,U924,L292,D406,L360,D203,R874,D506,R806,U9,R713,D891,L587,U538,L867,D637,R889,U186,R728,D672,R573,U461,R222,D703,R178,U336,L896,D924,L445,D365,L648,U3,L734,U959,R344,U314,R331,D929,L364,D937,L896,D191,R218,U256,L975,D506,R510,D392,R878,U896,L177,U4,R516,D873,R57,D530,R140,D827,L263,U848,L88,U309,L801,U670,R874,D358,L49,D259,L188,U419,R705,D498,R496,U576,R808,D959,L861,U437,L618,D112,R725,D546,R338,U879,R522,U892,R230,D367,R901,D737,L942,D689,R976,D369,R157"
        ),
        266
    );
    assert_eq!(
        combined_step_to_closest_intersection(
            "R1004,D53,L10,U126,R130,U533,R48,D185,L768,U786,L445,U694,L659,D237,R432,U147,R590,U200,R878,D970,L308,D134,R617,U431,L631,D548,L300,D509,R660,U698,L958,U170,R572,U514,R387,D385,L670,D374,R898,U870,L545,D262,L699,D110,R58,D84,R77,D58,L891,U9,R320,D914,L161,D148,L266,D334,R442,D855,R349,D618,R272,U514,R584,D269,R608,U542,L335,U855,L646,D678,R720,U325,L792,U60,L828,D915,L487,D253,L911,U907,R392,D981,R965,D725,R308,D574,L997,D332,L927,D855,R122,D5,L875,D336,L395,U697,R806,U420,R718,D575,L824,U397,L308,D988,L855,U332,R838,U853,L91,U778,R265,U549,L847,D665,L804,D768,L736,D201,L825,U87,L747,D375,L162,U336,R375,U754,R468,U507,R256,D107,L79,U871,L155,D667,L448,D847,L193,U263,R154,U859,R696,D222,R189,D307,R332,U522,L345,D961,L161,U274,L122,U931,L812,D852,R906,D269,R612,D723,L304,U944,R64,D20,R401,D260,L95,U278,R128,U637,L554,D650,L116,D720,R12,D434,R514,U379,L899,D359,R815,D843,L994,U775,R63,D942,R655,D91,L236,U175,L813,D572,R520,U812,L657,D935,L886,D178,R618,U260,R7,D953,L158,D471,R309,D858,R25,U746,R40,U832,L544,D311,R122,D224,L281,D699,R147,D310,R659,D662,L990,U160,L969,D335,L923,U201,R336,D643,R226,D91,R88,U350,L303,U20,L157,U987,L305,U766,R253,D790,R977,U482,R283,U793,R785,D799,L511,D757,L689,D841,L233,U742,L551,D466,R66,U579,L18,U838,R554,D143,L996,U557,L783,D799,R36,D563,L244,U440,L8,D945,L346,D747,L769,U661,L485,U965,L569,U952,R57,U773,L267,U453,R424,U66,R763,U105,R285,D870,L179,U548,L46,U914,L251,U194,L559,U736,R768,D917,R617,D55,R185,D464,L244",
            "L1005,D527,R864,D622,R482,D647,R29,U459,R430,D942,R550,D163,L898,U890,L271,D216,L52,U731,R715,U925,L614,U19,R687,D832,L381,U192,L293,D946,L642,D2,L124,U66,R492,U281,R181,U624,R294,U767,R443,U424,R241,D225,R432,D419,L647,U290,L647,D985,L694,D777,L382,D231,R809,D467,L917,D217,R422,U490,L873,D537,R176,U856,L944,D875,L485,D49,R333,D220,L354,U789,R256,D73,R905,U146,R798,D429,R111,D585,L275,D471,R220,D619,L680,U757,R580,U497,L620,U753,R58,U574,L882,U484,R297,D899,L95,D186,R619,D622,R65,U714,L402,U950,R647,D60,L659,U101,L917,D736,L531,U398,R26,U134,R837,U294,R364,D55,R254,D999,R868,U978,R434,U661,R362,D158,L50,D576,L146,D249,L562,D433,R206,D376,L650,U285,L427,D406,L526,D597,R557,U554,L463,D157,L811,U961,R648,D184,L962,U695,R138,U661,L999,U806,L413,U54,L865,U931,L319,U235,L794,D12,L456,D918,L456,U214,L739,D772,R90,D478,R23,D658,R919,D990,L307,D534,L40,D324,L4,U805,L605,U534,R727,U452,R733,D416,L451,U598,R215,D545,L563,D222,L295,D669,R706,U11,R44,D392,L518,D437,L634,U874,L641,U240,L11,D279,L153,U601,L238,U924,L292,D406,L360,D203,R874,D506,R806,U9,R713,D891,L587,U538,L867,D637,R889,U186,R728,D672,R573,U461,R222,D703,R178,U336,L896,D924,L445,D365,L648,U3,L734,U959,R344,U314,R331,D929,L364,D937,L896,D191,R218,U256,L975,D506,R510,D392,R878,U896,L177,U4,R516,D873,R57,D530,R140,D827,L263,U848,L88,U309,L801,U670,R874,D358,L49,D259,L188,U419,R705,D498,R496,U576,R808,D959,L861,U437,L618,D112,R725,D546,R338,U879,R522,U892,R230,D367,R901,D737,L942,D689,R976,D369,R157"
        ),
        19242
    );
}

fn day04() {
    let range = 172851..=675869usize;
    let mut v = vec![];
    for pw in range {
        v.push(pw);
    }
    let valid_pw_count = v
        .par_iter()
        .filter(|pw| pw.is_valid_password())
        .collect::<Vec<&usize>>()
        .len();
    assert_eq!(1135, valid_pw_count);
}

fn day05() {
    let mut v = day05_puzzle_input();
    assert_eq!(process_int_code_with_input(&mut v, 1), Some(11049715));
    //
    let mut v = day05_puzzle_input();
    assert_eq!(process_int_code_with_input(&mut v, 5), Some(2140710));
}

fn day06() {
    assert_eq!(
        count_orbits(day06_puzzle_input()),
        OrbitCount::from(1605, 252842)
    );
    assert_eq!(
        count_orbit_transfers(
            "AAA)BBB
AAA)CCC",
            "BBB",
            "CCC"
        ),
        0
    );
}

fn day07() {
    let v = day07_puzzle_input();
    assert_eq!(max_thrust_in_serial_mode(&v), 87138);

    let input = day07_puzzle_input();
    assert_eq!(max_thrust_in_feedback_loop_mode(&input), 17279674);
}

fn day08() {
    let image = SpaceImageFormat::new(25, 6, day08_puzzle_input());
    let idx = image.idx_of_layer_with_fewest('0');
    let num = image.count_digits('1', idx) * image.count_digits('2', idx);
    assert_eq!(num, 2048);

    let image = SpaceImageFormat::new(25, 6, day08_puzzle_input());
    assert_eq!(image.decoded(), String::from("100101111010001011001001010010100001000110010101001111011100010101001011000100101000000100111101010010010100000010010010101001001010000001001001010010"));
}

fn day09() {
    let mut icc = IntCodeComputer::new(&mut day09_puzzle_input());
    assert_eq!(icc.process_int_code_with_input(1), Some(3518157894));

    let mut icc = IntCodeComputer::new(&mut day09_puzzle_input());
    assert_eq!(icc.process_int_code_with_input(2), Some(80379));
}

fn day10() {
    assert_eq!(MonitoringStation::from(day10_puzzle_input()).count, 253);

    let mut station = MonitoringStation::from(day10_puzzle_input());
    let vaporized = station.vaporized();
    assert_eq!(vaporized[199], Point(8, 15));
}

fn day11() {
    // part 2
    let mut robot = Robot::new(&mut day11_puzzle_input(), Some(1));
    robot.run();
    let min_x = robot.canvas.iter().map(|(p, _)| p.0).min().unwrap();
    let max_x = robot.canvas.iter().map(|(p, _)| p.0).max().unwrap();
    let min_y = robot.canvas.iter().map(|(p, _)| p.1).min().unwrap();
    let max_y = robot.canvas.iter().map(|(p, _)| p.1).max().unwrap();

    let mut y = max_y;
    while y >= min_y {
        let mut line: Vec<char> = vec![];
        for x in min_x..=max_x {
            if let Some(&color) = robot.canvas.get(&day11::Point(x, y)) {
                if color == 0 {
                    line.push('#');
                } else {
                    line.push(' ');
                }
            } else {
                line.push(' ');
            }
        }
        y -= 1;
    }
    assert_eq!(robot.painted_panels.len(), 249);
}

fn day12() {
    let mut jupiter = Jupiter::from(day12_puzzle_input());
    jupiter.steps(1000);
    assert_eq!(jupiter.total_energy(), 14907);

    let mut jupiter = Jupiter::from(day12_puzzle_input());
    let periods = jupiter.determine_periods();
    let count = lcm(periods.0, lcm(periods.1, periods.2));
    assert_eq!(count, 467_081_194_429_464);
}

fn day13() {
    let mut arcade = ArcadeCabinet::new(&mut day_13_puzzle_input());
    let tiles = arcade.run();
    let (block_count, _, _, _, _) = stats(&tiles);
    assert_eq!(block_count, 265);

    let mut arcade = ArcadeCabinet::new(&mut day_13_puzzle_input());
    let tiles: Vec<(day13::Point, Tile)> = arcade.play();
    assert_eq!(tiles.len(), 26947); // Score 13331
}

fn day14() {
    let mut nf = NanoFactory::from(day14_puzzle_input());
    let target = Chemical::new(1, "FUEL");
    assert_eq!(
        nf.count_reactant_to_make_wanted_product(&target, "ORE"),
        158482
    );

    let mut range = 6_309_864..10_000_000;
    let mut max_fuel = 0;
    while range.start + 1 < range.end {
        let mid = ((range.start + range.end) as f64 / 2.0).round() as usize;
        let fuel = Chemical::new(mid, "FUEL");
        let ore_count = NanoFactory::from(day14_puzzle_input())
            .count_reactant_to_make_wanted_product(&fuel, "ORE");
        match ore_count.cmp(&1_000_000_000_000usize) {
            Ordering::Less => {
                max_fuel = fuel.amount;
                range = mid..range.end;
            }
            Ordering::Equal => break,
            Ordering::Greater => {
                range = range.start..mid;
            }
        }
    }
    assert_eq!(max_fuel, 7993831);
}

fn day16() {
    let fft = FlawedFrequencyTransmission::from(day_16_puzzle_input());
    assert_eq!(fft.check_sum(100), "78009100");
    //
    let mut fft = FlawedFrequencyTransmission::from(day_16_puzzle_input());
    assert_eq!(fft.message(100), "37717791");
}
