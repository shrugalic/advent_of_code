use crate::vec_2d::Vec2D;
use std::fmt::{Display, Formatter};

pub(crate) trait TileGrid<T> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn char_at(&self, pos: &Vec2D) -> Option<&T>;
}

pub(crate) trait GridContainsPosition {
    fn contains(&self, pos: &Vec2D) -> bool;
}

impl<T> GridContainsPosition for T
where
    T: TileGrid<char>,
{
    fn contains(&self, pos: &Vec2D) -> bool {
        pos.x >= 0 && pos.x < self.width() as isize && pos.y >= 0 && pos.y < self.height() as isize
    }
}

impl<T> Display for dyn TileGrid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..self.height())
                .map(|y| {
                    (0..self.width())
                        .map(|x| {
                            let pos = Vec2D::new(x, y);
                            self.char_at(&pos)
                                .map(|t| t.to_string())
                                .unwrap_or(" ".to_string())
                        })
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
