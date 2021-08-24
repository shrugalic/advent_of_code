use intcode::IntCodeComputer;
use std::collections::HashMap;

// Coffee example

use coffee::graphics::{Color, Frame, Mesh, Point, Rectangle, Shape, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};
use std::ops::RangeInclusive;

const DARK_RED: Color = Color {
    r: 0.5,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};
const RED: Color = Color {
    r: 1.0,
    g: 0.0,
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
const LIGHT_BLUE: Color = Color {
    r: 0.5,
    g: 0.5,
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
    RepairDroid::run(WindowSettings {
        title: String::from("Advent of code 2019 day 15"),
        size: (1200, 1200),
        resizable: true,
        fullscreen: false,
    })
}

impl Game for RepairDroid {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<RepairDroid> {
        // Load your game assets here. Check out the `load` module!
        Task::new(|| RepairDroid::new(day_15_puzzle_input()))
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        let ct = CoordinateTransformation::from_points(
            frame.width(),
            frame.height(),
            &self.board_state.keys().collect::<Vec<&Point2D>>(),
        );

        frame.clear(GREY);
        let mut mesh = Mesh::new();
        for (pos, state) in self.board_state.iter() {
            match state {
                BoardState::Clear => mesh.fill(ct.square_at(pos), Color::WHITE),
                BoardState::Wall => mesh.fill(ct.square_at(pos), Color::BLACK),
                BoardState::OxygenSystem => mesh.fill(ct.square_at(pos), BLUE),
                BoardState::Unexplored => (),
                BoardState::ShortestPath => mesh.fill(ct.square_at(pos), RED),
                BoardState::Searched => mesh.fill(ct.square_at(pos), DARK_RED),
                BoardState::Oxygenated => mesh.fill(ct.square_at(pos), LIGHT_BLUE),
                BoardState::OxygenatedPath => mesh.fill(ct.square_at(pos), BLUE),
            }
        }
        let origin = ct.square_at(&Point2D::default());
        mesh.fill(origin, GREEN);

        if let Some(oxygen_system) = self.goal_pos {
            let oxygen_system = ct.square_at(&oxygen_system);
            mesh.fill(oxygen_system, BLUE);
        }

        if self.droid_state == DroidState::Exploring {
            // draw robot while exploring
            let robot = ct.robot_at(&self.curr_pos, &self.dir);
            mesh.fill(robot, RED);
        }
        mesh.draw(&mut frame.as_target());
    }

    fn update(&mut self, _window: &Window) {
        match self.droid_state {
            DroidState::Exploring => self.explore(),
            DroidState::FullyExplored
            | DroidState::PathFinding
            | DroidState::ShortestPathFound
            | DroidState::Oxygenating => self.find_shortest_path(),
            DroidState::FullyOxygenated => (), // done
        }
    }
}
struct CoordinateTransformation {
    x_range: RangeInclusive<f32>,
    y_range: RangeInclusive<f32>,
    tile_size: f32,
}
impl CoordinateTransformation {
    fn from_points(w: f32, h: f32, points: &[&Point2D]) -> Self {
        let min_x = points.iter().map(|p| p.0).min().unwrap() as f32;
        let max_x = points.iter().map(|p| p.0).max().unwrap() as f32;
        let min_y = points.iter().map(|p| p.1).min().unwrap() as f32;
        let max_y = points.iter().map(|p| p.1).max().unwrap() as f32;
        let x_range = min_x..=max_x;
        let y_range = min_y..=max_y;

        let board_width = (max_x - min_x) + 1.0;
        let board_height = (max_y - min_y) + 1.0;
        let tile_size = f32::min(w / board_width, h / board_height);
        /*
        println!(
            "Board: w: {}, h: {}, x: {} to {}, y: {} to {}, tile_size: {}",
            board_width, board_height, min_x, max_x, min_y, max_y, tile_size
        );
        */
        CoordinateTransformation {
            x_range,
            y_range,
            tile_size,
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
            self.tile_size * (pos.0 as f32 - *self.x_range.start()),
            self.tile_size * (self.y_range.end() - pos.1 as f32),
        )
    }
    fn robot_at(&self, pos: &Point2D, dir: &MovementCommand) -> Shape {
        let top_left = self.point_at(pos);
        let top_right = self.point_at(&pos.offset_by(1, 0));
        let bottom_left = self.point_at(&pos.offset_by(0, 1));
        let bottom_right = self.point_at(&pos.offset_by(1, 1));
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

// day 15

#[derive(Debug, Clone, PartialEq)]
enum PathFinderState {
    SearchingOxygenSystem,
    FoundOxygenSystem,
    Oxygenating,
}

#[derive(Debug, Clone)]
struct PathFinder {
    path: Vec<Point2D>,
    board_state: HashMap<Point2D, BoardState>,
    from_origin: bool,
    state: PathFinderState,
}
impl PathFinder {
    fn oxygen_system_finder(start: Point2D, board: &HashMap<Point2D, BoardState>) -> Self {
        PathFinder::new(start, PathFinderState::SearchingOxygenSystem, board)
    }
    fn oxygenator(start: Point2D, board: &HashMap<Point2D, BoardState>) -> Self {
        PathFinder::new(start, PathFinderState::Oxygenating, board)
    }
    fn new(start: Point2D, state: PathFinderState, board: &HashMap<Point2D, BoardState>) -> Self {
        PathFinder {
            path: vec![start],
            board_state: board.clone(),
            from_origin: false,
            state,
        }
    }
    fn curr_pos(&self) -> Point2D {
        *self.path.last().unwrap()
    }
    fn found_oxygen_system(&self, pos: &Point2D) -> bool {
        self.state == PathFinderState::SearchingOxygenSystem
            && self.board_state.get(pos).unwrap() == &BoardState::OxygenSystem
    }
    /// Return zero to three starting points for next path finders
    fn find_path(&mut self) -> Vec<Point2D> {
        let curr_pos = self.curr_pos();
        if self.found_oxygen_system(&curr_pos) {
            self.state = PathFinderState::FoundOxygenSystem;
            return vec![];
        } else if self.state == PathFinderState::SearchingOxygenSystem {
            self.board_state.insert(curr_pos, BoardState::Searched);
        } else {
            assert_eq!(self.state, PathFinderState::Oxygenating);
            self.board_state.insert(curr_pos, BoardState::Oxygenated);
        }

        // Explore possibilities
        let next_starting_points: Vec<Point2D> = curr_pos
            .neighbors()
            .into_iter()
            .filter(|possible_pos| self.is_worth_going_to(possible_pos))
            .collect();

        next_starting_points
    }
    fn is_worth_going_to(&self, maybe_next_pos: &Point2D) -> bool {
        if *maybe_next_pos == self.curr_pos() {
            false
        } else {
            let board = self.board_state.get(maybe_next_pos).unwrap();
            if self.state == PathFinderState::SearchingOxygenSystem {
                board.needs_path_finding()
            } else {
                assert_eq!(self.state, PathFinderState::Oxygenating);
                board.needs_oxygen()
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum DroidState {
    Exploring,
    FullyExplored,
    PathFinding,
    ShortestPathFound,
    Oxygenating,
    FullyOxygenated,
}

#[derive(Debug)]
struct RepairDroid {
    icc: IntCodeComputer,
    curr_pos: Point2D,
    dir: MovementCommand,
    board_state: HashMap<Point2D, BoardState>,
    droid_state: DroidState,
    goal_pos: Option<Point2D>,
    pathfinders: Vec<PathFinder>,
    counter: usize,
}
impl RepairDroid {
    fn new(input: Vec<isize>) -> Self {
        let icc = IntCodeComputer::new(input);
        let curr_pos = Point2D::default();
        let board_state = [(curr_pos, BoardState::Clear)].iter().cloned().collect();
        RepairDroid {
            icc,
            curr_pos,
            dir: MovementCommand::North,
            board_state,
            droid_state: DroidState::Exploring,
            goal_pos: None,
            pathfinders: vec![],
            counter: 0,
        }
    }
}
impl RepairDroid {
    fn find_shortest_path(&mut self) {
        self.counter += 1;
        if self.counter > 500 {
            println!("Reached counter of {}", self.counter);
            return;
        };
        if self.droid_state == DroidState::FullyExplored {
            // start finding shortest path
            let origin = Point2D::default();
            //            self.board_state.insert(origin, BoardState::Searched);
            self.pathfinders
                .push(PathFinder::oxygen_system_finder(origin, &self.board_state));
            self.droid_state = DroidState::PathFinding;
        }
        if self.droid_state == DroidState::ShortestPathFound {
            // start oxygenation
            self.counter = 0;
            let oxygen_pos = self.goal_pos.unwrap();
            assert_eq!(
                self.board_state.get(&oxygen_pos).unwrap(),
                &BoardState::OxygenSystem
            );
            self.pathfinders
                .push(PathFinder::oxygenator(oxygen_pos, &self.board_state));
            self.droid_state = DroidState::Oxygenating;
        }
        if self.droid_state == DroidState::PathFinding
            || self.droid_state == DroidState::Oxygenating
        {
            let mut spawns: Vec<PathFinder> = vec![];
            let mut updated_board_states: HashMap<Point2D, BoardState> = HashMap::new();
            self.pathfinders.drain(0..).for_each(|mut pf| {
                let mut next_starting_points = pf.find_path();
                match pf.state {
                    PathFinderState::SearchingOxygenSystem => {
                        updated_board_states.insert(pf.curr_pos(), BoardState::Searched);
                    }
                    PathFinderState::FoundOxygenSystem => {
                        println!("Shortest path has length {}", pf.path.len() - 1);
                        pf.path.iter().rev().skip(1).for_each(|pos| {
                            updated_board_states.insert(*pos, BoardState::ShortestPath);
                        });
                    }
                    PathFinderState::Oxygenating => {
                        updated_board_states.insert(pf.curr_pos(), BoardState::Oxygenated);
                    }
                }
                next_starting_points.drain(0..).for_each(|next_pos| {
                    let mut fork = pf.clone();
                    fork.path.push(next_pos);
                    spawns.push(fork)
                });
            });
            updated_board_states.into_iter().for_each(|(pos, state)| {
                if self.board_state.get(&pos) == Some(&BoardState::ShortestPath)
                    && state == BoardState::Oxygenated
                {
                    self.board_state.insert(pos, BoardState::OxygenatedPath);
                } else {
                    self.board_state.insert(pos, state);
                }
            });
            assert!(self.pathfinders.is_empty());
            self.pathfinders.append(&mut spawns);
            if self.pathfinders.is_empty() {
                if self.droid_state == DroidState::PathFinding {
                    self.droid_state = DroidState::ShortestPathFound;
                } else {
                    // oxygenating
                    assert_eq!(self.droid_state, DroidState::Oxygenating);
                    self.droid_state = DroidState::FullyOxygenated;
                    println!("Oxygenation took {} ticks", self.counter);
                }
            }
        }
    }
    #[allow(unused)]
    fn explore_full_maze(&mut self) {
        while self.droid_state == DroidState::Exploring {
            self.explore();
        }
    }
    fn explore(&mut self) {
        if let Some(output) = self
            .icc
            .process_int_code_until_first_output(self.dir.to_input())
        {
            let status = StatusCode::from(output);
            match status {
                StatusCode::HitWall => {
                    let pos = self.curr_pos.offset_by_1_into(&self.dir);
                    // println!(" Wall at {:?}", pos);
                    self.set_state_at_pos(pos, BoardState::Wall);
                    self.turn_clockwise();
                }
                StatusCode::Moved => {
                    self.curr_pos.record_movement(&self.dir);
                    self.set_state(BoardState::Clear);
                    self.follow_left_wall();
                    if self.curr_pos == Point2D::default() {
                        self.droid_state = DroidState::FullyExplored;
                    }
                }
                StatusCode::ReachedTarget => {
                    self.curr_pos.record_movement(&self.dir);
                    self.set_state(BoardState::OxygenSystem);
                    self.goal_pos = Some(self.curr_pos);
                }
            }
        }
    }
    fn follow_left_wall(&mut self) {
        // if there is a wall to the left, follow it
        if !self.is_wall_to_the_left() {
            self.turn_counter_clockwise();
        }
    }
    #[allow(unused)]
    fn state_at_current_pos(&self) -> &BoardState {
        self.board_state.get(&self.curr_pos).unwrap()
    }
    fn set_state_at_pos(&mut self, pos: Point2D, state: BoardState) {
        self.board_state.entry(pos).or_insert(state);
    }
    fn set_state(&mut self, state: BoardState) {
        self.set_state_at_pos(self.curr_pos, state);
    }
    fn turn_clockwise(&mut self) {
        self.dir = self.dir.clockwise();
    }
    fn turn_counter_clockwise(&mut self) {
        self.dir = self.dir.counter_clockwise();
    }
    fn is_wall_to_the_left(&self) -> bool {
        self.left_state() == &BoardState::Wall
    }
    fn left_state(&self) -> &BoardState {
        self.state_in(&self.dir.counter_clockwise())
    }
    /*
    fn is_wall_to_the_right(&self) -> bool {
        self.right_state() == &BoardState::Wall
    }
    fn front_state(&self) -> &BoardState {
        self.state_in(&self.dir)
    }
    fn right_state(&self) -> &BoardState {
        self.state_in(&self.dir.clockwise())
    }
    fn behind_state(&self) -> &BoardState {
        self.state_in(&self.dir.opposite())
    }
    */
    fn state_in(&self, dir: &MovementCommand) -> &BoardState {
        let pos = self.curr_pos.offset_by_1_into(dir);
        self.board_state
            .get(&pos)
            .unwrap_or(&BoardState::Unexplored)
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
impl MovementCommand {
    fn to_input(&self) -> isize {
        match self {
            MovementCommand::North => 1,
            MovementCommand::South => 2,
            MovementCommand::West => 3,
            MovementCommand::East => 4,
        }
    }
    fn clockwise(&self) -> MovementCommand {
        match self {
            MovementCommand::North => MovementCommand::East,
            MovementCommand::South => MovementCommand::West,
            MovementCommand::West => MovementCommand::North,
            MovementCommand::East => MovementCommand::South,
        }
    }
    fn counter_clockwise(&self) -> MovementCommand {
        match self {
            MovementCommand::North => MovementCommand::West,
            MovementCommand::South => MovementCommand::East,
            MovementCommand::West => MovementCommand::South,
            MovementCommand::East => MovementCommand::North,
        }
    }
    /*
    fn opposite(&self) -> MovementCommand {
        match self {
            MovementCommand::North => MovementCommand::South,
            MovementCommand::South => MovementCommand::North,
            MovementCommand::West => MovementCommand::East,
            MovementCommand::East => MovementCommand::West,
        }
    }
    */
}

#[derive(PartialEq, Debug, Clone)]
enum StatusCode {
    HitWall,
    Moved,
    ReachedTarget,
}
impl From<isize> for StatusCode {
    fn from(cmd: isize) -> Self {
        match cmd {
            0 => StatusCode::HitWall,
            1 => StatusCode::Moved,
            2 => StatusCode::ReachedTarget,
            _ => panic!("Invalid status code {}", cmd),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum BoardState {
    // phase 1: exploring
    Unexplored,
    Clear, // implies visited
    Wall,
    OxygenSystem,
    // phase 2: path finding
    Searched,
    ShortestPath,
    // phase 3: oxygenating
    Oxygenated,
    OxygenatedPath,
}
impl BoardState {
    fn needs_path_finding(&self) -> bool {
        match self {
            BoardState::Clear | BoardState::OxygenSystem => true,
            BoardState::Wall | BoardState::ShortestPath | BoardState::Searched => false,
            BoardState::Unexplored | BoardState::Oxygenated | BoardState::OxygenatedPath => {
                unreachable!()
            }
        }
    }
    fn needs_oxygen(&self) -> bool {
        match self {
            BoardState::ShortestPath | BoardState::Searched => true,
            BoardState::OxygenSystem
            | BoardState::Wall
            | BoardState::Oxygenated
            | BoardState::OxygenatedPath => false,
            BoardState::Unexplored | BoardState::Clear => unreachable!(),
        }
    }
}

fn day_15_puzzle_input() -> Vec<isize> {
    vec![
        3, 1033, 1008, 1033, 1, 1032, 1005, 1032, 31, 1008, 1033, 2, 1032, 1005, 1032, 58, 1008,
        1033, 3, 1032, 1005, 1032, 81, 1008, 1033, 4, 1032, 1005, 1032, 104, 99, 102, 1, 1034,
        1039, 1002, 1036, 1, 1041, 1001, 1035, -1, 1040, 1008, 1038, 0, 1043, 102, -1, 1043, 1032,
        1, 1037, 1032, 1042, 1106, 0, 124, 1002, 1034, 1, 1039, 102, 1, 1036, 1041, 1001, 1035, 1,
        1040, 1008, 1038, 0, 1043, 1, 1037, 1038, 1042, 1105, 1, 124, 1001, 1034, -1, 1039, 1008,
        1036, 0, 1041, 102, 1, 1035, 1040, 102, 1, 1038, 1043, 1002, 1037, 1, 1042, 1105, 1, 124,
        1001, 1034, 1, 1039, 1008, 1036, 0, 1041, 102, 1, 1035, 1040, 102, 1, 1038, 1043, 1002,
        1037, 1, 1042, 1006, 1039, 217, 1006, 1040, 217, 1008, 1039, 40, 1032, 1005, 1032, 217,
        1008, 1040, 40, 1032, 1005, 1032, 217, 1008, 1039, 33, 1032, 1006, 1032, 165, 1008, 1040,
        35, 1032, 1006, 1032, 165, 1101, 2, 0, 1044, 1106, 0, 224, 2, 1041, 1043, 1032, 1006, 1032,
        179, 1101, 1, 0, 1044, 1106, 0, 224, 1, 1041, 1043, 1032, 1006, 1032, 217, 1, 1042, 1043,
        1032, 1001, 1032, -1, 1032, 1002, 1032, 39, 1032, 1, 1032, 1039, 1032, 101, -1, 1032, 1032,
        101, 252, 1032, 211, 1007, 0, 37, 1044, 1105, 1, 224, 1102, 0, 1, 1044, 1105, 1, 224, 1006,
        1044, 247, 101, 0, 1039, 1034, 101, 0, 1040, 1035, 102, 1, 1041, 1036, 1001, 1043, 0, 1038,
        1002, 1042, 1, 1037, 4, 1044, 1105, 1, 0, 31, 10, 7, 30, 32, 67, 8, 24, 11, 62, 6, 11, 19,
        78, 16, 20, 8, 80, 14, 19, 63, 8, 40, 36, 65, 34, 59, 23, 33, 29, 79, 19, 47, 28, 54, 8,
        11, 41, 33, 57, 85, 25, 56, 48, 16, 90, 74, 39, 11, 79, 68, 18, 46, 33, 74, 47, 25, 60, 1,
        23, 78, 69, 5, 55, 12, 28, 73, 22, 80, 30, 26, 55, 2, 6, 96, 21, 57, 34, 33, 10, 91, 72,
        61, 31, 2, 24, 29, 94, 24, 12, 43, 60, 72, 79, 27, 24, 21, 95, 59, 15, 53, 34, 9, 36, 82,
        83, 4, 67, 30, 62, 5, 70, 94, 1, 81, 75, 6, 18, 68, 9, 26, 38, 31, 1, 98, 57, 97, 63, 8,
        60, 35, 5, 48, 36, 59, 75, 4, 88, 23, 21, 39, 10, 99, 13, 36, 53, 66, 73, 28, 33, 80, 28,
        78, 23, 7, 30, 27, 77, 28, 69, 69, 1, 65, 78, 17, 17, 2, 16, 27, 91, 43, 27, 72, 93, 6, 5,
        92, 12, 55, 79, 94, 98, 60, 19, 15, 36, 35, 55, 9, 62, 84, 27, 74, 56, 25, 9, 60, 72, 15,
        34, 59, 15, 31, 58, 76, 24, 81, 62, 99, 35, 31, 14, 39, 25, 60, 3, 5, 46, 24, 48, 22, 1,
        73, 99, 96, 27, 46, 48, 5, 65, 26, 6, 48, 11, 13, 69, 12, 33, 22, 95, 11, 72, 28, 42, 28,
        88, 5, 31, 56, 50, 72, 30, 49, 84, 52, 32, 11, 45, 7, 54, 60, 12, 72, 33, 38, 62, 18, 54,
        31, 8, 92, 53, 34, 4, 76, 21, 46, 81, 53, 81, 21, 10, 63, 12, 75, 22, 62, 87, 32, 23, 30,
        40, 29, 24, 61, 6, 88, 70, 14, 18, 99, 13, 14, 4, 72, 5, 22, 54, 90, 75, 35, 1, 10, 49, 17,
        7, 98, 8, 81, 13, 47, 59, 13, 80, 70, 9, 26, 73, 22, 77, 3, 22, 73, 99, 74, 11, 10, 60, 4,
        27, 86, 46, 67, 30, 94, 29, 93, 26, 66, 25, 8, 14, 92, 24, 45, 78, 24, 23, 97, 31, 9, 25,
        25, 61, 44, 35, 31, 73, 52, 80, 35, 96, 32, 43, 8, 66, 57, 87, 31, 85, 12, 50, 74, 7, 23,
        61, 12, 7, 78, 1, 1, 53, 14, 54, 18, 18, 63, 41, 25, 90, 1, 85, 24, 22, 98, 62, 35, 14, 19,
        50, 80, 20, 7, 73, 21, 14, 81, 19, 89, 11, 31, 84, 7, 53, 9, 54, 20, 90, 72, 31, 70, 54,
        17, 31, 59, 18, 8, 69, 83, 58, 78, 12, 98, 20, 81, 26, 50, 95, 19, 25, 54, 31, 80, 67, 6,
        3, 87, 6, 99, 93, 22, 75, 73, 34, 52, 58, 22, 32, 52, 34, 30, 85, 54, 58, 75, 14, 22, 97,
        12, 36, 53, 67, 32, 99, 54, 15, 4, 66, 69, 7, 48, 87, 25, 17, 41, 57, 10, 63, 35, 24, 43,
        5, 57, 25, 93, 22, 71, 7, 36, 63, 84, 26, 4, 7, 78, 26, 68, 77, 35, 9, 70, 17, 12, 59, 41,
        78, 18, 54, 18, 80, 18, 86, 93, 19, 35, 73, 34, 53, 97, 23, 2, 95, 30, 32, 85, 21, 21, 79,
        19, 18, 85, 57, 23, 85, 35, 34, 61, 30, 66, 29, 19, 76, 30, 17, 46, 1, 16, 98, 26, 25, 91,
        15, 47, 54, 75, 26, 17, 36, 74, 60, 33, 28, 49, 53, 15, 13, 45, 6, 90, 26, 73, 17, 87, 4,
        68, 18, 30, 22, 96, 92, 97, 14, 40, 24, 50, 96, 15, 49, 55, 79, 8, 16, 1, 50, 5, 60, 55,
        14, 41, 67, 25, 26, 71, 18, 26, 89, 70, 14, 6, 51, 11, 94, 68, 69, 22, 73, 63, 6, 33, 88,
        36, 51, 20, 6, 44, 26, 71, 17, 31, 11, 86, 81, 23, 31, 80, 18, 87, 26, 12, 91, 8, 41, 6,
        18, 9, 33, 90, 1, 59, 56, 32, 29, 54, 50, 34, 12, 74, 97, 10, 39, 87, 41, 9, 52, 67, 21,
        22, 38, 61, 57, 1, 87, 4, 35, 98, 61, 16, 95, 78, 65, 17, 31, 9, 71, 9, 52, 52, 9, 8, 73,
        40, 36, 16, 48, 52, 9, 26, 39, 4, 17, 42, 1, 35, 80, 93, 4, 40, 23, 13, 66, 7, 28, 84, 73,
        22, 31, 76, 31, 21, 39, 4, 83, 84, 41, 27, 66, 34, 88, 15, 50, 65, 45, 22, 65, 26, 78, 15,
        50, 40, 79, 31, 38, 9, 60, 2, 51, 24, 46, 99, 42, 27, 45, 1, 71, 20, 78, 86, 95, 9, 81, 0,
        0, 21, 21, 1, 10, 1, 0, 0, 0, 0, 0, 0,
    ]
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point2D(isize, isize);
impl Default for Point2D {
    fn default() -> Self {
        Point2D(0, 0)
    }
}
impl Point2D {
    fn record_movement(&mut self, direction: &MovementCommand) {
        match direction {
            MovementCommand::North => self.1 += 1,
            MovementCommand::South => self.1 -= 1,
            MovementCommand::West => self.0 -= 1,
            MovementCommand::East => self.0 += 1,
        }
    }
    fn offset_by_1_into(self, direction: &MovementCommand) -> Point2D {
        match direction {
            MovementCommand::North => Point2D(self.0, self.1 + 1),
            MovementCommand::South => Point2D(self.0, self.1 - 1),
            MovementCommand::West => Point2D(self.0 - 1, self.1),
            MovementCommand::East => Point2D(self.0 + 1, self.1),
        }
    }
    fn offset_by(self, x: isize, y: isize) -> Point2D {
        Point2D(self.0 + x, self.1 + y)
    }
    fn neighbors(&self) -> Vec<Point2D> {
        vec![
            self.offset_by_1_into(&MovementCommand::North),
            self.offset_by_1_into(&MovementCommand::East),
            self.offset_by_1_into(&MovementCommand::South),
            self.offset_by_1_into(&MovementCommand::West),
        ]
    }
}

mod tests {
    use super::*;

    #[test]
    fn day_15() {
        let mut droid = RepairDroid::new(day_15_puzzle_input());
        droid.explore_full_maze();
        assert_eq!(droid.state_at_current_pos(), &BoardState::OxygenSystem);
    }
}
