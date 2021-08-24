use crate::point2d::Point2D;
use coffee::graphics::{Color, Frame, Mesh, Point, Rectangle, Shape, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};
use intcode::{IntCodeComputer, State, State::*};
use std::collections::{HashMap, VecDeque};
use std::ops::RangeInclusive;

const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};
const YELLOW: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 0.0,
    a: 1.0,
};
const GREEN: Color = Color {
    r: 0.25,
    g: 1.0,
    b: 0.25,
    a: 1.0,
};
const BLUE: Color = Color {
    r: 0.25,
    g: 0.25,
    b: 1.0,
    a: 1.0,
};
const GREY: Color = Color {
    r: 0.5,
    g: 0.5,
    b: 0.5,
    a: 1.0,
};

fn main() -> Result<()> {
    let tile_size = 40;
    ASCII::run(WindowSettings {
        title: String::from("Advent of code 2019 day 17"),
        size: ((58 + 1) * tile_size, (34 + 1) * tile_size),
        resizable: true,
        fullscreen: false,
    })
}

// day 17
#[derive(PartialEq)]
enum CameraOutput {
    Scaffold,
    ScaffoldCrossing,
    EmptySpace,
    NewLine,
    RobotUp,
    RobotDown,
    RobotLeft,
    RobotRight,
    RobotFallenOff,
}
impl From<isize> for CameraOutput {
    fn from(i: isize) -> Self {
        if i <= u8::MAX as isize {
            let c = i as u8 as char;
            match c {
                '#' => CameraOutput::Scaffold,       // #
                '.' => CameraOutput::EmptySpace,     // .
                '\n' => CameraOutput::NewLine,       // \n (line feed)
                '^' => CameraOutput::RobotUp,        // ^
                'v' => CameraOutput::RobotDown,      // v
                '<' => CameraOutput::RobotLeft,      // <
                '>' => CameraOutput::RobotRight,     // >
                'X' => CameraOutput::RobotFallenOff, // X
                _ => panic!("Unknown output '{}'", c),
            }
        } else {
            panic!("Output value '{}' is too large for a char", i);
        }
    }
}
struct ASCII {
    icc: IntCodeComputer,
    camera_output: HashMap<Point2D, CameraOutput>,
}
impl ASCII {
    fn new() -> Self {
        let input = day_17_puzzle_input();
        let icc = IntCodeComputer::new(input);
        let camera_output = HashMap::new();

        let mut ascii = ASCII { icc, camera_output };
        ascii.get_camera_output();
        ascii.mark_intersections();
        ascii.wake_robot_and_add_inputs();
        ascii
    }
    fn get_camera_output(&mut self) {
        let mut x = 0;
        let mut y = 0;
        let mut line = vec![];
        self.icc.run_until_halted();
        for output in self.icc.outputs() {
            line.push(output as u8 as char);
            let out = CameraOutput::from(output /*as u8 as char*/);
            if out == CameraOutput::NewLine {
                y += 1;
                x = 0;
                print!("{}", line.into_iter().collect::<String>());
                line = vec![];
            } else {
                self.camera_output.insert(Point2D::new(x, 34 - y), out);
                x += 1;
            }
        }
    }
    fn mark_intersections(&mut self) {
        let mut sum = 0;
        for y in 1..34 {
            //            println!("y = {}, x's:", y);
            for x in 1..58 {
                //                println!("{}", x);
                let center = &Point2D::new(x, y);
                if self.is_scaffold(center)
                    && center.neighbors().iter().all(|p| self.is_scaffold(p))
                {
                    self.camera_output
                        .insert(*center, CameraOutput::ScaffoldCrossing);
                    //                    println!("crossing ({}, {}), sum = {}", x, y, sum);
                    sum += y * x;
                }
            }
        }
        println!("alignment_parameter_sum = {}", sum);
    }
    fn wake_robot_and_add_inputs(&mut self) {
        println!("Creating new int code computer with woken up robot");
        let mut input = day_17_puzzle_input();
        input[0] = 2;
        self.icc = IntCodeComputer::new(input);

        // R,8,L,12,R,8,        :A
        // R,8,L,12,R,8,        :A
        // L,10,L,10,R,8,       :B
        // L,12,L,12,L,10,R,10, :C
        // L,10,L,10,R,8,       :B
        // L,12,L,12,L,10,R,10, :C
        // L,10,L,10,R,8,       :B
        // R,8,L,12,R,8,        :A
        // L,12,L,12,L,10,R,10, :C
        // R,8,L,12,R,8         :A
        let main = "A,A,B,C,B,C,B,A,C,A\n";
        let fn_a = "R,8,L,12,R,8\n";
        let fn_b = "L,10,L,10,R,8\n";
        let fn_c = "L,12,L,12,L,10,R,10\n";
        let feed = "n\n";

        self.icc.run_until_waiting_for_input();

        println!("Adding main");
        self.icc.add_inputs(&ASCII::codes(main));
        self.icc.run_until_waiting_for_input();

        self.icc.add_inputs(&ASCII::codes(fn_a));
        self.icc.run_until_waiting_for_input();

        self.icc.add_inputs(&ASCII::codes(fn_b));
        self.icc.run_until_waiting_for_input();

        self.icc.add_inputs(&ASCII::codes(fn_c));
        self.icc.run_until_waiting_for_input();

        self.icc.add_inputs(&ASCII::codes(feed));
    }
    fn is_scaffold(&self, p: &Point2D) -> bool {
        if let Some(&CameraOutput::Scaffold) = self.camera_output.get(p) {
            true
        } else {
            false
        }
    }
    fn codes(s: &str) -> Vec<isize> {
        s.chars().map(|c| ASCII::code(c) as isize).collect()
    }
    fn code(c: char) -> u8 {
        c as u8
    }
}

impl Game for ASCII {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<ASCII> {
        // Load your game assets here. Check out the `load` module!
        Task::new(|| ASCII::new())
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        let ct = CoordinateTransformation::from_points(
            frame.width(),
            frame.height(),
            &self.camera_output.keys().collect::<Vec<&Point2D>>(),
        );
        // println!("{:?}", ct);

        frame.clear(GREY);
        let mut mesh = Mesh::new();

        // empty space
        self.camera_output
            .iter()
            .filter(|(_, state)| state == &&CameraOutput::EmptySpace)
            .for_each(|(pos, _)| {
                mesh.fill(ct.square_at(pos), Color::BLACK);
            });

        // robot and scaffolding
        for (pos, state) in self.camera_output.iter() {
            match state {
                CameraOutput::Scaffold => mesh.stroke(ct.square_at(pos), Color::WHITE, 4),
                CameraOutput::ScaffoldCrossing => mesh.fill(ct.square_at(pos), GREEN),
                CameraOutput::RobotDown => {
                    mesh.fill(ct.robot_at(pos, &MovementCommand::South), RED)
                }
                CameraOutput::RobotUp => mesh.fill(ct.robot_at(pos, &MovementCommand::North), RED),
                CameraOutput::RobotLeft => mesh.fill(ct.robot_at(pos, &MovementCommand::West), RED),
                CameraOutput::RobotRight => {
                    mesh.fill(ct.robot_at(pos, &MovementCommand::East), RED)
                }
                CameraOutput::RobotFallenOff => mesh.fill(ct.square_at(pos), YELLOW),
                CameraOutput::EmptySpace => {}
                CameraOutput::NewLine => {}
            }
        }
        // robot fallen off
        self.camera_output
            .iter()
            .filter(|(_, state)| state == &&CameraOutput::RobotFallenOff)
            .for_each(|(pos, _)| {
                mesh.fill(ct.square_at(pos), YELLOW);
            });

        // origin
        mesh.fill(ct.square_at(&Point2D::default()), BLUE);

        mesh.draw(&mut frame.as_target());
    }

    fn update(&mut self, _window: &Window) {
        self.get_camera_output();
    }
}

fn day_17_puzzle_input() -> Vec<isize> {
    vec![
        1, 330, 331, 332, 109, 3546, 1101, 0, 1182, 15, 1101, 1481, 0, 24, 1001, 0, 0, 570, 1006,
        570, 36, 102, 1, 571, 0, 1001, 570, -1, 570, 1001, 24, 1, 24, 1105, 1, 18, 1008, 571, 0,
        571, 1001, 15, 1, 15, 1008, 15, 1481, 570, 1006, 570, 14, 21102, 58, 1, 0, 1106, 0, 786,
        1006, 332, 62, 99, 21101, 0, 333, 1, 21101, 0, 73, 0, 1106, 0, 579, 1101, 0, 0, 572, 1101,
        0, 0, 573, 3, 574, 101, 1, 573, 573, 1007, 574, 65, 570, 1005, 570, 151, 107, 67, 574, 570,
        1005, 570, 151, 1001, 574, -64, 574, 1002, 574, -1, 574, 1001, 572, 1, 572, 1007, 572, 11,
        570, 1006, 570, 165, 101, 1182, 572, 127, 1002, 574, 1, 0, 3, 574, 101, 1, 573, 573, 1008,
        574, 10, 570, 1005, 570, 189, 1008, 574, 44, 570, 1006, 570, 158, 1105, 1, 81, 21102, 1,
        340, 1, 1106, 0, 177, 21102, 1, 477, 1, 1106, 0, 177, 21101, 0, 514, 1, 21102, 1, 176, 0,
        1105, 1, 579, 99, 21102, 1, 184, 0, 1106, 0, 579, 4, 574, 104, 10, 99, 1007, 573, 22, 570,
        1006, 570, 165, 102, 1, 572, 1182, 21102, 375, 1, 1, 21101, 211, 0, 0, 1106, 0, 579, 21101,
        1182, 11, 1, 21101, 0, 222, 0, 1106, 0, 979, 21102, 388, 1, 1, 21102, 1, 233, 0, 1106, 0,
        579, 21101, 1182, 22, 1, 21102, 1, 244, 0, 1106, 0, 979, 21101, 0, 401, 1, 21102, 255, 1,
        0, 1106, 0, 579, 21101, 1182, 33, 1, 21102, 266, 1, 0, 1105, 1, 979, 21102, 414, 1, 1,
        21102, 1, 277, 0, 1105, 1, 579, 3, 575, 1008, 575, 89, 570, 1008, 575, 121, 575, 1, 575,
        570, 575, 3, 574, 1008, 574, 10, 570, 1006, 570, 291, 104, 10, 21102, 1, 1182, 1, 21102, 1,
        313, 0, 1105, 1, 622, 1005, 575, 327, 1102, 1, 1, 575, 21101, 0, 327, 0, 1106, 0, 786, 4,
        438, 99, 0, 1, 1, 6, 77, 97, 105, 110, 58, 10, 33, 10, 69, 120, 112, 101, 99, 116, 101,
        100, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 110, 97, 109, 101, 32, 98, 117, 116,
        32, 103, 111, 116, 58, 32, 0, 12, 70, 117, 110, 99, 116, 105, 111, 110, 32, 65, 58, 10, 12,
        70, 117, 110, 99, 116, 105, 111, 110, 32, 66, 58, 10, 12, 70, 117, 110, 99, 116, 105, 111,
        110, 32, 67, 58, 10, 23, 67, 111, 110, 116, 105, 110, 117, 111, 117, 115, 32, 118, 105,
        100, 101, 111, 32, 102, 101, 101, 100, 63, 10, 0, 37, 10, 69, 120, 112, 101, 99, 116, 101,
        100, 32, 82, 44, 32, 76, 44, 32, 111, 114, 32, 100, 105, 115, 116, 97, 110, 99, 101, 32,
        98, 117, 116, 32, 103, 111, 116, 58, 32, 36, 10, 69, 120, 112, 101, 99, 116, 101, 100, 32,
        99, 111, 109, 109, 97, 32, 111, 114, 32, 110, 101, 119, 108, 105, 110, 101, 32, 98, 117,
        116, 32, 103, 111, 116, 58, 32, 43, 10, 68, 101, 102, 105, 110, 105, 116, 105, 111, 110,
        115, 32, 109, 97, 121, 32, 98, 101, 32, 97, 116, 32, 109, 111, 115, 116, 32, 50, 48, 32,
        99, 104, 97, 114, 97, 99, 116, 101, 114, 115, 33, 10, 94, 62, 118, 60, 0, 1, 0, -1, -1, 0,
        1, 0, 0, 0, 0, 0, 0, 1, 12, 18, 0, 109, 4, 2102, 1, -3, 587, 20101, 0, 0, -1, 22101, 1, -3,
        -3, 21101, 0, 0, -2, 2208, -2, -1, 570, 1005, 570, 617, 2201, -3, -2, 609, 4, 0, 21201, -2,
        1, -2, 1106, 0, 597, 109, -4, 2106, 0, 0, 109, 5, 2102, 1, -4, 630, 20102, 1, 0, -2, 22101,
        1, -4, -4, 21101, 0, 0, -3, 2208, -3, -2, 570, 1005, 570, 781, 2201, -4, -3, 653, 20102, 1,
        0, -1, 1208, -1, -4, 570, 1005, 570, 709, 1208, -1, -5, 570, 1005, 570, 734, 1207, -1, 0,
        570, 1005, 570, 759, 1206, -1, 774, 1001, 578, 562, 684, 1, 0, 576, 576, 1001, 578, 566,
        692, 1, 0, 577, 577, 21101, 0, 702, 0, 1105, 1, 786, 21201, -1, -1, -1, 1106, 0, 676, 1001,
        578, 1, 578, 1008, 578, 4, 570, 1006, 570, 724, 1001, 578, -4, 578, 21102, 731, 1, 0, 1105,
        1, 786, 1106, 0, 774, 1001, 578, -1, 578, 1008, 578, -1, 570, 1006, 570, 749, 1001, 578, 4,
        578, 21102, 1, 756, 0, 1105, 1, 786, 1105, 1, 774, 21202, -1, -11, 1, 22101, 1182, 1, 1,
        21101, 0, 774, 0, 1106, 0, 622, 21201, -3, 1, -3, 1106, 0, 640, 109, -5, 2106, 0, 0, 109,
        7, 1005, 575, 802, 21001, 576, 0, -6, 20102, 1, 577, -5, 1106, 0, 814, 21102, 1, 0, -1,
        21102, 0, 1, -5, 21102, 0, 1, -6, 20208, -6, 576, -2, 208, -5, 577, 570, 22002, 570, -2,
        -2, 21202, -5, 59, -3, 22201, -6, -3, -3, 22101, 1481, -3, -3, 2101, 0, -3, 843, 1005, 0,
        863, 21202, -2, 42, -4, 22101, 46, -4, -4, 1206, -2, 924, 21102, 1, 1, -1, 1105, 1, 924,
        1205, -2, 873, 21102, 35, 1, -4, 1105, 1, 924, 2101, 0, -3, 878, 1008, 0, 1, 570, 1006,
        570, 916, 1001, 374, 1, 374, 1202, -3, 1, 895, 1101, 0, 2, 0, 2101, 0, -3, 902, 1001, 438,
        0, 438, 2202, -6, -5, 570, 1, 570, 374, 570, 1, 570, 438, 438, 1001, 578, 558, 921, 21002,
        0, 1, -4, 1006, 575, 959, 204, -4, 22101, 1, -6, -6, 1208, -6, 59, 570, 1006, 570, 814,
        104, 10, 22101, 1, -5, -5, 1208, -5, 35, 570, 1006, 570, 810, 104, 10, 1206, -1, 974, 99,
        1206, -1, 974, 1101, 0, 1, 575, 21102, 973, 1, 0, 1105, 1, 786, 99, 109, -7, 2105, 1, 0,
        109, 6, 21101, 0, 0, -4, 21102, 0, 1, -3, 203, -2, 22101, 1, -3, -3, 21208, -2, 82, -1,
        1205, -1, 1030, 21208, -2, 76, -1, 1205, -1, 1037, 21207, -2, 48, -1, 1205, -1, 1124,
        22107, 57, -2, -1, 1205, -1, 1124, 21201, -2, -48, -2, 1106, 0, 1041, 21102, 1, -4, -2,
        1106, 0, 1041, 21101, 0, -5, -2, 21201, -4, 1, -4, 21207, -4, 11, -1, 1206, -1, 1138, 2201,
        -5, -4, 1059, 1202, -2, 1, 0, 203, -2, 22101, 1, -3, -3, 21207, -2, 48, -1, 1205, -1, 1107,
        22107, 57, -2, -1, 1205, -1, 1107, 21201, -2, -48, -2, 2201, -5, -4, 1090, 20102, 10, 0,
        -1, 22201, -2, -1, -2, 2201, -5, -4, 1103, 1202, -2, 1, 0, 1105, 1, 1060, 21208, -2, 10,
        -1, 1205, -1, 1162, 21208, -2, 44, -1, 1206, -1, 1131, 1105, 1, 989, 21101, 0, 439, 1,
        1106, 0, 1150, 21102, 477, 1, 1, 1106, 0, 1150, 21101, 0, 514, 1, 21102, 1, 1149, 0, 1105,
        1, 579, 99, 21101, 0, 1157, 0, 1106, 0, 579, 204, -2, 104, 10, 99, 21207, -3, 22, -1, 1206,
        -1, 1138, 2101, 0, -5, 1176, 1201, -4, 0, 0, 109, -6, 2105, 1, 0, 6, 13, 27, 13, 6, 1, 11,
        1, 27, 1, 11, 1, 6, 1, 11, 1, 27, 1, 11, 1, 6, 1, 11, 1, 27, 1, 11, 1, 6, 1, 11, 1, 27, 1,
        11, 1, 6, 1, 11, 1, 27, 1, 11, 1, 6, 1, 11, 1, 1, 9, 9, 11, 9, 1, 6, 1, 11, 1, 1, 1, 7, 1,
        9, 1, 7, 1, 1, 1, 9, 1, 6, 1, 11, 13, 7, 1, 7, 1, 1, 1, 9, 1, 6, 1, 13, 1, 7, 1, 1, 1, 7,
        1, 7, 1, 1, 1, 9, 1, 6, 1, 13, 1, 7, 1, 1, 1, 5, 11, 1, 1, 9, 1, 6, 1, 13, 1, 7, 1, 1, 1,
        5, 1, 1, 1, 9, 1, 9, 1, 6, 11, 3, 1, 7, 1, 1, 1, 5, 1, 1, 1, 9, 1, 1, 9, 16, 1, 3, 1, 7, 1,
        1, 1, 5, 1, 1, 1, 9, 1, 1, 1, 24, 1, 3, 1, 7, 13, 7, 1, 1, 1, 24, 1, 3, 1, 9, 1, 5, 1, 1,
        1, 1, 1, 7, 1, 1, 1, 24, 1, 3, 1, 9, 9, 1, 1, 7, 11, 16, 1, 3, 1, 15, 1, 3, 1, 9, 1, 7, 1,
        12, 9, 15, 1, 3, 1, 9, 1, 7, 1, 16, 1, 19, 1, 3, 1, 9, 1, 7, 1, 16, 1, 19, 11, 3, 1, 7, 1,
        16, 1, 23, 1, 5, 1, 3, 1, 7, 1, 8, 9, 23, 11, 7, 1, 8, 1, 37, 1, 11, 1, 8, 1, 37, 1, 11, 1,
        8, 1, 37, 1, 11, 1, 8, 1, 37, 1, 11, 1, 8, 1, 37, 1, 11, 1, 8, 1, 37, 13, 8, 1, 58, 1, 58,
        1, 58, 1, 58, 1, 50, 9, 50,
    ]
}

#[derive(Debug)]
struct CoordinateTransformation {
    x_range: RangeInclusive<f32>,
    y_range: RangeInclusive<f32>,
    tile_size: f32,
}
impl CoordinateTransformation {
    fn from_points(width: f32, height: f32, points: &[&Point2D]) -> Self {
        let min_x = points.iter().map(|p| p.x()).min().unwrap() as f32;
        let max_x = points.iter().map(|p| p.x()).max().unwrap() as f32;
        let min_y = points.iter().map(|p| p.y()).min().unwrap() as f32;
        let max_y = points.iter().map(|p| p.y()).max().unwrap() as f32;
        let x_range = min_x..=max_x;
        let y_range = min_y..=max_y;

        let w_count = (max_x - min_x) + 1.0;
        let h_count = (max_y - min_y) + 1.0;
        let ppp = f32::min(width / w_count, height / h_count); // pixels-per-point

        //        println!(
        //            "Board: w: {}, h: {}, x: {} to {}, y: {} to {}, ppd: {}",
        //            w_count, h_count, min_x, max_x, min_y, max_y, ppp
        //        );

        CoordinateTransformation {
            x_range,
            y_range,
            tile_size: ppp,
        }
    }
    fn square_at(&self, pos: &Point2D) -> Shape {
        let top_left = self.point_at(pos);
        Shape::Rectangle(Rectangle {
            x: top_left.x,
            y: top_left.y,
            width: self.tile_size,
            height: self.tile_size,
        })
    }
    fn point_at(&self, pos: &Point2D) -> Point {
        Point::new(
            self.tile_size * (pos.x() as f32 - *self.x_range.start()),
            self.tile_size * (self.y_range.end() - pos.y() as f32),
        )
    }
    fn robot_at(&self, pos: &Point2D, dir: &MovementCommand) -> Shape {
        let top_left = self.point_at(pos);
        let top_right = self.point_at(&pos.offset_by(1, 0));
        let bottom_left = self.point_at(&pos.offset_by(0, -1));
        let bottom_right = self.point_at(&pos.offset_by(1, -1));
        let top = Point::new((top_left.x + top_right.x) / 2.0, top_left.y);
        let bottom = Point::new((bottom_left.x + bottom_right.x) / 2.0, bottom_left.y);
        let left = Point::new(top_left.x, (top_left.y + bottom_left.y) / 2.0);
        let right = Point::new(top_right.x, (top_right.y + bottom_right.y) / 2.0);

        let points = match dir {
            MovementCommand::North => vec![top, left, bottom_left, bottom_right, right],
            MovementCommand::South => vec![bottom, left, top_left, top_right, right],
            MovementCommand::West => vec![left, top, top_right, bottom_right, bottom],
            MovementCommand::East => vec![right, top, top_left, bottom_left, bottom],
        };

        Shape::Polyline { points }
    }
}

#[derive(Debug)]
enum MovementCommand {
    North,
    South,
    West,
    East,
}
impl From<isize> for MovementCommand {
    fn from(cmd: isize) -> Self {
        match cmd {
            1 => MovementCommand::North,
            2 => MovementCommand::South,
            3 => MovementCommand::West,
            4 => MovementCommand::East,
            _ => panic!("Invalid movement command {}", cmd),
        }
    }
}

mod tests {
    use crate::ASCII;

    #[test]
    fn ascii_from_char() {
        assert_eq!(ASCII::code(','), 44);
        assert_eq!(ASCII::code('A'), 65);
        assert_eq!(ASCII::code('B'), 66);
        assert_eq!(ASCII::code('C'), 67);
        assert_eq!(ASCII::code('L'), 76);
        assert_eq!(ASCII::code('R'), 82);
        assert_eq!(ASCII::code('8'), 56);
        assert_eq!(ASCII::code('\n'), 10);
    }
}
