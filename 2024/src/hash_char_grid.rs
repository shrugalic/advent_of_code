use crate::pos_2d::Position;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) struct HashCharGrid {
    width: usize,
    height: usize,
    pub(crate) chars: HashMap<Position, char>,
}

pub(crate) trait CharGrid {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn char_at(&self, pos: &Position) -> Option<&char>;
}

impl CharGrid for HashCharGrid {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn char_at(&self, pos: &Position) -> Option<&char> {
        self.chars.get(pos)
    }
}

pub(crate) trait GridContainsPosition {
    fn contains(&self, pos: &Position) -> bool;
}

impl<T> GridContainsPosition for T
where
    T: CharGrid,
{
    fn contains(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.width() as isize && pos.y >= 0 && pos.y < self.height() as isize
    }
}

impl From<&str> for HashCharGrid {
    fn from(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut chars = HashMap::new();
        for (y, line) in input.trim().lines().enumerate() {
            if width == 0 {
                width = line.len();
            }
            height = height.max(y);
            for (x, c) in line.chars().enumerate() {
                chars.insert(Position::new(x, y), c);
            }
            if height > 0 {
                height += 1;
            }
        }
        HashCharGrid {
            width,
            height,
            chars,
        }
    }
}

impl Display for dyn CharGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..self.height())
                .map(|y| {
                    (0..self.width())
                        .map(|x| {
                            let pos = Position::new(x, y);
                            self.char_at(&pos).unwrap_or(&' ')
                        })
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
