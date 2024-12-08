use crate::vec_2d::Vec2D;
use std::fmt::{Display, Formatter};

pub(crate) trait CharGrid {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn char_at(&self, pos: &Vec2D) -> Option<&char>;
}

pub(crate) trait GridContainsPosition {
    fn contains(&self, pos: &Vec2D) -> bool;
}

impl<T> GridContainsPosition for T
where
    T: CharGrid,
{
    fn contains(&self, pos: &Vec2D) -> bool {
        pos.x >= 0 && pos.x < self.width() as isize && pos.y >= 0 && pos.y < self.height() as isize
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
                            let pos = Vec2D::new(x, y);
                            self.char_at(&pos).unwrap_or(&' ')
                        })
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
