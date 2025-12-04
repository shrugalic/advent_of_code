use crate::tile_grid::TileGrid;
use crate::vec_2d::Vec2D;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) struct VecTileGrid<T> {
    pub(crate) lines: Vec<Vec<T>>,
}

impl<T> TileGrid<T> for VecTileGrid<T> {
    fn width(&self) -> usize {
        self.lines[0].len()
    }
    fn height(&self) -> usize {
        self.lines.len()
    }
    fn tile_at(&self, pos: &Vec2D) -> Option<&T> {
        self.lines
            .get(pos.y as usize)
            .and_then(|line| line.get(pos.x as usize))
    }
    fn mut_tile_at(&mut self, pos: &Vec2D) -> Option<&mut T> {
        self.lines
            .get_mut(pos.y as usize)
            .and_then(|line| line.get_mut(pos.x as usize))
    }

    fn positions(&self, filter: fn(&T) -> bool) -> Vec<Vec2D> {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_x, c)| filter(c))
                    .map(move |(x, _)| Vec2D::new(x, y))
            })
            .collect()
    }
}

impl<T> From<&str> for VecTileGrid<T>
where
    T: From<char>,
{
    fn from(input: &str) -> Self {
        VecTileGrid {
            lines: input
                .trim()
                .lines()
                .map(|line| line.chars().map(T::from).collect())
                .collect(),
        }
    }
}

impl<T> Display for VecTileGrid<T>
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
                            self.tile_at(&pos)
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
