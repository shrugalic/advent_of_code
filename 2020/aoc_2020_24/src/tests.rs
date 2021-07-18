use crate::*;
use line_reader::{read_file_to_lines, read_str_to_lines};

const EXAMPLE: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

#[test]
fn coordinate_of_path_esenee() {
    let expected = Coordinate { x: 3, y: -3, z: 0 };
    let actual = Coordinate::from(Path::from("esenee"));
    assert_eq!(expected, actual);
}

#[test]
fn coordinate_of_path_esew() {
    let expected = Coordinate { x: 0, y: -1, z: 1 };
    let actual = Coordinate::from(Path::from("esew"));
    assert_eq!(expected, actual);
}

#[test]
fn coordinate_of_path_nwwswee() {
    let expected = Coordinate { x: 0, y: 0, z: 0 };
    let actual = Coordinate::from(Path::from("nwwswee"));
    assert_eq!(expected, actual);
}

#[test]
fn example_1() {
    assert_eq!(black_tile_count(&read_str_to_lines(EXAMPLE)), 10);
}

#[test]
fn part_1() {
    assert_eq!(black_tile_count(&read_file_to_lines("input.txt")), 287);
}

#[test]
fn part_2_example_1_day_1() {
    assert_eq!(
        iterate_for_given_number_of_days(&read_str_to_lines(EXAMPLE), 1),
        15
    );
}

#[test]
fn part_2_example_1_day_2() {
    assert_eq!(
        iterate_for_given_number_of_days(&read_str_to_lines(EXAMPLE), 2),
        12
    );
}

#[test]
fn part_2_example_1_day_100() {
    assert_eq!(
        iterate_for_given_number_of_days(&read_str_to_lines(EXAMPLE), 100),
        2208
    );
}

#[test]
fn part_2() {
    assert_eq!(
        iterate_for_given_number_of_days(&read_file_to_lines("input.txt"), 100),
        3636
    );
}
