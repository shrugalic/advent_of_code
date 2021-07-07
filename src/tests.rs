use crate::*;
use line_reader::*;

const TILE_2311: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";

#[cfg(test)]
#[test]
fn part1() {
    assert_eq!(
        product_of_corner_tile_ids(&read_file_to_lines("input.txt")),
        60145080587029
    );
}

#[test]
fn example1() {
    assert_eq!(
        product_of_corner_tile_ids(&read_file_to_lines("example1.txt")),
        1951 * 3079 * 2971 * 1171
    );
}

#[test]
fn example0() {
    assert_eq!(
        product_of_corner_tile_ids(&read_file_to_lines("example0.txt")),
        1951 * 2311 * 2729 * 1427
    );
}

#[test]
fn tile() {
    assert_eq!(
        Tile::from(read_str_to_lines(TILE_2311).as_slice()),
        Tile::new(2311, 210, 89, 231, 498)
    );
}

#[test]
fn reverse_border() {
    // reverse(0011010010) = 0100101100
    assert_eq!(Border::reversed(210), 300);
    // reverse(0001011001) = 1001101000
    assert_eq!(Border::reversed(89), 616);
    // reverse(0011100111) = 1110011100
    assert_eq!(Border::reversed(231), 924);
    // reverse(0111110010) = 0100111110
    assert_eq!(Border::reversed(498), 318);
}

#[test]
fn borders_h_flip() {
    let mut actual = Tile::from(read_str_to_lines(TILE_2311).as_slice());
    actual.flip_h();
    assert_eq!(actual, Tile::new(2311, 300, 498, 924, 89));
}

#[test]
fn borders_rotate_cw() {
    let mut actual = Tile::from(read_str_to_lines(TILE_2311).as_slice());
    assert_eq!(actual, Tile::new(2311, 210, 89, 231, 498));
    actual.rotate_cw();
    assert_eq!(actual, Tile::new(2311, 318, 210, 616, 231));
}

// #[test]
#[allow(dead_code)]
fn all_8_configs() {
    let mut tile = Tile::from(read_str_to_lines(TILE_2311).as_slice());
    println!("{:?}", tile);
    tile.rotate_cw();
    println!("{:?}", tile);
    tile.rotate_cw();
    println!("{:?}", tile);
    tile.rotate_cw();
    println!("{:?}", tile);

    // These next four tuples contain the same numbers as above, but in different orders.
    // For example:
    // first (210, 89, 231, 498)
    //   7th (498, 231, 89, 210)
    // So when only counting values these latter four permutations don't matter
    tile.flip_h();
    println!("{:?}", tile);
    tile.rotate_cw();
    println!("{:?}", tile);
    tile.rotate_cw();
    println!("{:?}", tile);
    tile.rotate_cw();
    println!("{:?}", tile);
}

#[test]
fn part2_example1() {
    assert_eq!(
        count_hashes_not_part_of_sea_monsters(&read_file_to_lines("example1.txt")),
        273
    );
}
