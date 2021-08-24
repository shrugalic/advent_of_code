use intcode::IntCodeComputer;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Point(pub(crate) isize, pub(crate) isize);

#[derive(Debug)]
pub(crate) enum Dir {
    Up,
    Left,
    Down,
    Right,
}
#[derive(Debug)]
pub(crate) struct Robot {
    icc: IntCodeComputer,
    loc: Point,
    dir: Dir,
    painted_panels: HashSet<Point>,
    canvas: HashMap<Point, isize>, // 0 or 1
}
impl Robot {
    pub(crate) fn canvas(&self) -> &HashMap<Point, isize> {
        &self.canvas
    }
    pub(crate) fn painted_panel_count(&self) -> usize {
        self.painted_panels.len()
    }
    pub(crate) fn new(instr: Vec<isize>, initial_color: Option<isize>) -> Self {
        let icc = IntCodeComputer::new(instr);
        let mut robot = Robot {
            icc,
            loc: Point(0, 0),
            dir: Dir::Up,
            painted_panels: HashSet::new(),
            canvas: HashMap::new(),
        };
        robot.init_canvas(initial_color);
        robot
    }
    pub(crate) fn run(&mut self) {
        let mut input = self.color_at_current_loc();
        println!("Color {} at {:?}", input, &self.loc);
        while let Some(color) = self.process(input) {
            // println!("{:?} {}", self.loc, self.canvas.get(&self.loc).unwrap());
            self.paint(color);
            if let Some(direction) = self.process(input) {
                self.turn(direction);
                self.step();
                input = self.color_at_current_loc();
            } else {
                panic!("Did not get direction after color")
            }
        }
    }
    fn color_at_current_loc(&mut self) -> isize {
        *self.canvas.get(&self.loc).unwrap()
    }
    fn process(&mut self, input: isize) -> Option<isize> {
        self.icc.process_int_code_until_first_output(input)
        // self.icc.process_int_code_until_first_output(input)
    }
    fn paint(&mut self, color: isize) {
        if color == 0 || color == 1 {
            self.canvas.insert(self.loc.clone(), color);
            self.painted_panels.insert(self.loc.clone());
        } else {
            panic!("Invalid color {}", color);
        }
    }
    fn turn(&mut self, direction: isize) {
        if direction == 0 {
            // turn left
            self.dir = match self.dir {
                Dir::Up => Dir::Left,
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Right,
                Dir::Right => Dir::Up,
            }
        } else if direction == 1 {
            self.dir = match self.dir {
                Dir::Up => Dir::Right,
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
            }
        } else {
            panic!("Invalid direction {}", direction);
        }
    }
    fn step(&mut self) {
        self.loc = match self.dir {
            Dir::Up => Point(self.loc.0, self.loc.1 + 1),
            Dir::Left => Point(self.loc.0 - 1, self.loc.1),
            Dir::Down => Point(self.loc.0, self.loc.1 - 1),
            Dir::Right => Point(self.loc.0 + 1, self.loc.1),
        };
        self.init_canvas(None);
    }
    fn init_canvas(&mut self, color: Option<isize>) {
        if !self.canvas.contains_key(&self.loc) {
            self.canvas.insert(self.loc.clone(), color.unwrap_or(0)); // fallback to black
        }
    }
}

// day 13

#[derive(Debug, PartialEq)]
pub(crate) enum Tile {
    Empty,  // No game object appears in this tile.
    Wall,   // Walls are indestructible barriers.
    Block,  // Blocks can be broken by the ball.
    Paddle, // The paddle is indestructible.
    Ball,   // The ball moves diagonally and bounces off objects.
}
impl From<isize> for Tile {
    fn from(i: isize) -> Self {
        match i {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            i => panic!("Unknown tile {}", i),
        }
    }
}

#[derive(Debug)]
pub(crate) struct ArcadeCabinet {
    icc: intcode::IntCodeComputer,
}
impl ArcadeCabinet {
    pub(crate) fn new(instr: Vec<isize>) -> Self {
        let icc = intcode::IntCodeComputer::new(instr);
        ArcadeCabinet { icc }
    }
    pub(crate) fn run(&mut self) -> Vec<(Point, Tile)> {
        self.run_with(false)
    }
    pub(crate) fn play(&mut self) -> Vec<(Point, Tile)> {
        self.run_with(true)
    }
    fn run_with(&mut self, play: bool) -> Vec<(Point, Tile)> {
        let mut joystick = JoyStick::Neutral.to_input();
        if play {
            self.icc.set(0, 2);
        }
        let mut outputs: Vec<(Point, Tile)> = vec![];
        let mut ball: Option<Point> = None;
        let mut paddle: Option<Point> = None;
        while let Some(first) = self.icc.process_int_code_until_first_output(joystick) {
            if let Some(second) = self.icc.process_int_code_until_first_output(joystick) {
                if let Some(third) = self.icc.process_int_code_until_first_output(joystick) {
                    let pos = Point(first, second);
                    if pos == Point(-1, 0) {
                        // println!("Score = {}", third);
                    } else {
                        let tile = Tile::from(third);
                        if play {
                            match tile {
                                Tile::Ball => {
                                    ball = Some(pos.clone());
                                    joystick = ArcadeCabinet::calc_input(&ball, &paddle).to_input();
                                    // println!(
                                    //     "Ball: ball {:?}, paddle {:?}, joystick {:?}",
                                    //     pos, paddle, joystick
                                    // );
                                }
                                Tile::Paddle => {
                                    paddle = Some(pos.clone());
                                    joystick = ArcadeCabinet::calc_input(&ball, &paddle).to_input();
                                    // println!(
                                    //     "Paddle: ball {:?}, paddle {:?}, joystick {:?}",
                                    //     ball, pos, joystick
                                    // );
                                }
                                _ => (),
                            }
                        }
                        outputs.push((pos, tile));
                    }
                }
            }
        }
        outputs
    }
    /// Move the paddle towards the ball position
    fn calc_input(ball: &Option<Point>, paddle: &Option<Point>) -> JoyStick {
        if let (Some(ball_pos), Some(paddle_pos)) = (ball, paddle) {
            JoyStick::from(ball_pos.0 - paddle_pos.0)
        } else {
            JoyStick::Neutral
        }
    }
}
enum JoyStick {
    Neutral,
    Left,
    Right,
}
impl From<isize> for JoyStick {
    fn from(diff: isize) -> Self {
        match diff.signum() {
            0 => JoyStick::Neutral,
            -1 => JoyStick::Left,
            1 => JoyStick::Right,
            _ => unreachable!(),
        }
    }
}
impl JoyStick {
    fn to_input(&self) -> isize {
        match self {
            JoyStick::Neutral => 0,
            JoyStick::Left => -1,
            JoyStick::Right => 1,
        }
    }
}

pub(crate) fn day_13_puzzle_input() -> Vec<isize> {
    vec![
        1, 380, 379, 385, 1008, 2267, 610415, 381, 1005, 381, 12, 99, 109, 2268, 1101, 0, 0, 383,
        1101, 0, 0, 382, 20102, 1, 382, 1, 20101, 0, 383, 2, 21101, 37, 0, 0, 1106, 0, 578, 4, 382,
        4, 383, 204, 1, 1001, 382, 1, 382, 1007, 382, 37, 381, 1005, 381, 22, 1001, 383, 1, 383,
        1007, 383, 22, 381, 1005, 381, 18, 1006, 385, 69, 99, 104, -1, 104, 0, 4, 386, 3, 384,
        1007, 384, 0, 381, 1005, 381, 94, 107, 0, 384, 381, 1005, 381, 108, 1105, 1, 161, 107, 1,
        392, 381, 1006, 381, 161, 1101, -1, 0, 384, 1106, 0, 119, 1007, 392, 35, 381, 1006, 381,
        161, 1101, 0, 1, 384, 21001, 392, 0, 1, 21102, 1, 20, 2, 21101, 0, 0, 3, 21102, 138, 1, 0,
        1105, 1, 549, 1, 392, 384, 392, 21002, 392, 1, 1, 21101, 0, 20, 2, 21101, 3, 0, 3, 21101,
        161, 0, 0, 1106, 0, 549, 1101, 0, 0, 384, 20001, 388, 390, 1, 20101, 0, 389, 2, 21102, 1,
        180, 0, 1105, 1, 578, 1206, 1, 213, 1208, 1, 2, 381, 1006, 381, 205, 20001, 388, 390, 1,
        21002, 389, 1, 2, 21101, 205, 0, 0, 1106, 0, 393, 1002, 390, -1, 390, 1102, 1, 1, 384,
        21002, 388, 1, 1, 20001, 389, 391, 2, 21102, 228, 1, 0, 1106, 0, 578, 1206, 1, 261, 1208,
        1, 2, 381, 1006, 381, 253, 20101, 0, 388, 1, 20001, 389, 391, 2, 21101, 253, 0, 0, 1105, 1,
        393, 1002, 391, -1, 391, 1101, 1, 0, 384, 1005, 384, 161, 20001, 388, 390, 1, 20001, 389,
        391, 2, 21102, 1, 279, 0, 1105, 1, 578, 1206, 1, 316, 1208, 1, 2, 381, 1006, 381, 304,
        20001, 388, 390, 1, 20001, 389, 391, 2, 21101, 304, 0, 0, 1105, 1, 393, 1002, 390, -1, 390,
        1002, 391, -1, 391, 1101, 1, 0, 384, 1005, 384, 161, 20102, 1, 388, 1, 20101, 0, 389, 2,
        21101, 0, 0, 3, 21102, 338, 1, 0, 1105, 1, 549, 1, 388, 390, 388, 1, 389, 391, 389, 20102,
        1, 388, 1, 20101, 0, 389, 2, 21101, 0, 4, 3, 21102, 1, 365, 0, 1106, 0, 549, 1007, 389, 21,
        381, 1005, 381, 75, 104, -1, 104, 0, 104, 0, 99, 0, 1, 0, 0, 0, 0, 0, 0, 265, 16, 17, 1, 1,
        18, 109, 3, 21202, -2, 1, 1, 21201, -1, 0, 2, 21102, 1, 0, 3, 21102, 414, 1, 0, 1105, 1,
        549, 22101, 0, -2, 1, 21202, -1, 1, 2, 21101, 429, 0, 0, 1105, 1, 601, 2102, 1, 1, 435, 1,
        386, 0, 386, 104, -1, 104, 0, 4, 386, 1001, 387, -1, 387, 1005, 387, 451, 99, 109, -3,
        2106, 0, 0, 109, 8, 22202, -7, -6, -3, 22201, -3, -5, -3, 21202, -4, 64, -2, 2207, -3, -2,
        381, 1005, 381, 492, 21202, -2, -1, -1, 22201, -3, -1, -3, 2207, -3, -2, 381, 1006, 381,
        481, 21202, -4, 8, -2, 2207, -3, -2, 381, 1005, 381, 518, 21202, -2, -1, -1, 22201, -3, -1,
        -3, 2207, -3, -2, 381, 1006, 381, 507, 2207, -3, -4, 381, 1005, 381, 540, 21202, -4, -1,
        -1, 22201, -3, -1, -3, 2207, -3, -4, 381, 1006, 381, 529, 22101, 0, -3, -7, 109, -8, 2106,
        0, 0, 109, 4, 1202, -2, 37, 566, 201, -3, 566, 566, 101, 639, 566, 566, 2102, 1, -1, 0,
        204, -3, 204, -2, 204, -1, 109, -4, 2105, 1, 0, 109, 3, 1202, -1, 37, 593, 201, -2, 593,
        593, 101, 639, 593, 593, 21001, 0, 0, -2, 109, -3, 2105, 1, 0, 109, 3, 22102, 22, -2, 1,
        22201, 1, -1, 1, 21102, 1, 409, 2, 21102, 1, 463, 3, 21102, 1, 814, 4, 21102, 1, 630, 0,
        1106, 0, 456, 21201, 1, 1453, -2, 109, -3, 2105, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1, 1, 0, 0, 2, 2, 0, 2, 2, 0, 2, 2, 2, 0, 0, 0, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 0, 2, 2,
        2, 2, 0, 2, 0, 0, 0, 0, 1, 1, 0, 0, 2, 0, 0, 2, 2, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 2, 2,
        0, 2, 2, 2, 2, 0, 0, 2, 2, 0, 0, 2, 0, 2, 0, 1, 1, 0, 0, 2, 2, 2, 0, 0, 0, 2, 2, 2, 2, 0,
        0, 2, 2, 0, 2, 0, 2, 0, 2, 2, 0, 2, 2, 2, 0, 2, 2, 0, 2, 2, 2, 0, 1, 1, 0, 2, 0, 0, 2, 2,
        2, 0, 2, 2, 0, 2, 0, 2, 2, 2, 2, 0, 0, 2, 2, 2, 0, 2, 2, 0, 0, 0, 0, 2, 2, 0, 0, 2, 0, 1,
        1, 0, 2, 2, 2, 0, 0, 0, 2, 2, 2, 0, 2, 2, 2, 2, 2, 0, 0, 0, 2, 2, 0, 2, 2, 2, 0, 2, 0, 0,
        0, 0, 0, 2, 2, 0, 1, 1, 0, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 0, 0, 0, 0, 2, 0, 0, 2, 0, 2,
        2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 0, 1, 1, 0, 0, 0, 2, 0, 0, 2, 2, 2, 0, 2, 0, 0, 0, 0,
        2, 0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 2, 0, 0, 2, 0, 2, 2, 2, 0, 1, 1, 0, 2, 0, 0, 2, 2, 0, 0,
        0, 2, 0, 0, 0, 2, 2, 2, 2, 0, 2, 2, 0, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 1, 1, 0,
        2, 0, 0, 0, 2, 0, 2, 2, 2, 2, 2, 0, 0, 2, 0, 2, 2, 0, 0, 2, 2, 0, 2, 2, 0, 2, 2, 0, 2, 0,
        0, 2, 2, 0, 1, 1, 0, 2, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 0, 2, 2, 0, 2, 0, 0, 0, 2, 2, 0, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 1, 1, 0, 0, 0, 0, 2, 0, 2, 2, 2, 2, 0, 2, 2, 0, 2, 2, 2,
        0, 2, 2, 0, 2, 2, 2, 0, 2, 0, 0, 2, 2, 0, 2, 0, 0, 0, 1, 1, 0, 0, 2, 2, 2, 0, 2, 0, 2, 0,
        2, 2, 2, 0, 0, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 2, 0, 2, 2, 0, 2, 0, 2, 2, 0, 1, 1, 0, 0, 2,
        2, 2, 2, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 2, 0, 2, 2, 2, 0, 2,
        0, 0, 1, 1, 0, 2, 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 2, 2, 0, 0, 2, 2,
        2, 2, 2, 0, 0, 0, 2, 2, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 72, 10, 67, 45, 58, 25,
        55, 73, 97, 49, 19, 51, 58, 95, 30, 82, 74, 9, 98, 96, 38, 64, 30, 45, 14, 73, 42, 5, 3,
        61, 68, 23, 18, 14, 9, 16, 21, 7, 77, 39, 38, 16, 82, 17, 58, 87, 90, 64, 52, 1, 96, 67,
        66, 16, 65, 15, 22, 41, 69, 90, 93, 92, 96, 45, 68, 17, 63, 51, 15, 61, 51, 93, 65, 55, 42,
        76, 48, 52, 31, 98, 6, 88, 69, 65, 65, 30, 51, 88, 4, 13, 36, 90, 80, 23, 31, 42, 63, 86,
        52, 15, 79, 78, 59, 77, 57, 71, 84, 81, 73, 56, 1, 5, 7, 86, 75, 31, 63, 76, 21, 73, 16,
        41, 86, 15, 78, 85, 2, 79, 63, 54, 79, 65, 87, 13, 86, 96, 81, 69, 27, 76, 8, 48, 5, 79,
        10, 74, 76, 86, 95, 55, 72, 52, 23, 41, 50, 46, 68, 29, 86, 61, 96, 29, 34, 40, 86, 86, 1,
        20, 90, 35, 69, 64, 50, 51, 75, 65, 93, 19, 5, 15, 96, 3, 88, 8, 43, 66, 88, 72, 84, 69,
        42, 4, 95, 51, 80, 81, 27, 75, 92, 22, 45, 54, 63, 51, 82, 91, 13, 25, 54, 41, 84, 84, 29,
        98, 50, 91, 11, 40, 69, 13, 47, 42, 72, 46, 87, 31, 27, 98, 65, 94, 26, 51, 79, 39, 29, 38,
        42, 46, 25, 36, 26, 66, 12, 93, 58, 1, 61, 41, 37, 57, 60, 60, 9, 70, 63, 26, 56, 1, 27, 5,
        11, 93, 17, 48, 95, 19, 79, 16, 14, 16, 29, 79, 56, 16, 26, 37, 50, 10, 38, 53, 4, 10, 3,
        57, 20, 59, 16, 51, 88, 66, 74, 91, 56, 42, 84, 30, 36, 31, 36, 58, 68, 66, 91, 36, 71, 30,
        39, 96, 50, 84, 76, 95, 14, 89, 75, 59, 77, 66, 36, 88, 62, 60, 3, 45, 13, 39, 48, 33, 59,
        21, 19, 35, 90, 81, 66, 52, 75, 34, 70, 55, 56, 47, 22, 20, 87, 73, 73, 76, 73, 8, 96, 55,
        46, 5, 1, 64, 27, 8, 37, 87, 50, 8, 79, 74, 63, 26, 43, 44, 2, 85, 91, 28, 13, 16, 15, 55,
        87, 94, 28, 86, 66, 29, 34, 46, 18, 41, 37, 94, 63, 31, 78, 48, 17, 4, 25, 62, 15, 10, 18,
        19, 97, 50, 78, 5, 79, 5, 70, 64, 86, 61, 58, 59, 61, 5, 71, 68, 14, 24, 17, 56, 85, 52,
        64, 92, 45, 90, 94, 55, 47, 5, 56, 59, 20, 15, 41, 36, 58, 55, 25, 47, 45, 69, 58, 36, 44,
        80, 94, 52, 84, 17, 27, 20, 44, 51, 93, 10, 56, 77, 45, 29, 93, 63, 96, 95, 47, 31, 63, 69,
        64, 74, 53, 34, 36, 20, 14, 40, 30, 61, 86, 15, 3, 94, 61, 43, 75, 59, 64, 41, 34, 98, 32,
        65, 73, 18, 30, 46, 66, 38, 68, 25, 96, 16, 37, 54, 38, 44, 26, 52, 1, 2, 21, 93, 37, 26,
        4, 45, 69, 82, 59, 34, 55, 34, 77, 88, 46, 70, 32, 56, 82, 10, 20, 31, 40, 20, 55, 3, 3,
        93, 95, 65, 56, 61, 68, 41, 35, 62, 20, 58, 55, 42, 41, 40, 33, 51, 6, 52, 84, 27, 62, 81,
        32, 35, 87, 97, 79, 7, 97, 77, 40, 48, 74, 4, 6, 36, 58, 59, 25, 6, 5, 84, 7, 44, 51, 88,
        37, 9, 30, 29, 26, 91, 41, 72, 39, 24, 68, 58, 49, 80, 49, 43, 98, 43, 92, 9, 49, 64, 10,
        96, 50, 86, 56, 2, 72, 58, 80, 57, 77, 61, 74, 14, 42, 50, 55, 40, 21, 77, 20, 19, 16, 80,
        84, 92, 27, 32, 37, 80, 59, 69, 13, 11, 19, 6, 94, 54, 88, 51, 69, 41, 54, 68, 36, 82, 68,
        19, 77, 85, 37, 5, 58, 61, 72, 5, 67, 17, 35, 29, 18, 71, 46, 5, 29, 8, 93, 97, 36, 37, 25,
        93, 27, 33, 93, 79, 10, 84, 75, 6, 91, 98, 34, 32, 37, 70, 18, 84, 52, 32, 11, 88, 44, 69,
        58, 92, 52, 68, 77, 39, 90, 9, 58, 74, 1, 53, 56, 64, 75, 46, 59, 39, 52, 32, 41, 62, 81,
        75, 7, 93, 29, 89, 51, 34, 31, 93, 70, 94, 30, 98, 68, 3, 60, 2, 2, 49, 31, 15, 65, 11, 78,
        70, 2, 50, 29, 9, 9, 85, 65, 52, 28, 95, 55, 77, 98, 29, 65, 56, 51, 32, 44, 42, 82, 14,
        29, 22, 5, 29, 65, 86, 84, 88, 58, 63, 10, 13, 13, 51, 97, 17, 57, 19, 39, 83, 72, 93, 15,
        54, 31, 83, 3, 43, 21, 83, 74, 2, 86, 47, 25, 89, 20, 11, 68, 80, 29, 21, 58, 69, 610415,
    ]
}

pub(crate) fn stats(tiles: &[(Point, Tile)]) -> (usize, usize, usize, usize, usize) {
    let blocks = tiles.iter().filter(|(_, t)| t == &Tile::Block).count();
    let balls = tiles.iter().filter(|(_, t)| t == &Tile::Ball).count();
    let paddles = tiles.iter().filter(|(_, t)| t == &Tile::Paddle).count();
    let walls = tiles.iter().filter(|(_, t)| t == &Tile::Wall).count();
    let empty = tiles.iter().filter(|(_, t)| t == &Tile::Empty).count();
    // println!(
    //     "Tiles: {} blocks, {} balls, {} paddles, {} walls, {} empty, {} total",
    //     blocks,
    //     balls,
    //     paddles,
    //     walls,
    //     empty,
    //     tiles.len(),
    // );
    (blocks, balls, paddles, walls, empty)
}

mod tests {
    use super::{day_13_puzzle_input, stats, ArcadeCabinet, Point, Tile};

    #[test]
    fn day_13_part1() {
        let mut arcade = ArcadeCabinet::new(day_13_puzzle_input());
        let tiles = arcade.run();
        let (block_count, _, _, _, _) = stats(&tiles);

        assert_eq!(block_count, 265)
    }
    #[test]
    fn day_13_part2() {
        let mut arcade = ArcadeCabinet::new(day_13_puzzle_input());
        let tiles: Vec<(Point, Tile)> = arcade.play();
        assert_eq!(tiles.len(), 26947); // Score 13331
    }
}
