use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Display, Formatter};
use std::mem::swap;

mod tests;

const MIN_CORNER_COUNT: usize = 4;

#[derive(Debug, PartialEq, Clone)]
struct Border {
    value: usize,
}

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
        Border {
            value: usize::from_str_radix(&b, 2).unwrap(),
        }
    }
}

impl From<usize> for Border {
    fn from(value: usize) -> Self {
        Border { value }
    }
}

impl Display for Border {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Border {
    fn complement(&mut self) -> usize {
        self.value ^= 2usize.pow(10) - 1;
        self.value
    }

    fn reverse(&mut self) -> usize {
        self.value = Border::reverse_value(self.value);
        self.value
        // println!("reverse({:010b}) = {:010b}", v, f);
    }

    fn reverse_value(value: usize) -> usize {
        let s = format!("{:010b}", value);
        let f: String = s.chars().rev().collect();
        usize::from_str_radix(&f, 2).unwrap()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Borders {
    top: Border,
    right: Border,
    bottom: Border,
    left: Border,
}

impl Borders {
    fn new(top: usize, right: usize, bottom: usize, left: usize) -> Self {
        Borders {
            top: Border::from(top),
            right: Border::from(right),
            bottom: Border::from(bottom),
            left: Border::from(left),
        }
    }

    fn flip_h(&mut self) {
        self.top.reverse();
        self.bottom.reverse();
        swap(&mut self.left, &mut self.right);
    }

    fn rotate_cw(&mut self) {
        // One clockwise rotation with left as tmp element:
        // left -> top -> right -> bottom -> left
        swap(&mut self.top, &mut self.left);
        // top is old left && left is old top
        swap(&mut self.right, &mut self.left);
        // right is old top &&  left is old right
        swap(&mut self.bottom, &mut self.left);
        // bottom is old right && left is old bottom

        self.top.reverse();
        self.bottom.reverse();
    }

    fn contains_value(&self, value: usize) -> bool {
        self.top() == value
            || self.right() == value
            || self.bottom() == value
            || self.left() == value
    }

    fn could_contain_value(&self, value: usize) -> bool {
        Border::reverse_value(self.top()) == value
            || Border::reverse_value(self.right()) == value
            || Border::reverse_value(self.bottom()) == value
            || Border::reverse_value(self.left()) == value
    }

    fn top(&self) -> usize {
        self.top.value
    }
    fn right(&self) -> usize {
        self.right.value
    }
    fn bottom(&self) -> usize {
        self.bottom.value
    }
    fn left(&self) -> usize {
        self.left.value
    }
}

impl Display for Borders {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.top, self.right, self.bottom, self.left
        )
    }
}

#[derive(PartialEq, Clone)]
struct Tile {
    id: usize,
    borders: Borders,
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
        let top = Border::from(tail[0].as_ref());
        let right = Border::from(right.as_str());
        let bottom = Border::from(tail[tail.len() - 1].as_ref());
        let left = Border::from(left.as_str());
        let borders = Borders {
            top,
            right,
            bottom,
            left,
        };
        Tile { id, borders }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n  Tile {}: {}", self.id, self.borders)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tile {}: {}", self.id, self.borders)
    }
}

impl Tile {
    fn top(&self) -> usize {
        self.borders.top()
    }
    fn right(&self) -> usize {
        self.borders.right()
    }
    fn bottom(&self) -> usize {
        self.borders.bottom()
    }
    fn left(&self) -> usize {
        self.borders.left()
    }
    fn rotate_cw(&mut self) {
        self.borders.rotate_cw();
    }
    fn flip_h(&mut self) {
        self.borders.flip_h();
    }

    fn adapt_to_have_bottom_match(&mut self, target: usize) {
        if !self.rotated_to_have_bottom_match(target) {
            self.flip_h();
            if !self.rotated_to_have_bottom_match(target) {
                panic!("Could not adapt to have bottom match {}!", target)
            }
        }
    }

    fn rotated_to_have_bottom_match(&mut self, target: usize) -> bool {
        let mut i = 0;
        while self.bottom() != target && i < 4 {
            self.rotate_cw();
            i += 1;
        }
        self.bottom() == target
    }

    fn adapt_to_have_top_match(&mut self, target: usize) {
        if !self.rotated_to_have_top_match(target) {
            self.flip_h();
            if !self.rotated_to_have_top_match(target) {
                panic!("Could not adapt to have top match {}!", target)
            }
        }
    }

    fn rotated_to_have_top_match(&mut self, target: usize) -> bool {
        let mut i = 0;
        while self.top() != target && i < 4 {
            self.rotate_cw();
            i += 1;
        }
        self.top() == target
    }
}

fn product_of_corner_tile_ids<T>(input: &[T]) -> usize
where
    T: AsRef<str> + Debug,
{
    let mut tiles: Vec<Tile> = input
        .split(|line| line.as_ref().is_empty())
        .map(Tile::from)
        .collect();

    // let mut counts_by_id = count_ids(&tiles);
    // let mut counts: Vec<(usize, usize)> = counts_by_id.iter().map(|(v, c)| (*v, *c)).collect();
    // counts.sort_by(|a, b| b.1.cmp(&a.1));
    // println!("Counts:\n{:?}", counts);

    // try to rotate and flip tiles to get the required number of matching tiles
    // corners match 2, borders match 3, inside tiles match 4

    // for (id, _count) in counts {
    //     for tile in tiles.iter() {
    //         if !tile.borders.contains_id(id) && tile.borders.could_contain_id(id) {
    //             println!("Tile {} could be made to contain ID", tile)
    //         }
    //     }
    // }

    let (tl, tr, bl, br) = arrange(tiles);
    tl * tr * bl * br

    // println!("Permuting tiles:\n{:?}", tiles);
    // if let Some(result) = permute(&mut tiles, 0) {
    //     println!("\nFound result {:?}", result);
    //     1
    // } else {
    //     0
    // }
}

fn arrange(mut tiles: Vec<Tile>) -> (usize, usize, usize, usize) {
    // Stats
    let side_len = (tiles.len() as f64).sqrt() as usize;
    let sides = 4 * (side_len - 2);
    let insides = (side_len - 2) * (side_len - 2);
    println!(
        "Square of {} * {} tiles has {} corners, {} sides, {} insides",
        side_len, side_len, MIN_CORNER_COUNT, sides, insides
    );

    let mut cols: VecDeque<Column> = VecDeque::new();
    while cols.len() < side_len {
        let root = if cols.is_empty() {
            println!("Random column root: {}", tiles[0]);
            tiles.remove(0)
        } else if let Some(id) = id_of_tile_with_matching_border(&mut tiles, cols[0].top().left()) {
            println!("Left column root: {}", tiles[id]);
            tiles.remove(id)
        } else if let Some(id) =
            id_of_tile_with_matching_border(&mut tiles, cols[cols.len() - 1].top().right())
        {
            println!("Right column root: {}", tiles[id]);
            tiles.remove(id)
        } else {
            panic!("Found no column matching either side!");
        };
        // println!("Rest: {:?}", tiles);
        let mut col = Column::from(root);
        // let mut col: VecDeque<Tile> = VecDeque::from(vec![root]);
        tiles = col.attach_tiles_to_top_and_bottom(tiles, side_len);

        if cols.is_empty() {
            cols.push_front(col);
        } else {
            // check left: as-is, flipped, rotated or rotated & flipped
            if col.adapted_to_match_to_the_left_of(&cols[0]) {
                println!("Column matched to the left!");
                cols.push_front(col);
            } else if col.adapted_to_match_to_the_right_of(&cols[cols.len() - 1]) {
                println!("Column matched to the right!");
                cols.push_back(col);
            } else {
                panic!("Column didn't match!");
            }
        }
        println!("Col is complete");
        if cols.len() > 0 {
            break;
        }
    }

    // (top-left, top-right, bottom-left, bottom-right)
    (
        cols[0].top().id,
        cols[cols.len() - 1].top().id,
        cols[0].bottom().id,
        cols[cols.len() - 1].bottom().id,
    )
}

struct Column {
    col: VecDeque<Tile>,
}

impl Column {
    fn len(&self) -> usize {
        self.col.len()
    }

    fn adapted_to_match_to_the_left_of(&mut self, other: &Column) -> bool {
        let top_left = other.top().left();
        if self.top().right() == top_left {
            // check rest of column
        } else if self.top().left() == top_left {
            self.flip_h();
            assert_eq!(self.top().right(), top_left)
        } else if self.bottom().left() == Border::reverse_value(top_left) {
            self.rotate_180();
            assert_eq!(self.top().right(), top_left)
        } else if self.bottom().right() == Border::reverse_value(top_left) {
            self.rotate_180();
            self.flip_h();
            assert_eq!(self.top().right(), top_left)
        } else {
            println!("No to-the-left match");
        }
        self.matches_left(other)
    }

    fn matches_left(&self, other: &Column) -> bool {
        assert_eq!(self.len(), other.len());
        self.col
            .iter()
            .zip(other.col.iter())
            .all(|(slf, oth)| slf.right() == oth.left())
    }

    fn adapted_to_match_to_the_right_of(&mut self, other: &Column) -> bool {
        let top_right = other.top().right();
        if self.top().left() == top_right {
            // check rest of column
        } else if self.top().right() == top_right {
            self.flip_h();
            assert_eq!(self.top().left(), top_right)
        } else if self.bottom().right() == Border::reverse_value(top_right) {
            self.rotate_180();
            assert_eq!(self.top().left(), top_right);
        } else if self.bottom().left() == Border::reverse_value(top_right) {
            self.rotate_180();
            self.flip_h();
            assert_eq!(self.top().left(), top_right)
        } else {
            println!("No to-the-right match");
        }
        self.matches_right(other)
    }

    fn matches_right(&self, other: &Column) -> bool {
        assert_eq!(self.len(), other.len());
        self.col
            .iter()
            .zip(other.col.iter())
            .all(|(slf, oth)| slf.left() == oth.right())
    }

    fn top(&self) -> &Tile {
        &self.col[0]
    }

    fn bottom(&self) -> &Tile {
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
        while self.col.len() < side_len {
            self.attach_tile_at_top(&mut tiles);
            self.attach_tile_to_bottom(&mut tiles)
        }
        tiles
    }

    fn attach_tile_at_top(&mut self, mut tiles: &mut Vec<Tile>) {
        if let Some(idx) = id_of_tile_with_matching_border(&mut tiles, self.top().top()) {
            let mut candidate = tiles.remove(idx);
            candidate.adapt_to_have_bottom_match(self.top().top());
            println!("Adding candidate above: {}", candidate);
            self.col.push_front(candidate);
        }
    }

    fn attach_tile_to_bottom(&mut self, mut tiles: &mut Vec<Tile>) {
        if let Some(idx) = id_of_tile_with_matching_border(&mut tiles, self.bottom().bottom()) {
            let mut candidate = tiles.remove(idx);
            candidate.adapt_to_have_top_match(self.bottom().bottom());
            println!("Adding candidate below: {}", candidate);
            self.col.push_back(candidate);
        }
    }
}

impl From<Tile> for Column {
    fn from(tile: Tile) -> Self {
        Column {
            col: VecDeque::from(vec![tile]),
        }
    }
}

fn id_of_tile_with_matching_border(tiles: &mut Vec<Tile>, wanted: usize) -> Option<usize> {
    tiles
        .iter()
        .enumerate()
        .filter(|(_i, t)| t.borders.contains_value(wanted) || t.borders.could_contain_value(wanted))
        .map(|(i, _t)| i)
        .next()
}

fn permute(tiles: &mut [Tile], idx: usize) -> Option<Vec<Tile>> {
    if idx == tiles.len() {
        // println!("----");
        // return None;
        // println!("\nFinished 1 permutation:\n{:?}", tiles);
        // panic!("done");
        // Stats
        let side_len = (tiles.len() as f64).sqrt() as usize;
        let min_sides_count = 4 * (side_len - 2);
        let min_insides_count = (side_len - 2) * (side_len - 2);
        let counts_by_value = count_values(tiles);
        if let Some((id, count)) = counts_by_value.iter().find(|(_, &c)| c > 4) {
            println!("There's a config where ID {} appears {} times!", id, count);
        }
        let inside_candidates: Vec<(&usize, &usize)> =
            counts_by_value.iter().filter(|(_, &c)| c == 4).collect();
        let side_candidates: Vec<(&usize, &usize)> =
            counts_by_value.iter().filter(|(_, &c)| c >= 3).collect();
        let corner_candidates: Vec<(&usize, &usize)> =
            counts_by_value.iter().filter(|(_, &c)| c >= 2).collect();
        if inside_candidates.len() >= min_insides_count
            && side_candidates.len() >= min_sides_count
            && corner_candidates.len() >= MIN_CORNER_COUNT
        {
            println!(
                "Found a solution with {} suitable corners, {} suitable sides, {} suitable insides",
                corner_candidates.len(),
                side_candidates.len(),
                inside_candidates.len()
            );
            println!("Inside candidate border-values are:{:?}", inside_candidates);
            let values: Vec<&usize> = inside_candidates.iter().map(|(v, _c)| *v).collect();
            for value in values {
                let tiles: Vec<&Tile> = tiles
                    .iter()
                    .filter(|t| {
                        t.borders.contains_value(*value) || t.borders.could_contain_value(*value)
                    })
                    .collect();
                println!(
                    "{} tiles matching value {} are:\n{:?}",
                    tiles.len(),
                    value,
                    tiles
                );
            }
            Some(tiles.to_vec())
        } else {
            None
        }
    } else {
        permute(tiles, idx + 1).or_else(|| {
            tiles[idx].borders.rotate_cw();
            if idx == 0 {
                println!("{}", tiles[idx]);
            }
            permute(tiles, idx + 1).or_else(|| {
                tiles[idx].borders.rotate_cw();
                if idx == 0 {
                    println!("{}", tiles[idx]);
                }
                permute(tiles, idx + 1).or_else(|| {
                    tiles[idx].borders.rotate_cw();
                    if idx == 0 {
                        println!("{}", tiles[idx]);
                    }
                    permute(tiles, idx + 1).or_else(|| {
                        // Do we need to explore more than 4 permutations?
                        if false {
                            // println!("Exploring more than 4!");
                            tiles[idx].borders.flip_h();
                            permute(tiles, idx + 1).or_else(|| {
                                tiles[idx].borders.rotate_cw();
                                permute(tiles, idx + 1).or_else(|| {
                                    tiles[idx].borders.rotate_cw();
                                    permute(tiles, idx + 1).or_else(|| {
                                        tiles[idx].borders.rotate_cw();
                                        permute(tiles, idx + 1)
                                    })
                                })
                            })
                        } else {
                            None
                        }
                    })
                })
            })
        })
    }
}

fn count_values(tiles: &[Tile]) -> HashMap<usize, usize> {
    let mut counts_by_id: HashMap<usize, usize> = HashMap::new();
    for tile in tiles.iter() {
        *counts_by_id.entry(tile.borders.top.value).or_insert(0) += 1;
        *counts_by_id.entry(tile.borders.right.value).or_insert(0) += 1;
        *counts_by_id.entry(tile.borders.bottom.value).or_insert(0) += 1;
        *counts_by_id.entry(tile.borders.left.value).or_insert(0) += 1;
    }
    counts_by_id
}
