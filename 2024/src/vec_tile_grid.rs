use crate::tile_grid::TileGrid;
use crate::vec_2d::Vec2D;

#[derive(Debug)]
pub(crate) struct VecTileGrid<T> {
    pub(crate) chars: Vec<Vec<T>>,
}

impl<T> TileGrid<T> for VecTileGrid<T> {
    fn width(&self) -> usize {
        self.chars[0].len()
    }
    fn height(&self) -> usize {
        self.chars.len()
    }
    fn char_at(&self, pos: &Vec2D) -> Option<&T> {
        self.chars
            .get(pos.y as usize)
            .and_then(|line| line.get(pos.x as usize))
    }
}

impl<T> From<&str> for VecTileGrid<T>
where
    T: From<char>,
{
    fn from(input: &str) -> Self {
        VecTileGrid {
            chars: input
                .trim()
                .lines()
                .map(|line| line.chars().map(T::from).collect())
                .collect(),
        }
    }
}
