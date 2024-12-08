use crate::char_grid::CharGrid;
use crate::vec_2d::Vec2D;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct HashCharGrid {
    width: usize,
    height: usize,
    pub(crate) chars: HashMap<Vec2D, char>,
}

impl CharGrid for HashCharGrid {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn char_at(&self, pos: &Vec2D) -> Option<&char> {
        self.chars.get(pos)
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
                chars.insert(Vec2D::new(x, y), c);
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
