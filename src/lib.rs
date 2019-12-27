use std::collections::{HashMap, HashSet};
use Mode::*;
use Op::*;

const TEST_MODE_INPUT: isize = 1;
const PRINT_INT_CODE_COMPUTER_OUTPUT: bool = false;

#[derive(Debug)]
struct IntCodeComputer {
    instr: Vec<isize>, // program
    ptr: usize,        // instruction pointer
    base: isize,       // relative base
}
impl IntCodeComputer {
    fn new(instr: &mut Vec<isize>) -> Self {
        IntCodeComputer {
            instr: instr.to_vec(),
            ptr: 0,
            base: 0,
        }
    }
    fn next_op_as_5_digit_string_padded_with_leading_zeroes(&self) -> String {
        let s = self.instr[self.ptr].to_string();
        "0".repeat(5 - s.len()) + s.as_ref()
    }
    fn get(&mut self, idx: usize) -> isize {
        self.grow_if_needed(idx);
        self.instr[idx]
    }
    fn set(&mut self, idx: usize, val: isize) {
        self.grow_if_needed(idx);
        self.instr[idx] = val;
    }
    fn grow_if_needed(&mut self, idx: usize) {
        if idx >= self.instr.len() {
            let diff = 1 + idx - self.instr.len();
            if diff > 10000 {
                if PRINT_INT_CODE_COMPUTER_OUTPUT {
                    println!(
                        "old size = {}, idx = {}, huge diff = {}",
                        self.instr.len(),
                        idx,
                        diff
                    );
                }
            }
            self.instr.extend(vec![0; diff]);
        }
    }
    fn get_value(&mut self, offset: usize, mode: &Mode) -> isize {
        let val = self.get(self.ptr + offset);
        match mode {
            Immediate => val,
            Position => self.get(val as usize),
            Relative => self.get((self.base + val) as usize),
        }
    }
    fn set_result(&mut self, offset: usize, mode: &Mode, res: isize) {
        let val = self.get(self.ptr + offset);
        if PRINT_INT_CODE_COMPUTER_OUTPUT {
            println!(" [{}] = {}", val, res);
        }
        match mode {
            Immediate => panic!("Output parameter in immediate mode!"),
            Position => self.set(val as usize, res),
            Relative => self.set((self.base + val) as usize, res),
        }
    }

    fn process_int_code_with_default_input(&mut self) -> Option<isize> {
        self.process_int_code_with_input(TEST_MODE_INPUT)
    }

    fn process_int_code_with_input(&mut self, input: isize) -> Option<isize> {
        let mut output = None;
        while self.ptr < self.instr.len() {
            let s = self.next_op_as_5_digit_string_padded_with_leading_zeroes();
            let code = to_num(&s[(s.len() - 2)..s.len()]);
            let op = Op::from_code(code);
            let modes = Mode::extract_modes(&s);
            let pre = format!("{:?}: {:?}", s, op);
            match op {
                Add | Multiply | LessThan | Equals => {
                    let p1 = self.get_value(1, &modes[0]);
                    let p2 = self.get_value(2, &modes[1]);
                    let res = match op {
                        Add => p1 + p2,
                        Multiply => p1 * p2,
                        LessThan => eval(p1 < p2),
                        Equals => eval(p1 == p2),
                        _ => unreachable!(),
                    };
                    //                    print!("{}({}, {})", pre, p1, p2);
                    self.set_result(3, &modes[2], res);
                }
                Input => {
                    if PRINT_INT_CODE_COMPUTER_OUTPUT {
                        print!("{}", pre);
                    }
                    self.set_result(1, &modes[0], input);
                }
                Output => {
                    let value = self.get_value(1, &modes[0]);
                    if PRINT_INT_CODE_COMPUTER_OUTPUT {
                        println!("{} = {}", pre, value);
                    }
                    output = Some(value);
                }
                ShiftRelativeBase => {
                    let shift = self.get_value(1, &modes[0]);
                    let _old_base = self.base;
                    self.base = self.base + shift;
                    //                    println!("{} by {} from {} to {}", pre, shift, old_base, self.base);
                }
                JumpIfTrue | JumpIfFalse => {
                    let p1 = self.get_value(1, &modes[0]);
                    let p2 = self.get_value(2, &modes[1]);
                    if op == JumpIfTrue && p1 != 0 || op == JumpIfFalse && p1 == 0 {
                        self.ptr = p2 as usize;
                        //                        println!("{} ({}) == true -> jump to {}", pre, p1, p2);
                        continue; // jump, rather than increasing idx below
                    }
                    //                    println!("{} ({}) == false -> NO jump (to {})", pre, p1, p2);
                }
                Stop => {
                    if PRINT_INT_CODE_COMPUTER_OUTPUT {
                        println!("{}", pre);
                    }
                    break;
                }
            }
            self.ptr += op.value_count();
            if output.is_some() {
                break;
            }
        }
        output
    }
}
fn to_num(s: &str) -> isize {
    s.parse::<isize>().unwrap()
}
fn eval(b: bool) -> isize {
    if b {
        1
    } else {
        0
    }
}

#[derive(PartialEq, Debug)]
enum Op {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    ShiftRelativeBase,
    Stop,
}

impl Op {
    fn from_code(code: isize) -> Op {
        match code {
            1 => Add,
            2 => Multiply,
            3 => Input,
            4 => Output,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => LessThan,
            8 => Equals,
            9 => ShiftRelativeBase,
            99 => Stop,
            _ => panic!("Unknown Op code {:?}", code),
        }
    }
    fn value_count(&self) -> usize {
        match self {
            Add | Multiply | LessThan | Equals => 4,
            JumpIfTrue | JumpIfFalse => 3,
            Input | Output | ShiftRelativeBase => 2,
            Stop => 1,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Mode {
    Position,
    Immediate,
    Relative,
}
impl Mode {
    fn extract_modes(s: &str) -> Vec<Mode> {
        vec![
            Mode::from_code(to_num(&s[2..=2])),
            Mode::from_code(to_num(&s[1..=1])),
            Mode::from_code(to_num(&s[0..=0])),
        ]
    }
    fn from_code(code: isize) -> Mode {
        match code {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            _ => panic!("Unknown Mode code {:?}", code),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point(isize, isize);
#[derive(Debug)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}
#[derive(Debug)]
struct Robot {
    icc: IntCodeComputer,
    loc: Point,
    dir: Dir,
    painted_panels: HashSet<Point>,
    canvas: HashMap<Point, isize>, // 0 or 1
}
impl Robot {
    fn new(input: &mut Vec<isize>, initial_color: Option<isize>) -> Self {
        let icc = IntCodeComputer::new(input);
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
    fn run(&mut self) {
        println!("{:?} {}", self.loc, self.canvas.get(&self.loc).unwrap());
        let mut counter = 1;
        let mut input = self.provide_input();
        loop {
            println!(
                "----------------- {}: {:?} {} -----------------",
                counter,
                self.loc,
                self.canvas.get(&self.loc).unwrap()
            );
            counter += 1;
            if let Some(color) = self.process(input) {
                self.paint(color);
                if let Some(direction) = self.process(input) {
                    self.turn(direction);
                    self.step();
                    input = self.provide_input();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    fn provide_input(&mut self) -> isize {
        *self.canvas.get(&self.loc).unwrap()
    }
    fn process(&mut self, input: isize) -> Option<isize> {
        self.icc.process_int_code_with_input(input as isize)
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
enum Tile {
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
struct ArcadeCabinet {
    icc: IntCodeComputer,
}
impl ArcadeCabinet {
    fn new(input: &mut Vec<isize>) -> Self {
        let icc = IntCodeComputer::new(input);
        ArcadeCabinet { icc }
    }
    fn run(&mut self) -> Vec<(Point, Tile)> {
        self.run_with(false)
    }
    fn play(&mut self) -> Vec<(Point, Tile)> {
        self.run_with(true)
    }
    fn run_with(&mut self, play: bool) -> Vec<(Point, Tile)> {
        let mut joystick = JoyStick::Neutral.to_input();
        if play {
            self.icc.instr[0] = 2;
        }
        let mut outputs: Vec<(Point, Tile)> = vec![];
        let mut ball: Option<Point> = None;
        let mut paddle: Option<Point> = None;
        while let Some(first) = self.icc.process_int_code_with_input(joystick) {
            if let Some(second) = self.icc.process_int_code_with_input(joystick) {
                if let Some(third) = self.icc.process_int_code_with_input(joystick) {
                    let pos = Point(first, second);
                    if pos == Point(-1, 0) {
                        println!("Score = {}", third);
                    } else {
                        let tile = Tile::from(third);
                        if play {
                            match tile {
                                Tile::Ball => {
                                    ball = Some(pos.clone());
                                    joystick = ArcadeCabinet::calc_input(&ball, &paddle).to_input();
                                    println!(
                                        "Ball: ball {:?}, paddle {:?}, joystick {:?}",
                                        pos, paddle, joystick
                                    );
                                }
                                Tile::Paddle => {
                                    paddle = Some(pos.clone());
                                    joystick = ArcadeCabinet::calc_input(&ball, &paddle).to_input();
                                    println!(
                                        "Paddle: ball {:?}, paddle {:?}, joystick {:?}",
                                        ball, pos, joystick
                                    );
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

mod tests {
    use crate::{ArcadeCabinet, IntCodeComputer, Op, Op::*, Point, Robot, Tile};

    // day 13, part 1

    #[test]
    fn day_13_part1() {
        let mut arcade = ArcadeCabinet::new(&mut day_13_puzzle_input());
        let tiles = arcade.run();
        let (block_count, _, _, _, _) = stats(&tiles);

        assert_eq!(block_count, 265)
    }
    #[test]
    fn day_13_part2() {
        let mut arcade = ArcadeCabinet::new(&mut day_13_puzzle_input());
        let tiles: Vec<(Point, Tile)> = arcade.play();
        assert_eq!(tiles.len(), 26947); // Score 13331
    }

    fn stats(tiles: &Vec<(Point, Tile)>) -> (usize, usize, usize, usize, usize) {
        let blocks = tiles.iter().filter(|(_, t)| t == &Tile::Block).count();
        let balls = tiles.iter().filter(|(_, t)| t == &Tile::Ball).count();
        let paddles = tiles.iter().filter(|(_, t)| t == &Tile::Paddle).count();
        let walls = tiles.iter().filter(|(_, t)| t == &Tile::Wall).count();
        let empty = tiles.iter().filter(|(_, t)| t == &Tile::Empty).count();
        println!(
            "Tiles: {} blocks, {} balls, {} paddles, {} walls, {} empty, {} total",
            blocks,
            balls,
            paddles,
            walls,
            empty,
            tiles.len(),
        );
        (blocks, balls, paddles, walls, empty)
    }

    fn day_13_puzzle_input() -> Vec<isize> {
        vec![
            1, 380, 379, 385, 1008, 2267, 610415, 381, 1005, 381, 12, 99, 109, 2268, 1101, 0, 0,
            383, 1101, 0, 0, 382, 20102, 1, 382, 1, 20101, 0, 383, 2, 21101, 37, 0, 0, 1106, 0,
            578, 4, 382, 4, 383, 204, 1, 1001, 382, 1, 382, 1007, 382, 37, 381, 1005, 381, 22,
            1001, 383, 1, 383, 1007, 383, 22, 381, 1005, 381, 18, 1006, 385, 69, 99, 104, -1, 104,
            0, 4, 386, 3, 384, 1007, 384, 0, 381, 1005, 381, 94, 107, 0, 384, 381, 1005, 381, 108,
            1105, 1, 161, 107, 1, 392, 381, 1006, 381, 161, 1101, -1, 0, 384, 1106, 0, 119, 1007,
            392, 35, 381, 1006, 381, 161, 1101, 0, 1, 384, 21001, 392, 0, 1, 21102, 1, 20, 2,
            21101, 0, 0, 3, 21102, 138, 1, 0, 1105, 1, 549, 1, 392, 384, 392, 21002, 392, 1, 1,
            21101, 0, 20, 2, 21101, 3, 0, 3, 21101, 161, 0, 0, 1106, 0, 549, 1101, 0, 0, 384,
            20001, 388, 390, 1, 20101, 0, 389, 2, 21102, 1, 180, 0, 1105, 1, 578, 1206, 1, 213,
            1208, 1, 2, 381, 1006, 381, 205, 20001, 388, 390, 1, 21002, 389, 1, 2, 21101, 205, 0,
            0, 1106, 0, 393, 1002, 390, -1, 390, 1102, 1, 1, 384, 21002, 388, 1, 1, 20001, 389,
            391, 2, 21102, 228, 1, 0, 1106, 0, 578, 1206, 1, 261, 1208, 1, 2, 381, 1006, 381, 253,
            20101, 0, 388, 1, 20001, 389, 391, 2, 21101, 253, 0, 0, 1105, 1, 393, 1002, 391, -1,
            391, 1101, 1, 0, 384, 1005, 384, 161, 20001, 388, 390, 1, 20001, 389, 391, 2, 21102, 1,
            279, 0, 1105, 1, 578, 1206, 1, 316, 1208, 1, 2, 381, 1006, 381, 304, 20001, 388, 390,
            1, 20001, 389, 391, 2, 21101, 304, 0, 0, 1105, 1, 393, 1002, 390, -1, 390, 1002, 391,
            -1, 391, 1101, 1, 0, 384, 1005, 384, 161, 20102, 1, 388, 1, 20101, 0, 389, 2, 21101, 0,
            0, 3, 21102, 338, 1, 0, 1105, 1, 549, 1, 388, 390, 388, 1, 389, 391, 389, 20102, 1,
            388, 1, 20101, 0, 389, 2, 21101, 0, 4, 3, 21102, 1, 365, 0, 1106, 0, 549, 1007, 389,
            21, 381, 1005, 381, 75, 104, -1, 104, 0, 104, 0, 99, 0, 1, 0, 0, 0, 0, 0, 0, 265, 16,
            17, 1, 1, 18, 109, 3, 21202, -2, 1, 1, 21201, -1, 0, 2, 21102, 1, 0, 3, 21102, 414, 1,
            0, 1105, 1, 549, 22101, 0, -2, 1, 21202, -1, 1, 2, 21101, 429, 0, 0, 1105, 1, 601,
            2102, 1, 1, 435, 1, 386, 0, 386, 104, -1, 104, 0, 4, 386, 1001, 387, -1, 387, 1005,
            387, 451, 99, 109, -3, 2106, 0, 0, 109, 8, 22202, -7, -6, -3, 22201, -3, -5, -3, 21202,
            -4, 64, -2, 2207, -3, -2, 381, 1005, 381, 492, 21202, -2, -1, -1, 22201, -3, -1, -3,
            2207, -3, -2, 381, 1006, 381, 481, 21202, -4, 8, -2, 2207, -3, -2, 381, 1005, 381, 518,
            21202, -2, -1, -1, 22201, -3, -1, -3, 2207, -3, -2, 381, 1006, 381, 507, 2207, -3, -4,
            381, 1005, 381, 540, 21202, -4, -1, -1, 22201, -3, -1, -3, 2207, -3, -4, 381, 1006,
            381, 529, 22101, 0, -3, -7, 109, -8, 2106, 0, 0, 109, 4, 1202, -2, 37, 566, 201, -3,
            566, 566, 101, 639, 566, 566, 2102, 1, -1, 0, 204, -3, 204, -2, 204, -1, 109, -4, 2105,
            1, 0, 109, 3, 1202, -1, 37, 593, 201, -2, 593, 593, 101, 639, 593, 593, 21001, 0, 0,
            -2, 109, -3, 2105, 1, 0, 109, 3, 22102, 22, -2, 1, 22201, 1, -1, 1, 21102, 1, 409, 2,
            21102, 1, 463, 3, 21102, 1, 814, 4, 21102, 1, 630, 0, 1106, 0, 456, 21201, 1, 1453, -2,
            109, -3, 2105, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 2, 2, 0,
            2, 2, 0, 2, 2, 2, 0, 0, 0, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 0, 2, 0, 0, 0,
            0, 1, 1, 0, 0, 2, 0, 0, 2, 2, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 2, 2, 0, 2, 2, 2, 2, 0,
            0, 2, 2, 0, 0, 2, 0, 2, 0, 1, 1, 0, 0, 2, 2, 2, 0, 0, 0, 2, 2, 2, 2, 0, 0, 2, 2, 0, 2,
            0, 2, 0, 2, 2, 0, 2, 2, 2, 0, 2, 2, 0, 2, 2, 2, 0, 1, 1, 0, 2, 0, 0, 2, 2, 2, 0, 2, 2,
            0, 2, 0, 2, 2, 2, 2, 0, 0, 2, 2, 2, 0, 2, 2, 0, 0, 0, 0, 2, 2, 0, 0, 2, 0, 1, 1, 0, 2,
            2, 2, 0, 0, 0, 2, 2, 2, 0, 2, 2, 2, 2, 2, 0, 0, 0, 2, 2, 0, 2, 2, 2, 0, 2, 0, 0, 0, 0,
            0, 2, 2, 0, 1, 1, 0, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 0, 0, 0, 0, 2, 0, 0, 2, 0, 2, 2,
            2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 0, 1, 1, 0, 0, 0, 2, 0, 0, 2, 2, 2, 0, 2, 0, 0, 0, 0,
            2, 0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 2, 0, 0, 2, 0, 2, 2, 2, 0, 1, 1, 0, 2, 0, 0, 2, 2, 0,
            0, 0, 2, 0, 0, 0, 2, 2, 2, 2, 0, 2, 2, 0, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 1,
            1, 0, 2, 0, 0, 0, 2, 0, 2, 2, 2, 2, 2, 0, 0, 2, 0, 2, 2, 0, 0, 2, 2, 0, 2, 2, 0, 2, 2,
            0, 2, 0, 0, 2, 2, 0, 1, 1, 0, 2, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 0, 2, 2, 0, 2, 0, 0, 0,
            2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 1, 1, 0, 0, 0, 0, 2, 0, 2, 2, 2, 2, 0, 2,
            2, 0, 2, 2, 2, 0, 2, 2, 0, 2, 2, 2, 0, 2, 0, 0, 2, 2, 0, 2, 0, 0, 0, 1, 1, 0, 0, 2, 2,
            2, 0, 2, 0, 2, 0, 2, 2, 2, 0, 0, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 2, 0, 2, 2, 0, 2, 0, 2,
            2, 0, 1, 1, 0, 0, 2, 2, 2, 2, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0,
            0, 2, 0, 2, 2, 2, 0, 2, 0, 0, 1, 1, 0, 2, 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2,
            0, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 2, 2, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 1, 72, 10, 67, 45, 58, 25, 55, 73, 97, 49, 19, 51, 58, 95, 30,
            82, 74, 9, 98, 96, 38, 64, 30, 45, 14, 73, 42, 5, 3, 61, 68, 23, 18, 14, 9, 16, 21, 7,
            77, 39, 38, 16, 82, 17, 58, 87, 90, 64, 52, 1, 96, 67, 66, 16, 65, 15, 22, 41, 69, 90,
            93, 92, 96, 45, 68, 17, 63, 51, 15, 61, 51, 93, 65, 55, 42, 76, 48, 52, 31, 98, 6, 88,
            69, 65, 65, 30, 51, 88, 4, 13, 36, 90, 80, 23, 31, 42, 63, 86, 52, 15, 79, 78, 59, 77,
            57, 71, 84, 81, 73, 56, 1, 5, 7, 86, 75, 31, 63, 76, 21, 73, 16, 41, 86, 15, 78, 85, 2,
            79, 63, 54, 79, 65, 87, 13, 86, 96, 81, 69, 27, 76, 8, 48, 5, 79, 10, 74, 76, 86, 95,
            55, 72, 52, 23, 41, 50, 46, 68, 29, 86, 61, 96, 29, 34, 40, 86, 86, 1, 20, 90, 35, 69,
            64, 50, 51, 75, 65, 93, 19, 5, 15, 96, 3, 88, 8, 43, 66, 88, 72, 84, 69, 42, 4, 95, 51,
            80, 81, 27, 75, 92, 22, 45, 54, 63, 51, 82, 91, 13, 25, 54, 41, 84, 84, 29, 98, 50, 91,
            11, 40, 69, 13, 47, 42, 72, 46, 87, 31, 27, 98, 65, 94, 26, 51, 79, 39, 29, 38, 42, 46,
            25, 36, 26, 66, 12, 93, 58, 1, 61, 41, 37, 57, 60, 60, 9, 70, 63, 26, 56, 1, 27, 5, 11,
            93, 17, 48, 95, 19, 79, 16, 14, 16, 29, 79, 56, 16, 26, 37, 50, 10, 38, 53, 4, 10, 3,
            57, 20, 59, 16, 51, 88, 66, 74, 91, 56, 42, 84, 30, 36, 31, 36, 58, 68, 66, 91, 36, 71,
            30, 39, 96, 50, 84, 76, 95, 14, 89, 75, 59, 77, 66, 36, 88, 62, 60, 3, 45, 13, 39, 48,
            33, 59, 21, 19, 35, 90, 81, 66, 52, 75, 34, 70, 55, 56, 47, 22, 20, 87, 73, 73, 76, 73,
            8, 96, 55, 46, 5, 1, 64, 27, 8, 37, 87, 50, 8, 79, 74, 63, 26, 43, 44, 2, 85, 91, 28,
            13, 16, 15, 55, 87, 94, 28, 86, 66, 29, 34, 46, 18, 41, 37, 94, 63, 31, 78, 48, 17, 4,
            25, 62, 15, 10, 18, 19, 97, 50, 78, 5, 79, 5, 70, 64, 86, 61, 58, 59, 61, 5, 71, 68,
            14, 24, 17, 56, 85, 52, 64, 92, 45, 90, 94, 55, 47, 5, 56, 59, 20, 15, 41, 36, 58, 55,
            25, 47, 45, 69, 58, 36, 44, 80, 94, 52, 84, 17, 27, 20, 44, 51, 93, 10, 56, 77, 45, 29,
            93, 63, 96, 95, 47, 31, 63, 69, 64, 74, 53, 34, 36, 20, 14, 40, 30, 61, 86, 15, 3, 94,
            61, 43, 75, 59, 64, 41, 34, 98, 32, 65, 73, 18, 30, 46, 66, 38, 68, 25, 96, 16, 37, 54,
            38, 44, 26, 52, 1, 2, 21, 93, 37, 26, 4, 45, 69, 82, 59, 34, 55, 34, 77, 88, 46, 70,
            32, 56, 82, 10, 20, 31, 40, 20, 55, 3, 3, 93, 95, 65, 56, 61, 68, 41, 35, 62, 20, 58,
            55, 42, 41, 40, 33, 51, 6, 52, 84, 27, 62, 81, 32, 35, 87, 97, 79, 7, 97, 77, 40, 48,
            74, 4, 6, 36, 58, 59, 25, 6, 5, 84, 7, 44, 51, 88, 37, 9, 30, 29, 26, 91, 41, 72, 39,
            24, 68, 58, 49, 80, 49, 43, 98, 43, 92, 9, 49, 64, 10, 96, 50, 86, 56, 2, 72, 58, 80,
            57, 77, 61, 74, 14, 42, 50, 55, 40, 21, 77, 20, 19, 16, 80, 84, 92, 27, 32, 37, 80, 59,
            69, 13, 11, 19, 6, 94, 54, 88, 51, 69, 41, 54, 68, 36, 82, 68, 19, 77, 85, 37, 5, 58,
            61, 72, 5, 67, 17, 35, 29, 18, 71, 46, 5, 29, 8, 93, 97, 36, 37, 25, 93, 27, 33, 93,
            79, 10, 84, 75, 6, 91, 98, 34, 32, 37, 70, 18, 84, 52, 32, 11, 88, 44, 69, 58, 92, 52,
            68, 77, 39, 90, 9, 58, 74, 1, 53, 56, 64, 75, 46, 59, 39, 52, 32, 41, 62, 81, 75, 7,
            93, 29, 89, 51, 34, 31, 93, 70, 94, 30, 98, 68, 3, 60, 2, 2, 49, 31, 15, 65, 11, 78,
            70, 2, 50, 29, 9, 9, 85, 65, 52, 28, 95, 55, 77, 98, 29, 65, 56, 51, 32, 44, 42, 82,
            14, 29, 22, 5, 29, 65, 86, 84, 88, 58, 63, 10, 13, 13, 51, 97, 17, 57, 19, 39, 83, 72,
            93, 15, 54, 31, 83, 3, 43, 21, 83, 74, 2, 86, 47, 25, 89, 20, 11, 68, 80, 29, 21, 58,
            69, 610415,
        ]
    }

    // day 11

    #[ignore] // takes too long
    #[test]
    fn part1() {
        let mut robot = Robot::new(&mut day11_puzzle_input(), None);
        robot.run();
        println!("number of painted panels = {}", robot.painted_panels.len());
        assert_eq!(robot.painted_panels.len(), 2373);
    }
    #[test]
    fn part2() {
        let mut robot = Robot::new(&mut day11_puzzle_input(), Some(1));
        robot.run();
        println!("number of painted panels = {}", robot.painted_panels.len());
        let min_x = robot.canvas.iter().map(|(p, _)| p.0).min().unwrap();
        let max_x = robot.canvas.iter().map(|(p, _)| p.0).max().unwrap();
        let min_y = robot.canvas.iter().map(|(p, _)| p.1).min().unwrap();
        let max_y = robot.canvas.iter().map(|(p, _)| p.1).max().unwrap();
        println!(
            "x: from {} to {}, y: from {} to {}",
            min_x, max_x, min_y, max_y
        );

        // print the result: PCKRLPUK
        let mut y = max_y;
        while y >= min_y {
            let mut line: Vec<char> = vec![];
            for x in min_x..=max_x {
                if let Some(&color) = robot.canvas.get(&Point(x, y)) {
                    if color == 0 {
                        line.push('#');
                    } else {
                        line.push(' ');
                    }
                } else {
                    line.push(' ');
                }
            }
            println!("{}", line.into_iter().collect::<String>());
            y -= 1;
        }
        assert_eq!(robot.painted_panels.len(), 249);
        /*
        #   ###  ## ## #   ## ####   ## ## # ## ##
          ## # ## # # ## ## # #### ## # ## # # ####
          ## # ####  ### ## # #### ## # ## #  #####
        #   ## #### # ##   ## ####   ## ## # # ###
        # #### ## # # ## # ## #### #### ## # # ###
          #####  ## ## # ## #    # #####  ## ## #
                */
    }
    fn day11_puzzle_input() -> Vec<isize> {
        vec![
            3,
            8,
            1005,
            8,
            332,
            1106,
            0,
            11,
            0,
            0,
            0,
            104,
            1,
            104,
            0,
            3,
            8,
            102,
            -1,
            8,
            10,
            101,
            1,
            10,
            10,
            4,
            10,
            108,
            1,
            8,
            10,
            4,
            10,
            101,
            0,
            8,
            28,
            3,
            8,
            102,
            -1,
            8,
            10,
            1001,
            10,
            1,
            10,
            4,
            10,
            1008,
            8,
            1,
            10,
            4,
            10,
            101,
            0,
            8,
            51,
            1,
            1103,
            5,
            10,
            1,
            1104,
            9,
            10,
            2,
            1003,
            0,
            10,
            1,
            5,
            16,
            10,
            3,
            8,
            102,
            -1,
            8,
            10,
            101,
            1,
            10,
            10,
            4,
            10,
            108,
            0,
            8,
            10,
            4,
            10,
            1001,
            8,
            0,
            88,
            1006,
            0,
            2,
            1006,
            0,
            62,
            2,
            8,
            2,
            10,
            3,
            8,
            1002,
            8,
            -1,
            10,
            101,
            1,
            10,
            10,
            4,
            10,
            1008,
            8,
            1,
            10,
            4,
            10,
            102,
            1,
            8,
            121,
            1006,
            0,
            91,
            1006,
            0,
            22,
            1006,
            0,
            23,
            1006,
            0,
            1,
            3,
            8,
            102,
            -1,
            8,
            10,
            1001,
            10,
            1,
            10,
            4,
            10,
            1008,
            8,
            1,
            10,
            4,
            10,
            101,
            0,
            8,
            155,
            1006,
            0,
            97,
            1,
            1004,
            2,
            10,
            2,
            1003,
            6,
            10,
            3,
            8,
            1002,
            8,
            -1,
            10,
            101,
            1,
            10,
            10,
            4,
            10,
            108,
            0,
            8,
            10,
            4,
            10,
            1002,
            8,
            1,
            187,
            1,
            104,
            15,
            10,
            2,
            107,
            9,
            10,
            1006,
            0,
            37,
            1006,
            0,
            39,
            3,
            8,
            1002,
            8,
            -1,
            10,
            1001,
            10,
            1,
            10,
            4,
            10,
            108,
            0,
            8,
            10,
            4,
            10,
            102,
            1,
            8,
            223,
            2,
            2,
            17,
            10,
            1,
            1102,
            5,
            10,
            3,
            8,
            1002,
            8,
            -1,
            10,
            101,
            1,
            10,
            10,
            4,
            10,
            108,
            0,
            8,
            10,
            4,
            10,
            1001,
            8,
            0,
            253,
            3,
            8,
            102,
            -1,
            8,
            10,
            1001,
            10,
            1,
            10,
            4,
            10,
            1008,
            8,
            1,
            10,
            4,
            10,
            1002,
            8,
            1,
            276,
            1006,
            0,
            84,
            3,
            8,
            102,
            -1,
            8,
            10,
            101,
            1,
            10,
            10,
            4,
            10,
            1008,
            8,
            0,
            10,
            4,
            10,
            1001,
            8,
            0,
            301,
            2,
            1009,
            9,
            10,
            1006,
            0,
            10,
            2,
            102,
            15,
            10,
            101,
            1,
            9,
            9,
            1007,
            9,
            997,
            10,
            1005,
            10,
            15,
            99,
            109,
            654,
            104,
            0,
            104,
            1,
            21102,
            1,
            936995738516,
            1,
            21101,
            0,
            349,
            0,
            1105,
            1,
            453,
            21102,
            1,
            825595015976,
            1,
            21102,
            1,
            360,
            0,
            1105,
            1,
            453,
            3,
            10,
            104,
            0,
            104,
            1,
            3,
            10,
            104,
            0,
            104,
            0,
            3,
            10,
            104,
            0,
            104,
            1,
            3,
            10,
            104,
            0,
            104,
            1,
            3,
            10,
            104,
            0,
            104,
            0,
            3,
            10,
            104,
            0,
            104,
            1,
            21102,
            46375541763,
            1,
            1,
            21101,
            0,
            407,
            0,
            1105,
            1,
            453,
            21102,
            1,
            179339005019,
            1,
            21101,
            0,
            418,
            0,
            1106,
            0,
            453,
            3,
            10,
            104,
            0,
            104,
            0,
            3,
            10,
            104,
            0,
            104,
            0,
            21102,
            825012036372,
            1,
            1,
            21102,
            441,
            1,
            0,
            1105,
            1,
            453,
            21101,
            988648461076,
            0,
            1,
            21101,
            452,
            0,
            0,
            1105,
            1,
            453,
            99,
            109,
            2,
            22102,
            1,
            -1,
            1,
            21102,
            40,
            1,
            2,
            21102,
            484,
            1,
            3,
            21101,
            0,
            474,
            0,
            1106,
            0,
            517,
            109,
            -2,
            2105,
            1,
            0,
            0,
            1,
            0,
            0,
            1,
            109,
            2,
            3,
            10,
            204,
            -1,
            1001,
            479,
            480,
            495,
            4,
            0,
            1001,
            479,
            1,
            479,
            108,
            4,
            479,
            10,
            1006,
            10,
            511,
            1102,
            1,
            0,
            479,
            109,
            -2,
            2105,
            1,
            0,
            0,
            109,
            4,
            2102,
            1,
            -1,
            516,
            1207,
            -3,
            0,
            10,
            1006,
            10,
            534,
            21101,
            0,
            0,
            -3,
            21202,
            -3,
            1,
            1,
            22101,
            0,
            -2,
            2,
            21102,
            1,
            1,
            3,
            21102,
            553,
            1,
            0,
            1106,
            0,
            558,
            109,
            -4,
            2106,
            0,
            0,
            109,
            5,
            1207,
            -3,
            1,
            10,
            1006,
            10,
            581,
            2207,
            -4,
            -2,
            10,
            1006,
            10,
            581,
            22102,
            1,
            -4,
            -4,
            1105,
            1,
            649,
            21202,
            -4,
            1,
            1,
            21201,
            -3,
            -1,
            2,
            21202,
            -2,
            2,
            3,
            21101,
            0,
            600,
            0,
            1105,
            1,
            558,
            21201,
            1,
            0,
            -4,
            21101,
            0,
            1,
            -1,
            2207,
            -4,
            -2,
            10,
            1006,
            10,
            619,
            21101,
            0,
            0,
            -1,
            22202,
            -2,
            -1,
            -2,
            2107,
            0,
            -3,
            10,
            1006,
            10,
            641,
            22102,
            1,
            -1,
            1,
            21102,
            1,
            641,
            0,
            106,
            0,
            516,
            21202,
            -2,
            -1,
            -2,
            22201,
            -4,
            -2,
            -4,
            109,
            -5,
            2105,
            1,
            0,
        ]
    }

    // day 9 part 1

    #[test]
    fn test_autoextend_on_get() {
        let mut icc = IntCodeComputer::new(&mut vec![]);
        assert_eq!(icc.get(0), 0);
        assert_eq!(icc.instr.len(), 1);
    }

    #[test]
    fn test_autoextend_on_set() {
        let mut icc = IntCodeComputer::new(&mut vec![]);
        icc.set(0, 123);
        assert_eq!(icc.instr.len(), 1);
        assert_eq!(icc.get(0), 123);
    }

    #[ignore]
    #[test]
    fn day9_part1_example1() {
        let mut icc = IntCodeComputer::new(&mut vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]);
        assert_eq!(icc.process_int_code_with_default_input(), Some(99));
    }

    #[test]
    fn day9_part1_example2() {
        let mut icc = IntCodeComputer::new(&mut vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        assert_eq!(
            icc.process_int_code_with_default_input(),
            Some(1219070632396864)
        );
    }

    #[test]
    fn day9_part1_example3() {
        let mut icc = IntCodeComputer::new(&mut vec![104, 1125899906842624, 99]);
        assert_eq!(
            icc.process_int_code_with_default_input(),
            Some(1125899906842624)
        );
    }
    #[test]
    fn day9_add_with_relative_input() {
        let mut icc = IntCodeComputer::new(&mut vec![109, 6, 2201, 1, 2, 9, 99, 11, 22, 0]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![109, 6, 2201, 1, 2, 9, 99, 11, 22, 33]);
    }
    #[test]
    fn day9_add_with_relative_output() {
        let mut icc = IntCodeComputer::new(&mut vec![109, 6, 21101, 11, 22, 1, 99, 0]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![109, 6, 21101, 11, 22, 1, 99, 33]);
    }
    #[test]
    fn day9_part1_output_with_relative_base_above_0() {
        let mut icc = IntCodeComputer::new(&mut vec![109, 10, 204, -5, 99, 123]);
        assert_eq!(icc.process_int_code_with_default_input(), Some(123));
    }
    #[test]
    fn day9_part1_output_with_relative_base_below_0() {
        let mut icc = IntCodeComputer::new(&mut vec![109, -5, 204, 10, 99, 123]);
        assert_eq!(icc.process_int_code_with_default_input(), Some(123));
    }
    #[test]
    fn day9_part1_mirror_relative_input_to_output() {
        let mut icc = IntCodeComputer::new(&mut vec![203, 3, 104, 0, 99]);
        assert_eq!(icc.process_int_code_with_input(22), Some(22));
    }
    #[test]
    fn day9_part1_mirror_shifted_relative_input_to_output() {
        let mut icc = IntCodeComputer::new(&mut vec![109, 6, 203, -1, 104, 0, 99]);
        println!("{:?}", icc.instr);
        assert_eq!(icc.process_int_code_with_input(33), Some(33));
    }

    #[test]
    fn day9_part1() {
        let mut icc = IntCodeComputer::new(&mut day9_puzzle_input());
        assert_eq!(icc.process_int_code_with_input(1), Some(3518157894));
    }
    #[ignore]
    #[test]
    fn day9_part2() {
        let mut icc = IntCodeComputer::new(&mut day9_puzzle_input());
        assert_eq!(icc.process_int_code_with_input(2), Some(80379));
    }

    fn day9_puzzle_input() -> Vec<isize> {
        vec![
            1102, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1101, 3, 0, 1000,
            109, 988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65,
            1008, 1000, 2, 63, 1005, 63, 904, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99,
            4, 0, 104, 0, 99, 4, 17, 104, 0, 99, 0, 0, 1101, 25, 0, 1016, 1102, 760, 1, 1023, 1102,
            1, 20, 1003, 1102, 1, 22, 1015, 1102, 1, 34, 1000, 1101, 0, 32, 1006, 1101, 21, 0,
            1017, 1102, 39, 1, 1010, 1101, 30, 0, 1005, 1101, 0, 1, 1021, 1101, 0, 0, 1020, 1102,
            1, 35, 1007, 1102, 1, 23, 1014, 1102, 1, 29, 1019, 1101, 767, 0, 1022, 1102, 216, 1,
            1025, 1102, 38, 1, 1011, 1101, 778, 0, 1029, 1102, 1, 31, 1009, 1101, 0, 28, 1004,
            1101, 33, 0, 1008, 1102, 1, 444, 1027, 1102, 221, 1, 1024, 1102, 1, 451, 1026, 1101,
            787, 0, 1028, 1101, 27, 0, 1018, 1101, 0, 24, 1013, 1102, 26, 1, 1012, 1101, 0, 36,
            1002, 1102, 37, 1, 1001, 109, 28, 21101, 40, 0, -9, 1008, 1019, 41, 63, 1005, 63, 205,
            1001, 64, 1, 64, 1105, 1, 207, 4, 187, 1002, 64, 2, 64, 109, -9, 2105, 1, 5, 4, 213,
            1106, 0, 225, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -9, 1206, 10, 243, 4, 231, 1001,
            64, 1, 64, 1105, 1, 243, 1002, 64, 2, 64, 109, -3, 1208, 2, 31, 63, 1005, 63, 261, 4,
            249, 1106, 0, 265, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 5, 21108, 41, 41, 0, 1005,
            1012, 287, 4, 271, 1001, 64, 1, 64, 1105, 1, 287, 1002, 64, 2, 64, 109, 6, 21102, 42,
            1, -5, 1008, 1013, 45, 63, 1005, 63, 307, 1105, 1, 313, 4, 293, 1001, 64, 1, 64, 1002,
            64, 2, 64, 109, -9, 1201, 0, 0, 63, 1008, 63, 29, 63, 1005, 63, 333, 1106, 0, 339, 4,
            319, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -13, 2102, 1, 4, 63, 1008, 63, 34, 63,
            1005, 63, 361, 4, 345, 1105, 1, 365, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 5, 1201, 7,
            0, 63, 1008, 63, 33, 63, 1005, 63, 387, 4, 371, 1105, 1, 391, 1001, 64, 1, 64, 1002,
            64, 2, 64, 109, 7, 1202, 1, 1, 63, 1008, 63, 32, 63, 1005, 63, 411, 1105, 1, 417, 4,
            397, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 20, 1205, -7, 431, 4, 423, 1106, 0, 435,
            1001, 64, 1, 64, 1002, 64, 2, 64, 109, 2, 2106, 0, -3, 1001, 64, 1, 64, 1105, 1, 453,
            4, 441, 1002, 64, 2, 64, 109, -7, 21101, 43, 0, -9, 1008, 1014, 43, 63, 1005, 63, 479,
            4, 459, 1001, 64, 1, 64, 1105, 1, 479, 1002, 64, 2, 64, 109, -5, 21108, 44, 43, 0,
            1005, 1018, 495, 1105, 1, 501, 4, 485, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -7, 1205,
            9, 517, 1001, 64, 1, 64, 1105, 1, 519, 4, 507, 1002, 64, 2, 64, 109, 11, 1206, -1, 531,
            1106, 0, 537, 4, 525, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -15, 1208, 0, 36, 63,
            1005, 63, 557, 1001, 64, 1, 64, 1106, 0, 559, 4, 543, 1002, 64, 2, 64, 109, 7, 2101, 0,
            -7, 63, 1008, 63, 35, 63, 1005, 63, 581, 4, 565, 1106, 0, 585, 1001, 64, 1, 64, 1002,
            64, 2, 64, 109, -3, 21107, 45, 46, 4, 1005, 1015, 607, 4, 591, 1001, 64, 1, 64, 1105,
            1, 607, 1002, 64, 2, 64, 109, -16, 2102, 1, 10, 63, 1008, 63, 31, 63, 1005, 63, 631,
            1001, 64, 1, 64, 1106, 0, 633, 4, 613, 1002, 64, 2, 64, 109, 1, 2107, 33, 10, 63, 1005,
            63, 649, 1106, 0, 655, 4, 639, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 17, 2101, 0, -9,
            63, 1008, 63, 31, 63, 1005, 63, 679, 1001, 64, 1, 64, 1106, 0, 681, 4, 661, 1002, 64,
            2, 64, 109, -6, 2107, 34, 0, 63, 1005, 63, 703, 4, 687, 1001, 64, 1, 64, 1106, 0, 703,
            1002, 64, 2, 64, 109, 5, 1207, -5, 34, 63, 1005, 63, 719, 1105, 1, 725, 4, 709, 1001,
            64, 1, 64, 1002, 64, 2, 64, 109, -15, 1202, 6, 1, 63, 1008, 63, 20, 63, 1005, 63, 751,
            4, 731, 1001, 64, 1, 64, 1105, 1, 751, 1002, 64, 2, 64, 109, 21, 2105, 1, 5, 1001, 64,
            1, 64, 1106, 0, 769, 4, 757, 1002, 64, 2, 64, 109, 5, 2106, 0, 5, 4, 775, 1001, 64, 1,
            64, 1106, 0, 787, 1002, 64, 2, 64, 109, -27, 1207, 4, 35, 63, 1005, 63, 809, 4, 793,
            1001, 64, 1, 64, 1106, 0, 809, 1002, 64, 2, 64, 109, 13, 2108, 33, -1, 63, 1005, 63,
            831, 4, 815, 1001, 64, 1, 64, 1106, 0, 831, 1002, 64, 2, 64, 109, 4, 21107, 46, 45, 1,
            1005, 1014, 851, 1001, 64, 1, 64, 1105, 1, 853, 4, 837, 1002, 64, 2, 64, 109, 3, 21102,
            47, 1, -3, 1008, 1013, 47, 63, 1005, 63, 875, 4, 859, 1106, 0, 879, 1001, 64, 1, 64,
            1002, 64, 2, 64, 109, -9, 2108, 28, 2, 63, 1005, 63, 895, 1106, 0, 901, 4, 885, 1001,
            64, 1, 64, 4, 64, 99, 21101, 27, 0, 1, 21102, 1, 915, 0, 1106, 0, 922, 21201, 1, 59074,
            1, 204, 1, 99, 109, 3, 1207, -2, 3, 63, 1005, 63, 964, 21201, -2, -1, 1, 21102, 942, 1,
            0, 1105, 1, 922, 21201, 1, 0, -1, 21201, -2, -3, 1, 21102, 1, 957, 0, 1105, 1, 922,
            22201, 1, -1, -2, 1106, 0, 968, 22102, 1, -2, -2, 109, -3, 2105, 1, 0,
        ]
    }

    // day 5 below

    #[test]
    fn op_from_int_code() {
        assert_eq!(Add, Op::from_code(1));
    }

    #[test]
    fn explanation_example() {
        let mut icc = IntCodeComputer::new(&mut vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(
            icc.instr,
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
    }

    #[test]
    fn add_example_1() {
        let mut icc = IntCodeComputer::new(&mut vec![1, 0, 0, 0, 99]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn mult_example_1() {
        let mut icc = IntCodeComputer::new(&mut vec![2, 3, 0, 3, 99]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn mult_example_2() {
        let mut icc = IntCodeComputer::new(&mut vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn add_example_2() {
        let mut icc = IntCodeComputer::new(&mut vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    // day 5 part 1

    #[test]
    fn multiply_example() {
        let mut icc = IntCodeComputer::new(&mut vec![1002, 4, 3, 4, 33]);
        assert_eq!(icc.process_int_code_with_default_input(), None);
        assert_eq!(icc.instr, vec![1002, 4, 3, 4, 99]);
    }

    fn day5_puzzle_input() -> Vec<isize> {
        vec![
            3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1, 192, 154, 224, 101, -161, 224, 224,
            4, 224, 102, 8, 223, 223, 101, 5, 224, 224, 1, 223, 224, 223, 1001, 157, 48, 224, 1001,
            224, -61, 224, 4, 224, 102, 8, 223, 223, 101, 2, 224, 224, 1, 223, 224, 223, 1102, 15,
            28, 225, 1002, 162, 75, 224, 1001, 224, -600, 224, 4, 224, 1002, 223, 8, 223, 1001,
            224, 1, 224, 1, 224, 223, 223, 102, 32, 57, 224, 1001, 224, -480, 224, 4, 224, 102, 8,
            223, 223, 101, 1, 224, 224, 1, 224, 223, 223, 1101, 6, 23, 225, 1102, 15, 70, 224,
            1001, 224, -1050, 224, 4, 224, 1002, 223, 8, 223, 101, 5, 224, 224, 1, 224, 223, 223,
            101, 53, 196, 224, 1001, 224, -63, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224, 1,
            224, 223, 223, 1101, 64, 94, 225, 1102, 13, 23, 225, 1101, 41, 8, 225, 2, 105, 187,
            224, 1001, 224, -60, 224, 4, 224, 1002, 223, 8, 223, 101, 6, 224, 224, 1, 224, 223,
            223, 1101, 10, 23, 225, 1101, 16, 67, 225, 1101, 58, 10, 225, 1101, 25, 34, 224, 1001,
            224, -59, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 3, 224, 1, 223, 224, 223, 4, 223,
            99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247,
            1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106,
            0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280,
            1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0,
            300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999,
            1108, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 329, 101, 1, 223, 223, 107, 226, 226,
            224, 1002, 223, 2, 223, 1005, 224, 344, 1001, 223, 1, 223, 107, 677, 226, 224, 102, 2,
            223, 223, 1005, 224, 359, 101, 1, 223, 223, 7, 677, 226, 224, 102, 2, 223, 223, 1005,
            224, 374, 101, 1, 223, 223, 108, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 389, 101,
            1, 223, 223, 1007, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 404, 101, 1, 223, 223,
            7, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 419, 101, 1, 223, 223, 1107, 226, 677,
            224, 1002, 223, 2, 223, 1005, 224, 434, 1001, 223, 1, 223, 1108, 226, 677, 224, 102, 2,
            223, 223, 1005, 224, 449, 101, 1, 223, 223, 108, 226, 677, 224, 102, 2, 223, 223, 1005,
            224, 464, 1001, 223, 1, 223, 8, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 479, 1001,
            223, 1, 223, 1007, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 494, 101, 1, 223, 223,
            1008, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 509, 101, 1, 223, 223, 1107, 677,
            226, 224, 1002, 223, 2, 223, 1006, 224, 524, 1001, 223, 1, 223, 108, 677, 677, 224,
            1002, 223, 2, 223, 1005, 224, 539, 1001, 223, 1, 223, 1107, 226, 226, 224, 1002, 223,
            2, 223, 1006, 224, 554, 1001, 223, 1, 223, 7, 226, 226, 224, 1002, 223, 2, 223, 1006,
            224, 569, 1001, 223, 1, 223, 8, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 584, 101,
            1, 223, 223, 1008, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 599, 101, 1, 223, 223,
            1007, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 614, 1001, 223, 1, 223, 8, 677, 677,
            224, 1002, 223, 2, 223, 1005, 224, 629, 101, 1, 223, 223, 107, 677, 677, 224, 102, 2,
            223, 223, 1005, 224, 644, 101, 1, 223, 223, 1108, 677, 226, 224, 102, 2, 223, 223,
            1005, 224, 659, 101, 1, 223, 223, 1008, 226, 226, 224, 102, 2, 223, 223, 1006, 224,
            674, 1001, 223, 1, 223, 4, 223, 99, 226,
        ]
    }

    #[ignore]
    #[test]
    fn day5_part_1() {
        let mut icc = IntCodeComputer::new(&mut day5_puzzle_input());
        assert_eq!(icc.process_int_code_with_input(1), Some(11049715));
    }

    // day 5 part 2

    #[test]
    fn input_equal_to_8_position_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(8), Some(1));
    }
    #[test]
    fn input_not_equal_to_8_position_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(9), Some(0));
    }
    #[test]
    fn input_less_than_8_position_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(7), Some(1));
    }
    #[test]
    fn input_not_less_than_8_position_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(8), Some(0));
    }
    #[test]
    fn input_equal_to_8_immediate_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 3, 1108, -1, 8, 3, 4, 3, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(8), Some(1));
    }
    #[test]
    fn input_not_equal_to_8_immediate_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 3, 1108, -1, 8, 3, 4, 3, 99, -1, 8]);
        assert_eq!(icc.process_int_code_with_input(9), Some(0));
    }
    #[test]
    fn input_less_than_to_8_immediate_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        assert_eq!(icc.process_int_code_with_input(7), Some(1));
    }
    #[test]
    fn input_not_less_than_to_8_immediate_mode() {
        let mut icc = IntCodeComputer::new(&mut vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        assert_eq!(icc.process_int_code_with_input(8), Some(0));
    }
    #[test]
    fn jump_test_position_mode_1() {
        let mut icc = IntCodeComputer::new(&mut vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        assert_eq!(icc.process_int_code_with_input(1), Some(1));
    }
    #[test]
    fn jump_test_position_mode_0() {
        let mut icc = IntCodeComputer::new(&mut vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        assert_eq!(icc.process_int_code_with_input(0), Some(0));
    }
    #[test]
    fn jump_test_immediate_mode_1() {
        let mut icc =
            IntCodeComputer::new(&mut vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        assert_eq!(icc.process_int_code_with_input(1), Some(1));
    }
    #[test]
    fn jump_test_immediate_mode_0() {
        let mut icc =
            IntCodeComputer::new(&mut vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        assert_eq!(icc.process_int_code_with_input(0), Some(0));
    }

    fn larger_example_input() -> Vec<isize> {
        vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ]
    }
    #[test]
    fn larger_example_less_than_8() {
        let mut icc = IntCodeComputer::new(&mut larger_example_input());
        assert_eq!(icc.process_int_code_with_input(7), Some(999));
    }
    #[test]
    fn larger_example_exactly_8() {
        let mut icc = IntCodeComputer::new(&mut larger_example_input());
        assert_eq!(icc.process_int_code_with_input(8), Some(1000));
    }
    #[test]
    fn larger_example_greater_than_8() {
        let mut icc = IntCodeComputer::new(&mut larger_example_input());
        assert_eq!(icc.process_int_code_with_input(9), Some(1001));
    }

    #[test]
    fn part_2() {
        let mut icc = IntCodeComputer::new(&mut day5_puzzle_input());
        assert_eq!(icc.process_int_code_with_input(5), Some(2140710));
    }
}
