use crate::vec_2d::Vec2D;

pub(crate) trait TileGrid<T> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn tile_at(&self, pos: &Vec2D) -> Option<&T>;
    fn mut_tile_at(&mut self, pos: &Vec2D) -> Option<&mut T>;
    fn positions(&self, filter: fn(&T) -> bool) -> Vec<Vec2D>;
}

pub(crate) trait GridContainsPosition {
    fn contains(&self, pos: &Vec2D) -> bool;
}

impl<G> GridContainsPosition for G
where
    G: TileGrid<char>, // How to implement this for TileGrid<T> instead?
{
    fn contains(&self, pos: &Vec2D) -> bool {
        pos.x >= 0 && pos.x < self.width() as isize && pos.y >= 0 && pos.y < self.height() as isize
    }
}
