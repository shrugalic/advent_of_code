use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};

trait Flippable {
    fn flip_h(&mut self);
}

trait Rotatable {
    fn rotate_cw(&mut self);
}

#[derive(PartialEq, Clone)]
struct Border(u16);

impl From<&str> for Border {
    fn from(s: &str) -> Self {
        Border(s.chars().fold(0_u16, |e, c| e << 1 | (c == '#') as u16))
    }
}

impl From<u16> for Border {
    fn from(value: u16) -> Self {
        Border(value)
    }
}

impl Debug for Border {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Border {
    fn reverse(&mut self) {
        self.0 = Border::reversed(self.0);
    }

    fn reversed(value: u16) -> u16 {
        let bits_per_border = 10;
        value.reverse_bits() >> (16 - bits_per_border)
    }
}

const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

#[derive(PartialEq, Clone)]
struct Tile {
    id: usize,
    borders: [Border; 4],
    contents: Square<char>,
}

impl<T> Flippable for Square<T> {
    fn flip_h(&mut self) {
        self.columns.make_contiguous();
        self.columns.as_mut_slices().0.reverse();
    }
}
impl<T> Rotatable for Square<T>
where
    T: Clone + Copy,
{
    fn rotate_cw(&mut self) {
        let previous = self.columns.clone();
        let side_len = self.columns.len();
        for (col_idx, prev_col) in previous.iter().enumerate() {
            for (row_idx, prev_val) in prev_col.0.iter().enumerate() {
                let new_col = side_len - 1 - row_idx;
                let new_row = col_idx;
                self.columns[new_col].0[new_row] = *prev_val;
            }
        }
    }
}

impl<T> From<&[T]> for Tile
where
    T: AsRef<str> + Debug,
{
    fn from(input: &[T]) -> Self {
        let (head, tail) = input.split_first().unwrap();
        let id: usize = head
            .as_ref()
            .matches(char::is_numeric)
            .collect::<String>()
            .parse()
            .unwrap();
        let right: String = tail
            .iter()
            .map(|line| line.as_ref().chars().last().unwrap())
            .collect();
        let left: String = tail
            .iter()
            .map(|line| line.as_ref().chars().next().unwrap())
            .collect();
        let borders = [
            Border::from(tail[0].as_ref()),
            Border::from(right.as_str()),
            Border::from(tail[tail.len() - 1].as_ref()),
            Border::from(left.as_str()),
        ];
        let content_lines: Vec<String> = tail
            .iter()
            .skip(1)
            .take(tail.len() - 2)
            .map(|line| {
                let len = line.as_ref().len();
                line.as_ref().chars().skip(1).take(len - 2).collect()
            })
            .collect();
        let mut content_columns: VecDeque<Column<char>> =
            VecDeque::from(vec![
                Column(VecDeque::from(vec!['x'; content_lines.len()]));
                content_lines.len()
            ]);
        for (row, line) in content_lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                content_columns[col].0[row] = ch;
            }
        }
        Tile {
            id,
            borders,
            contents: Square {
                columns: content_columns,
            },
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tile {}: ({:?}, {:?}, {:?}, {:?})",
            self.id,
            self.borders[TOP],
            self.borders[RIGHT],
            self.borders[BOTTOM],
            self.borders[LEFT]
        )
    }
}

impl Flippable for Tile {
    fn flip_h(&mut self) {
        self.borders[TOP].reverse();
        self.borders[BOTTOM].reverse();
        self.borders.swap(LEFT, RIGHT);

        self.contents.flip_h();
    }
}

impl Rotatable for Tile {
    fn rotate_cw(&mut self) {
        // One clockwise rotation with left as tmp element:
        // left -> top -> right -> bottom -> left
        self.borders.swap(TOP, LEFT);
        // top is old left && left is old top
        self.borders.swap(RIGHT, LEFT);
        // right is old top &&  left is old right
        self.borders.swap(BOTTOM, LEFT);
        // bottom is old right && left is old bottom

        self.borders[TOP].reverse();
        self.borders[BOTTOM].reverse();

        self.contents.rotate_cw();
    }
}

impl Tile {
    #[cfg(test)]
    fn new(id: usize, top: u16, right: u16, bottom: u16, left: u16) -> Tile {
        Tile {
            id,
            borders: [Border(top), Border(right), Border(bottom), Border(left)],
            contents: Square::new(),
        }
    }

    fn is_any_border_matching(&self, value: u16) -> bool {
        self.top_value() == value
            || self.right_value() == value
            || self.bottom_value() == value
            || self.left_value() == value
            // Value might be reversed depending on rotation/flip of tile
            || Border::reversed(self.top_value()) == value
            || Border::reversed(self.right_value()) == value
            || Border::reversed(self.bottom_value()) == value
            || Border::reversed(self.left_value()) == value
    }

    fn top_value(&self) -> u16 {
        self.borders[TOP].0
    }
    fn right_value(&self) -> u16 {
        self.borders[RIGHT].0
    }
    fn bottom_value(&self) -> u16 {
        self.borders[BOTTOM].0
    }
    fn left_value(&self) -> u16 {
        self.borders[LEFT].0
    }

    fn adapted_to_match(mut self, target_value: u16, at_loc: usize) -> Self {
        if !self.rotated_to_have_matching(target_value, at_loc) {
            self.flip_h();
            if !self.rotated_to_have_matching(target_value, at_loc) {
                panic!("Could not adapt to have bottom match {}!", target_value)
            }
        }
        self
    }

    fn rotated_to_have_matching(&mut self, target_value: u16, at_loc: usize) -> bool {
        let mut i = 0;
        while self.borders[at_loc].0 != target_value && i < 4 {
            self.rotate_cw();
            i += 1;
        }
        self.borders[at_loc].0 == target_value
    }
}

pub(crate) fn product_of_corner_tile_ids<T>(input: &[T]) -> usize
where
    T: AsRef<str> + Debug,
{
    let square = form_square(input);
    square.top_left_corner().id
        * square.top_right_corner().id
        * square.bottom_left_corner().id
        * square.bottom_right_corner().id
}

const SEA_MONSTER: [(usize, usize); 15] = [
    (18, 0),
    (0, 1),
    (5, 1),
    (6, 1),
    (11, 1),
    (12, 1),
    (17, 1),
    (18, 1),
    (19, 1),
    (1, 2),
    (4, 2),
    (7, 2),
    (10, 2),
    (13, 2),
    (16, 2),
];

pub(crate) fn count_hashes_not_part_of_sea_monsters<T>(input: &[T]) -> usize
where
    T: AsRef<str> + Debug,
{
    let tile_square: Square<Tile> = form_square(input);
    let mut char_square = convert_to_big_square(&tile_square);

    char_square.rotate_and_flip_until_sea_monsters_are_found();

    let total_hashes = char_square
        .columns
        .iter()
        .map(|col| col.0.iter().filter(|c| c == &&'#').count())
        .sum();
    total_hashes
}

fn convert_to_big_square(tile_square: &Square<Tile>) -> Square<char> {
    let tile_count = tile_square.len();
    let chars_per_tile = tile_square.columns[0].0[0].contents.len();
    let square_len = tile_count * chars_per_tile;
    let mut char_square: Square<char> = Square {
        columns: VecDeque::from(vec![
            Column(VecDeque::from(vec![' '; square_len]));
            square_len
        ]),
    };
    // println!(
    //     "tile_count * chars_per_tile = {} * {} = {} square_len",
    //     tile_count, chars_per_tile, square_len
    // );
    for t_row in 0..tile_count {
        for t_col in 0..tile_count {
            let tile = &tile_square.columns[t_col].0[t_row];
            for row in 0..chars_per_tile {
                for col in 0..chars_per_tile {
                    char_square.columns[t_col * chars_per_tile + col].0
                        [t_row * chars_per_tile + row] = tile.contents.columns[col].0[row];
                }
            }
        }
    }
    char_square
}

impl Square<char> {
    fn rotate_and_flip_until_sea_monsters_are_found(&mut self) {
        let mut counter = 0;
        while !self.marked_sea_monsters() && counter < 8 {
            // println!("{}\n", self.as_string());
            counter += 1;
            if counter % 4 == 0 {
                // println!("flipped");
                self.flip_h();
            } else {
                // println!("rotated");
                self.rotate_cw();
            }
        }
        // println!("{}\n", self.as_string());
    }

    fn marked_sea_monsters(&mut self) -> bool {
        let square_len = self.columns.len();
        let monster_max_col = SEA_MONSTER.iter().map(|(col, _)| col).max().unwrap();
        let monster_max_row = SEA_MONSTER.iter().map(|(_, row)| row).max().unwrap();
        // println!(
        //     "Sea monster max-col * max-row = {} * {}",
        //     monster_max_col, monster_max_row
        // );
        let mut found_one = false;
        for col in 0..(square_len - monster_max_col) {
            for row in 0..(square_len - monster_max_row) {
                if SEA_MONSTER.iter().all(|(c, r)| {
                    let ch = self.columns[col + c].0[row + r];
                    ch == '#' || ch == 'O'
                }) {
                    found_one = true;
                    // println!("Found one!");
                    SEA_MONSTER
                        .iter()
                        .for_each(|(c, r)| self.columns[col + c].0[row + r] = 'O');
                    // println!("{}", self.as_string());
                }
            }
        }
        found_one
    }
}
fn form_square<T>(input: &[T]) -> Square<Tile>
where
    T: AsRef<str> + Debug,
{
    let tiles: Vec<Tile> = input
        .split(|line| line.as_ref().is_empty())
        .map(Tile::from)
        .collect();
    arrange_as_square(tiles)
}

fn arrange_as_square(mut tiles: Vec<Tile>) -> Square<Tile> {
    let side_len = print_stats(&mut tiles);
    let mut square = Square::new();
    while square.len() < side_len {
        let seed = square.find_seed(&mut tiles);
        // println!("Rest: {:?}", tiles);

        let mut column = Column::from(seed);
        column.attach_matching_tiles_to_top_and_bottom(&mut tiles);
        // assert_eq!(column.len(), side_len);

        square.attach(column);
    }
    square
}

fn print_stats(tiles: &mut Vec<Tile>) -> usize {
    let side_len = (tiles.len() as f64).sqrt() as usize;
    // let sides = 4 * (side_len - 2);
    // let insides = (side_len - 2) * (side_len - 2);
    // println!(
    //     "Square of {} * {} tiles has 4 corners, {} sides, {} insides",
    //     side_len, side_len, sides, insides
    // );
    side_len
}

#[derive(PartialEq, Clone)]
struct Square<T> {
    columns: VecDeque<Column<T>>,
}

impl<T> Square<T> {
    fn new() -> Self {
        Self {
            columns: VecDeque::new(),
        }
    }

    fn len(&self) -> usize {
        self.columns.len()
    }
}

impl Square<Tile> {
    fn left_column(&self) -> &Column<Tile> {
        &self.columns[0]
    }
    fn right_column(&self) -> &Column<Tile> {
        &self.columns[self.columns.len() - 1]
    }
    fn add_left(&mut self, column: Column<Tile>) {
        self.columns.push_front(column)
    }
    fn add_right(&mut self, column: Column<Tile>) {
        self.columns.push_back(column)
    }
    fn is_empty(&self) -> bool {
        self.columns.is_empty()
    }
    fn top_left_corner(&self) -> &Tile {
        self.left_column().top_elem()
    }
    fn top_right_corner(&self) -> &Tile {
        self.right_column().top_elem()
    }
    fn bottom_left_corner(&self) -> &Tile {
        self.left_column().bottom_elem()
    }
    fn bottom_right_corner(&self) -> &Tile {
        self.right_column().bottom_elem()
    }
}

impl Square<Tile> {
    fn attach(&mut self, mut column: Column<Tile>) {
        if self.is_empty() {
            self.add_left(column);
        } else {
            // check left: as-is, flipped, rotated or rotated & flipped
            if column.adapted_to_match_to_the_left_of(self.left_column()) {
                // println!("Column matched to the left!");
                self.add_left(column);
            } else if column.adapted_to_match_to_the_right_of(self.right_column()) {
                // println!("Column matched to the right!");
                self.add_right(column);
            } else {
                panic!("Column didn't match!");
            }
        }
    }

    fn find_seed(&self, mut tiles: &mut Vec<Tile>) -> Tile {
        if self.is_empty() {
            // println!("First column root: {}", tiles[0]);
            tiles.remove(0)
        } else if let Some(id) = index_of_tile_with_border_matching(
            &mut tiles,
            self.left_column().top_elem().left_value(),
        ) {
            tiles
                .remove(id)
                .adapted_to_match(self.left_column().top_elem().left_value(), RIGHT)
        } else if let Some(id) = index_of_tile_with_border_matching(
            &mut tiles,
            self.right_column().top_elem().right_value(),
        ) {
            tiles
                .remove(id)
                .adapted_to_match(self.right_column().top_elem().right_value(), LEFT)
        } else {
            panic!("Found no column matching either side!");
        }
    }
}

#[derive(PartialEq, Clone)]
struct Column<T>(VecDeque<T>);

impl<T> Column<T>
where
    T: Flippable + Rotatable,
{
    fn len(&self) -> usize {
        self.0.len()
    }

    fn top_elem(&self) -> &T {
        &self.0[0]
    }

    fn bottom_elem(&self) -> &T {
        &self.0[self.0.len() - 1]
    }

    fn flip_h(&mut self)
    where
        T: Flippable,
    {
        self.0.iter_mut().for_each(|t| t.flip_h());
    }

    fn rotate_180(&mut self) {
        self.0.make_contiguous();
        self.0.as_mut_slices().0.reverse();
        self.0.iter_mut().for_each(|t| {
            t.rotate_cw();
            t.rotate_cw();
        });
    }
}

impl Column<Tile> {
    fn adapted_to_match_to_the_left_of(&mut self, other: &Column<Tile>) -> bool {
        let top_left = other.top_elem().left_value();
        if self.top_elem().right_value() == top_left {
            // check rest of column
        } else if self.top_elem().left_value() == top_left {
            self.flip_h();
            assert_eq!(self.top_elem().right_value(), top_left)
        } else if self.bottom_elem().left_value() == Border::reversed(top_left) {
            self.rotate_180();
            assert_eq!(self.top_elem().right_value(), top_left)
        } else if self.bottom_elem().right_value() == Border::reversed(top_left) {
            self.rotate_180();
            self.flip_h();
            assert_eq!(self.top_elem().right_value(), top_left)
        } else {
            // println!("No to-the-left match");
        }
        self.matches_left(other)
    }

    fn matches_left(&self, other: &Column<Tile>) -> bool {
        assert_eq!(self.len(), other.len());
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(slf, oth)| slf.right_value() == oth.left_value())
    }

    fn adapted_to_match_to_the_right_of(&mut self, other: &Column<Tile>) -> bool {
        let top_right = other.top_elem().right_value();
        if self.top_elem().left_value() == top_right {
            // check rest of column
        } else if self.top_elem().right_value() == top_right {
            self.flip_h();
            assert_eq!(self.top_elem().left_value(), top_right)
        } else if self.bottom_elem().right_value() == Border::reversed(top_right) {
            self.rotate_180();
            assert_eq!(self.top_elem().left_value(), top_right);
        } else if self.bottom_elem().left_value() == Border::reversed(top_right) {
            self.rotate_180();
            self.flip_h();
            assert_eq!(self.top_elem().left_value(), top_right)
        } else {
            // println!("No to-the-right match");
        }
        self.matches_right(other)
    }

    fn matches_right(&self, other: &Column<Tile>) -> bool {
        assert_eq!(self.len(), other.len());
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(slf, oth)| slf.left_value() == oth.right_value())
    }

    fn top_value(&self) -> u16 {
        self.top_elem().top_value()
    }

    fn bottom_value(&self) -> u16 {
        self.bottom_elem().bottom_value()
    }

    fn attach_matching_tiles_to_top_and_bottom(&mut self, tiles: &mut Vec<Tile>) {
        while let Some(idx) = index_of_tile_with_border_matching(tiles, self.top_value()) {
            let tile = tiles.remove(idx).adapted_to_match(self.top_value(), BOTTOM);
            // println!("Adding candidate above: {}", tile);
            self.0.push_front(tile);
        }
        while let Some(idx) = index_of_tile_with_border_matching(tiles, self.bottom_value()) {
            let tile = tiles.remove(idx).adapted_to_match(self.bottom_value(), TOP);
            // println!("Adding candidate below: {}", tile);
            self.0.push_back(tile);
        }
    }
}

impl From<Tile> for Column<Tile> {
    fn from(tile: Tile) -> Self {
        Column(VecDeque::from(vec![tile]))
    }
}

fn index_of_tile_with_border_matching(tiles: &mut Vec<Tile>, wanted: u16) -> Option<usize> {
    let indices: Vec<usize> = tiles
        .iter()
        .enumerate()
        .filter(|(_i, t)| t.is_any_border_matching(wanted))
        .map(|(i, _t)| i)
        .collect();
    match indices.len() {
        0 => None,
        1 => Some(indices[0]),
        n => panic!("Found {} matching tiles!", n),
    }
}

impl<T> Square<T>
where
    T: ToString,
{
    fn as_string(&self) -> String {
        let len = self.columns.len();
        let mut lines: Vec<Vec<char>> = vec![vec!['_'; len]; len];
        for (col, column) in self.columns.iter().enumerate() {
            for (row, ch) in column.0.iter().enumerate() {
                lines[row][col] = ch.to_string().parse().unwrap();
            }
        }
        let lines: String = lines
            .iter()
            .map(|v| v.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        lines
    }
}

impl<T> Display for Square<T>
where
    T: ToString,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl<T> Debug for Square<T>
where
    T: ToString,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::*;
    #[cfg(test)]
    use std::collections::HashMap;

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

    #[test]
    fn part1() {
        assert_eq!(
            product_of_corner_tile_ids(&read_file_to_lines("input/day20.txt")),
            60145080587029
        );
    }

    #[test]
    fn example1() {
        assert_eq!(
            product_of_corner_tile_ids(&read_file_to_lines("input/day20/example1.txt")),
            1951 * 3079 * 2971 * 1171
        );
    }

    #[test]
    fn example0() {
        assert_eq!(
            product_of_corner_tile_ids(&read_file_to_lines("input/day20/example0.txt")),
            1951 * 2311 * 2729 * 1427
        );
    }

    #[test]
    fn tile() {
        assert_eq!(
            Tile::from(read_str_to_lines(TILE_2311).as_slice()).borders,
            Tile::new(2311, 210, 89, 231, 498).borders
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
        assert_eq!(actual.borders, Tile::new(2311, 300, 498, 924, 89).borders);
    }

    #[test]
    fn contents_h_flip() {
        let mut actual = Tile::from(read_str_to_lines(TILE_2311).as_slice());
        // println!("{}\n", actual.contents.as_string());
        let flipped: String = actual
            .contents
            .to_string()
            .split('\n')
            .map(|line| line.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        actual.flip_h();
        // println!("{}", actual.contents.as_string());
        assert_eq!(actual.contents.to_string(), flipped);
    }

    #[test]
    fn borders_rotate_cw() {
        let mut actual = Tile::from(read_str_to_lines(TILE_2311).as_slice());
        assert_eq!(actual.borders, Tile::new(2311, 210, 89, 231, 498).borders);
        actual.rotate_cw();
        assert_eq!(actual.borders, Tile::new(2311, 318, 210, 616, 231).borders);
    }

    #[test]
    fn contents_rotate_cw() {
        let original = Tile::from(read_str_to_lines(TILE_2311).as_slice());
        let mut actual = original.clone();
        // println!("{}\n", actual.contents.as_string());
        actual.rotate_cw();
        // println!("{}\n", actual.contents.as_string());
        actual.rotate_cw();
        // println!("{}\n", actual.contents.as_string());

        let rotated_180 = original
            .contents
            .to_string()
            .split('\n')
            .collect::<Vec<&str>>()
            .iter()
            .rev()
            .map(|line| line.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        // println!("{}\n", rotated);
        assert_eq!(actual.contents.as_string(), rotated_180);

        actual.rotate_cw();
        // println!("{}\n", actual.contents.as_string());
        actual.rotate_cw();
        // println!("{}\n", actual.contents.as_string());
        assert_eq!(actual.contents, original.contents);
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
            count_hashes_not_part_of_sea_monsters(&read_file_to_lines("input/day20/example1.txt")),
            273
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            count_hashes_not_part_of_sea_monsters(&read_file_to_lines("input/day20.txt")),
            1901
        );
    }

    #[test]
    fn direct_access_to_map() {
        let mut m: HashMap<usize, usize> = HashMap::new();
        m.insert(1, 2);
        assert_eq!(m[&1], 2);
    }
}
