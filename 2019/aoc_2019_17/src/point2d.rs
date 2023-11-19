use crate::MovementCommand;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point2D(isize, isize);
impl Default for Point2D {
    fn default() -> Self {
        Point2D(0, 0)
    }
}
impl Point2D {
    pub(crate) fn x(&self) -> isize {
        self.0
    }
    pub(crate) fn y(&self) -> isize {
        self.1
    }
    pub(crate) fn new(x: isize, y: isize) -> Self {
        Point2D(x, y)
    }
    pub(crate) fn offset_by_1_into(self, direction: &MovementCommand) -> Point2D {
        match direction {
            MovementCommand::North => Point2D(self.0, self.1 + 1),
            MovementCommand::South => Point2D(self.0, self.1 - 1),
            MovementCommand::West => Point2D(self.0 - 1, self.1),
            MovementCommand::East => Point2D(self.0 + 1, self.1),
        }
    }
    pub(crate) fn offset_by(self, x: isize, y: isize) -> Point2D {
        Point2D(self.0 + x, self.1 + y)
    }
    pub(crate) fn neighbors(&self) -> Vec<Point2D> {
        vec![
            self.offset_by_1_into(&MovementCommand::North),
            self.offset_by_1_into(&MovementCommand::East),
            self.offset_by_1_into(&MovementCommand::South),
            self.offset_by_1_into(&MovementCommand::West),
        ]
    }
}
