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
