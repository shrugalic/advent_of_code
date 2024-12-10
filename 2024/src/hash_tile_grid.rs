use crate::tile_grid::TileGrid;
use crate::vec_2d::Vec2D;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct HashTileGrid<T> {
    width: usize,
    height: usize,
    pub(crate) chars: HashMap<Vec2D, T>,
}

impl<T> TileGrid<T> for HashTileGrid<T> {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn char_at(&self, pos: &Vec2D) -> Option<&T> {
        self.chars.get(pos)
    }
}

impl<T> From<&str> for HashTileGrid<T>
where
    T: From<char>,
{
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
                chars.insert(Vec2D::new(x, y), T::from(c));
            }
            if height > 0 {
                height += 1;
            }
        }
        HashTileGrid {
            width,
            height,
            chars,
        }
    }
}
