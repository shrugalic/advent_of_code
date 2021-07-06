use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};

mod tests;

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
        Tile { id, borders }
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

impl Tile {
    fn new(id: usize, top: usize, right: usize, bottom: usize, left: usize) -> Tile {
        Tile {
            id,
            borders: [Border(top), Border(right), Border(bottom), Border(left)],
        }
    }

    fn flip_h(&mut self) {
        self.borders[TOP].reverse();
        self.borders[BOTTOM].reverse();
        self.borders.swap(LEFT, RIGHT);
    }

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

    fn adapt_to_match(&mut self, target_value: usize, at_loc: usize) {
        if !self.rotated_to_have_matching(target_value, at_loc) {
            self.flip_h();
            if !self.rotated_to_have_matching(target_value, at_loc) {
                panic!("Could not adapt to have bottom match {}!", target_value)
            }
        }
    }

    fn rotated_to_have_matching(&mut self, target_value: usize, at_loc: usize) -> bool {
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
    let tiles: Vec<Tile> = input
        .split(|line| line.as_ref().is_empty())
        .map(Tile::from)
        .collect();

    let (tl, tr, bl, br) = arrange(tiles);
    tl * tr * bl * br
}

fn arrange(mut tiles: Vec<Tile>) -> (usize, usize, usize, usize) {
    let side_len = print_stats(&mut tiles);
    let mut cols: VecDeque<Column> = VecDeque::new();

    while cols.len() < side_len {
        let seed = if cols.is_empty() {
            // println!("First column root: {}", tiles[0]);
            tiles.remove(0)
        } else if let Some(id) =
            index_of_tile_with_border_matching(&mut tiles, cols[0].top_tile().left_value())
        {
            let mut seed = tiles.remove(id);
            seed.adapt_to_match(cols[0].top_tile().left_value(), RIGHT);
            seed
        } else if let Some(id) = index_of_tile_with_border_matching(
            &mut tiles,
            cols[cols.len() - 1].top_tile().right_value(),
        ) {
            let mut seed = tiles.remove(id);
            seed.adapt_to_match(cols[cols.len() - 1].top_tile().right_value(), LEFT);
            seed
        } else {
            panic!("Found no column matching either side!");
        };
        // println!("Rest: {:?}", tiles);
        let mut col = Column::from(seed);
        tiles = col.attach_tiles_to_top_and_bottom(tiles, side_len);

        if cols.is_empty() {
            cols.push_front(col);
        } else {
            // check left: as-is, flipped, rotated or rotated & flipped
            if col.adapted_to_match_to_the_left_of(&cols[0]) {
                // println!("Column matched to the left!");
                cols.push_front(col);
            } else if col.adapted_to_match_to_the_right_of(&cols[cols.len() - 1]) {
                // println!("Column matched to the right!");
                cols.push_back(col);
            } else {
                panic!("Column didn't match!");
            }
        }
    }

    // Corners (top-left, top-right, bottom-left, bottom-right)
    (
        cols[0].top_tile().id,
        cols[cols.len() - 1].top_tile().id,
        cols[0].bottom_tile().id,
        cols[cols.len() - 1].bottom_tile().id,
    )
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

struct Column {
    col: VecDeque<Tile>,
}

impl Column {
    fn len(&self) -> usize {
        self.col.len()
    }

    fn adapted_to_match_to_the_left_of(&mut self, other: &Column) -> bool {
        let top_left = other.top_tile().left_value();
        if self.top_tile().right_value() == top_left {
            // check rest of column
        } else if self.top_tile().left_value() == top_left {
            self.flip_h();
            assert_eq!(self.top_tile().right_value(), top_left)
        } else if self.bottom_tile().left_value() == Border::reversed(top_left) {
            self.rotate_180();
            assert_eq!(self.top_tile().right_value(), top_left)
        } else if self.bottom_tile().right_value() == Border::reversed(top_left) {
            self.rotate_180();
            self.flip_h();
            assert_eq!(self.top_tile().right_value(), top_left)
        } else {
            // println!("No to-the-left match");
        }
        self.matches_left(other)
    }

    fn matches_left(&self, other: &Column) -> bool {
        assert_eq!(self.len(), other.len());
        self.col
            .iter()
            .zip(other.col.iter())
            .all(|(slf, oth)| slf.right_value() == oth.left_value())
    }

    fn adapted_to_match_to_the_right_of(&mut self, other: &Column) -> bool {
        let top_right = other.top_tile().right_value();
        if self.top_tile().left_value() == top_right {
            // check rest of column
        } else if self.top_tile().right_value() == top_right {
            self.flip_h();
            assert_eq!(self.top_tile().left_value(), top_right)
        } else if self.bottom_tile().right_value() == Border::reversed(top_right) {
            self.rotate_180();
            assert_eq!(self.top_tile().left_value(), top_right);
        } else if self.bottom_tile().left_value() == Border::reversed(top_right) {
            self.rotate_180();
            self.flip_h();
            assert_eq!(self.top_tile().left_value(), top_right)
        } else {
            // println!("No to-the-right match");
        }
        self.matches_right(other)
    }

    fn matches_right(&self, other: &Column) -> bool {
        assert_eq!(self.len(), other.len());
        self.col
            .iter()
            .zip(other.col.iter())
            .all(|(slf, oth)| slf.left_value() == oth.right_value())
    }

    fn top_tile(&self) -> &Tile {
        &self.col[0]
    }

    fn bottom_tile(&self) -> &Tile {
        &self.col[self.col.len() - 1]
    }

    fn flip_h(&mut self) {
        self.col.iter_mut().for_each(|t| t.flip_h());
    }

    fn rotate_180(&mut self) {
        self.col.make_contiguous();
        self.col.as_mut_slices().0.reverse();
        self.col.iter_mut().for_each(|t| {
            t.rotate_cw();
            t.rotate_cw();
        });
    }

    fn attach_tiles_to_top_and_bottom(
        &mut self,
        mut tiles: Vec<Tile>,
        side_len: usize,
    ) -> Vec<Tile> {
        while let Some(idx) =
            index_of_tile_with_border_matching(&mut tiles, self.top_tile().top_value())
        {
            let mut candidate = tiles.remove(idx);
            candidate.adapt_to_match(self.top_tile().top_value(), BOTTOM);
            // println!("Adding candidate above: {}", candidate);
            self.col.push_front(candidate);
        }
        while let Some(idx) =
            index_of_tile_with_border_matching(&mut tiles, self.bottom_tile().bottom_value())
        {
            let mut candidate = tiles.remove(idx);
            candidate.adapt_to_match(self.bottom_tile().bottom_value(), TOP);
            // println!("Adding candidate below: {}", candidate);
            self.col.push_back(candidate);
        }
        assert_eq!(self.len(), side_len);
        tiles
    }
}

impl From<Tile> for Column {
    fn from(tile: Tile) -> Self {
        Column {
            col: VecDeque::from(vec![tile]),
        }
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
