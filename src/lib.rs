use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};

mod tests;

trait Flipable {
    fn flip_h(&mut self);
}

trait Rotatable {
    fn rotate_cw(&mut self);
}

#[derive(PartialEq, Clone)]
struct Border(usize);

impl From<&str> for Border {
    fn from(s: &str) -> Self {
        let b = s
            .chars()
            .map(|c| match c {
                '#' => '1',
                '.' => '0',
                _ => panic!("Invalid char '{}'", c),
            })
            .collect::<String>();
        Border(usize::from_str_radix(&b, 2).unwrap())
    }
}

impl From<usize> for Border {
    fn from(value: usize) -> Self {
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

    fn reversed(value: usize) -> usize {
        let s = format!("{:010b}", value);
        let f: String = s.chars().rev().collect();
        usize::from_str_radix(&f, 2).unwrap()
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

impl Flipable for char {
    fn flip_h(&mut self) {
        // nop
    }
}

impl Rotatable for char {
    fn rotate_cw(&mut self) {
        // nop
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

impl Flipable for Tile {
    fn flip_h(&mut self) {
        self.borders[TOP].reverse();
        self.borders[BOTTOM].reverse();
        self.borders.swap(LEFT, RIGHT);

        self.contents.columns.make_contiguous();
        self.contents.columns.as_mut_slices().0.reverse();
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

        // TODO self.contents
    }
}

impl Tile {
    fn new(id: usize, top: usize, right: usize, bottom: usize, left: usize) -> Tile {
        Tile {
            id,
            borders: [Border(top), Border(right), Border(bottom), Border(left)],
            contents: Square::new(),
        }
    }

    fn is_any_border_matching(&self, value: usize) -> bool {
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

    fn top_value(&self) -> usize {
        self.borders[TOP].0
    }
    fn right_value(&self) -> usize {
        self.borders[RIGHT].0
    }
    fn bottom_value(&self) -> usize {
        self.borders[BOTTOM].0
    }
    fn left_value(&self) -> usize {
        self.borders[LEFT].0
    }

    fn adapted_to_match(mut self, target_value: usize, at_loc: usize) -> Self {
        if !self.rotated_to_have_matching(target_value, at_loc) {
            self.flip_h();
            if !self.rotated_to_have_matching(target_value, at_loc) {
                panic!("Could not adapt to have bottom match {}!", target_value)
            }
        }
        self
    }

    fn rotated_to_have_matching(&mut self, target_value: usize, at_loc: usize) -> bool {
        let mut i = 0;
        while self.borders[at_loc].0 != target_value && i < 4 {
            self.rotate_cw();
            i += 1;
        }
        self.borders[at_loc].0 == target_value
    }

    fn contents_to_string(&self) -> String {
        let len = self.contents.len();
        let mut lines: Vec<Vec<char>> = vec![vec!['_'; len]; len];
        for (col, column) in self.contents.columns.iter().enumerate() {
            for (row, ch) in column.0.iter().enumerate() {
                lines[row][col] = *ch;
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

fn product_of_corner_tile_ids<T>(input: &[T]) -> usize
where
    T: AsRef<str> + Debug,
{
    let square = form_square(input);
    square.top_left_corner().id
        * square.top_right_corner().id
        * square.bottom_left_corner().id
        * square.bottom_right_corner().id
}

fn count_hashes_not_part_of_sea_monsters<T>(input: &[T]) -> usize
where
    T: AsRef<str> + Debug,
{
    let square = form_square(input);

    0
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
    let sides = 4 * (side_len - 2);
    let insides = (side_len - 2) * (side_len - 2);
    println!(
        "Square of {} * {} tiles has 4 corners, {} sides, {} insides",
        side_len, side_len, sides, insides
    );
    side_len
}

#[derive(PartialEq, Clone)]
struct Square<T> {
    columns: VecDeque<Column<T>>,
}

impl<T> Square<T>
where
    T: Flipable + Rotatable,
{
    fn new() -> Self {
        Self {
            columns: VecDeque::new(),
        }
    }
    fn len(&self) -> usize {
        self.columns.len()
    }
    fn left_column(&self) -> &Column<T> {
        &self.columns[0]
    }
    fn right_column(&self) -> &Column<T> {
        &self.columns[self.columns.len() - 1]
    }
    fn add_left(&mut self, column: Column<T>) {
        self.columns.push_front(column)
    }
    fn add_right(&mut self, column: Column<T>) {
        self.columns.push_back(column)
    }
    fn is_empty(&self) -> bool {
        self.columns.is_empty()
    }
    fn top_left_corner(&self) -> &T {
        self.left_column().top_elem()
    }
    fn top_right_corner(&self) -> &T {
        self.right_column().top_elem()
    }
    fn bottom_left_corner(&self) -> &T {
        self.left_column().bottom_elem()
    }
    fn bottom_right_corner(&self) -> &T {
        self.right_column().bottom_elem()
    }
}

impl Square<Tile> {
    fn attach(&mut self, mut column: Column<Tile>) {
        if self.is_empty() {
            self.add_left(column);
        } else {
            // check left: as-is, flipped, rotated or rotated & flipped
            if column.adapted_to_match_to_the_left_of(&self.left_column()) {
                // println!("Column matched to the left!");
                self.add_left(column);
            } else if column.adapted_to_match_to_the_right_of(&self.right_column()) {
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
    T: Flipable + Rotatable,
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
        T: Flipable,
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

    fn top_value(&self) -> usize {
        self.top_elem().top_value()
    }

    fn bottom_value(&self) -> usize {
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

fn index_of_tile_with_border_matching(tiles: &mut Vec<Tile>, wanted: usize) -> Option<usize> {
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
