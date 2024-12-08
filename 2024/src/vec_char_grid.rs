use crate::char_grid::CharGrid;
use crate::vec_2d::Vec2D;

#[derive(Debug)]
pub(crate) struct VecCharGrid {
    pub(crate) chars: Vec<Vec<char>>,
}

impl CharGrid for VecCharGrid {
    fn width(&self) -> usize {
        self.chars[0].len()
    }
    fn height(&self) -> usize {
        self.chars.len()
    }
    fn char_at(&self, pos: &Vec2D) -> Option<&char> {
        self.chars
            .get(pos.y as usize)
            .and_then(|line| line.get(pos.x as usize))
    }
}

impl From<&str> for VecCharGrid {
    fn from(input: &str) -> Self {
        VecCharGrid {
            chars: input
                .trim()
                .lines()
                .map(|line| line.chars().collect())
                .collect(),
        }
    }
}
