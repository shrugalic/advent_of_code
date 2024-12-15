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
    fn mut_char_at(&mut self, pos: &Vec2D) -> Option<&mut T> {
        self.chars.get_mut(pos)
    }

    fn positions(&self, filter: fn(&T) -> bool) -> Vec<Vec2D> {
        (0..self.height)
            .flat_map(|y| {
                (0..self.width)
                    .map(move |x| Vec2D::new(x, y))
                    .filter(|pos| self.char_at(pos).is_some_and(filter))
            })
            .collect()
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
