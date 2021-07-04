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
        1
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
        Tile {
            id: 2311,
            borders: Borders::new(210, 89, 231, 498)
        }
    );
}

#[test]
fn complement_border() {
    // ..##.#..#. = 0011010010 = 210
    // __________ = 1100101101 = 813
    assert_eq!(Border::from(210).complement(), 1023 - 210);
    // ...#.##..# = 0001011001 = 89
    // __________ = 1110100110 = 934
    assert_eq!(Border::from(89).complement(), 1023 - 89);
    // ..###..### = 0011100111 = 231
    // __________ = 1100011000 = 792
    assert_eq!(Border::from(231).complement(), 1023 - 231);
    // .#####..#. = 0111110010 = 498
    // __________ = 1000001101 = 525
    assert_eq!(Border::from(498).complement(), 1023 - 498);
}

#[test]
fn reverse_border() {
    // reverse(0011010010) = 0100101100
    assert_eq!(Border::from(210).reverse(), 300);
    // reverse(0001011001) = 1001101000
    assert_eq!(Border::from(89).reverse(), 616);
    // reverse(0011100111) = 1110011100
    assert_eq!(Border::from(231).reverse(), 924);
    // reverse(0111110010) = 0100111110
    assert_eq!(Border::from(498).reverse(), 318);
}

#[test]
fn borders_h_flip() {
    let mut actual = Tile::from(read_str_to_lines(TILE_2311).as_slice());
    actual.borders.flip_h();
    assert_eq!(
        actual,
        Tile {
            id: 2311,
            borders: Borders::new(300, 498, 924, 89)
        }
    );
}

#[test]
fn borders_rotate_cw() {
    let mut actual = Tile::from(read_str_to_lines(TILE_2311).as_slice());
    assert_eq!(
        actual,
        Tile {
            id: 2311,
            borders: Borders::new(210, 89, 231, 498)
        }
    );
    actual.borders.rotate_cw();
    assert_eq!(
        actual,
        Tile {
            id: 2311,
            borders: Borders::new(318, 210, 616, 231)
        }
    );
}
#[test]
fn all_8_configs() {
    let mut tile = Tile::from(read_str_to_lines(TILE_2311).as_slice());
    println!("{}", tile);
    tile.borders.rotate_cw();
    println!("{}", tile);
    tile.borders.rotate_cw();
    println!("{}", tile);
    tile.borders.rotate_cw();
    println!("{}", tile);

    // These next four tuples contain the same numbers as above, but in different orders.
    // For example:
    // first (210, 89, 231, 498)
    //   7th (498, 231, 89, 210)
    // So when only counting values these latter four permutations don't matter
    tile.borders.flip_h();
    println!("{}", tile);
    tile.borders.rotate_cw();
    println!("{}", tile);
    tile.borders.rotate_cw();
    println!("{}", tile);
    tile.borders.rotate_cw();
    println!("{}", tile);
}
