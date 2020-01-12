use coffee::graphics::{Color, Frame, Mesh, Point, Rectangle, Shape, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};
use std::fmt;
use std::fmt::{Display, Formatter};
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
    UndergroundVault::run(WindowSettings {
        title: String::from("Advent of code 2019 day 18"),
        size: (1280, 1280),
        resizable: true,
        fullscreen: false,
    })
}
impl Game for UndergroundVault {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<UndergroundVault> {
        // Load your game assets here. Check out the `load` module!
        Task::new(|| UndergroundVault::from(day_18_puzzle_input()))
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        let ct = CoordinateTransformation::from_points(
            frame.width(),
            frame.height(),
            &self.map.points(),
        );
        frame.clear(GREY);
        let mut mesh = Mesh::new();

        self.map
            .point_obj_pairs()
            .iter()
            .for_each(|(pos, obj)| match obj {
                Obj::Player => mesh.fill(ct.square_at(pos), YELLOW),
                Obj::Wall => mesh.fill(ct.square_at(pos), Color::BLACK),
                Obj::EmptySpace => mesh.fill(ct.square_at(pos), Color::WHITE),
                Obj::Key(_) => mesh.fill(ct.square_at(pos), GREEN),
                Obj::Door(_) => mesh.fill(ct.square_at(pos), RED),
            });

        mesh.draw(&mut frame.as_target());
    }

    fn update(&mut self, _window: &Window) {
        self.explore();
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

#[derive(Debug, PartialEq)]
struct CoordinateTransformation {
    x_range: RangeInclusive<f32>,
    y_range: RangeInclusive<f32>,
    tile_size: f32,
}
impl CoordinateTransformation {
    fn from_points(width: f32, height: f32, points: &[Vec2]) -> Self {
        let min_x = points.iter().map(|p| p.x).min().unwrap() as f32;
        let max_x = points.iter().map(|p| p.x).max().unwrap() as f32;
        let min_y = points.iter().map(|p| p.y).min().unwrap() as f32;
        let max_y = points.iter().map(|p| p.y).max().unwrap() as f32;
        let xr = min_x..=max_x;
        let yr = min_y..=max_y;

        let w_count = (max_x - min_x) + 1.0;
        let h_count = (max_y - min_y) + 1.0;
        let ppp = f32::min(width / w_count, height / h_count); // pixels-per-point

        // println!(
        // "Board: w: {}, h: {}, x: {} to {}, y: {} to {}, ppd: {}",
        // w_count,
        // h_count,
        // xr.start(),
        // xr.end(),
        // yr.start(),
        // yr.end(),
        // ppp
        // );

        CoordinateTransformation {
            x_range: xr,
            y_range: yr,
            tile_size: ppp,
        }
    }
    fn square_at(&self, pos: &Vec2) -> Shape {
        let top_left = self.point_at(pos);
        Shape::Rectangle(Rectangle {
            x: top_left.x,
            y: top_left.y,
            width: self.tile_size,
            height: self.tile_size,
        })
    }
    fn point_at(&self, pos: &Vec2) -> Point {
        Point::new(
            self.tile_size * (pos.x as f32 - *self.x_range.start()),
            self.tile_size * (pos.y as f32 - *self.y_range.start()),
        )
    }
}

// Day 18 itself

#[derive(PartialEq, Debug, Copy, Clone)]
enum Obj {
    Player,
    Wall,
    EmptySpace,
    Key(char),
    Door(char),
}
impl From<char> for Obj {
    fn from(c: char) -> Self {
        match c {
            '@' => Obj::Player,
            '#' => Obj::Wall,
            '.' => Obj::EmptySpace,
            'a'..='z' => Obj::Key(c),
            'A'..='Z' => Obj::Door(c),
            _ => panic!("Invalid object '{}'", c),
        }
    }
}
impl Obj {
    fn matching_key(&self) -> char {
        if let Obj::Door(door) = self {
            // assert_eq!(b'a' - b'A', 32);
            ((*door as u8) + 32) as char
        } else {
            ' '
        }
    }
    fn matching_door(&self) -> char {
        if let Obj::Key(key) = self {
            // assert_eq!(b'a' - b'A', 32);
            ((*key as u8) - 32) as char
        } else {
            ' '
        }
    }
    fn is_key(&self) -> bool {
        match self {
            Obj::Key(_) => true,
            Obj::Door(_) | Obj::Wall | Obj::Player | Obj::EmptySpace => false,
        }
    }
}
#[derive(PartialEq, Debug)]
struct Map {
    map: Vec<Vec<Obj>>,
    width: usize,
    height: usize,
}
impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let map: Vec<Vec<Obj>> = input
            .split('\n')
            // .inspect(|row| println!("{}", row))
            .map(|row| row.chars().map(|c| Obj::from(c)).collect())
            .collect();
        let width = map[0].len();
        let height = map.len();
        Map { map, width, height }
    }
}
impl Map {
    fn can_be_moved_onto(&self, pos: &Vec2, keys: &[char]) -> bool {
        match self.object_at(pos) {
            Obj::Player | Obj::EmptySpace | Obj::Key(_) => true,
            Obj::Wall => false,
            d @ Obj::Door(_) => keys.contains(&d.matching_key()),
        }
    }
    fn player_pos(&self) -> Vec2 {
        self.point_obj_pairs()
            .iter()
            .filter_map(|(pos, obj)| {
                if obj == &Obj::Player {
                    Some(pos.clone())
                } else {
                    None
                }
            })
            .last()
            .expect("Map contains no player object")
    }
    fn object_at(&self, pos: &Vec2) -> &Obj {
        &self.map[pos.y][pos.x]
    }
    fn key_count(&self) -> usize {
        self.map
            .iter()
            .map(|row| row.iter().filter(|obj| obj.is_key()).count())
            .sum()
    }
    fn includes(&self, pos: &Vec2) -> bool {
        pos.x < self.width && pos.y < self.height
    }
    fn point_obj_pairs(&self) -> Vec<(Vec2, Obj)> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, obj)| (Vec2 { x, y }, obj.clone()))
            })
            .collect()
    }
    fn points(&self) -> Vec<Vec2> {
        self.point_obj_pairs()
            .into_iter()
            .map(|(pos, _obj)| pos)
            .collect()
    }
}
#[derive(PartialEq, Debug)]
struct UndergroundVault {
    /// A map of the tunnels
    // map[y[x]] = object
    // Example:
    // x: 012345789
    // y: 0 [[#########],
    // y: 1  [#b.A.@.a#],
    // y: 2  [#########]]
    map: Map,
    explorer: Explorer,
}
impl From<&str> for UndergroundVault {
    fn from(input: &str) -> Self {
        let map = Map::from(input);
        let explorer = Explorer::initial(map.player_pos());
        UndergroundVault { map, explorer }
    }
}
impl From<Map> for UndergroundVault {
    fn from(map: Map) -> Self {
        let explorer = Explorer::initial(map.player_pos());
        UndergroundVault { map, explorer }
    }
}
impl Map {
    fn set(&mut self, pos: Vec2, obj: Obj) {
        self.map[pos.y][pos.x] = obj;
    }
}
impl UndergroundVault {
    fn move_to_keys_with(&self, start: &Explorer) -> Vec<Explorer> {
        let mut explorers_on_keys: Vec<Explorer> = vec![];
        let mut explorers: Vec<Explorer> = vec![start.clone()];
        while !explorers.is_empty() {
            // println!("explorers = {}", explorers.len());
            let mut next_explorers: Vec<Explorer> = vec![];
            explorers.drain(0..).for_each(|mut explorer| {
                if let Some(key) = explorer.new_key_at_current_pos(&self.map) {
                    // println!("New key {} at pos {:?}", key, pf.pos);
                    explorer.keys.push(key);
                    explorers_on_keys.push(explorer);
                } else {
                    self.possible_next_positions(&explorer)
                        .drain(0..)
                        .for_each(|next_explorer| next_explorers.push(next_explorer))
                }
            });
            explorers.extend(next_explorers);
        }
        explorers_on_keys
    }
    fn possible_next_positions(&self, explorer: &Explorer) -> Vec<Explorer> {
        explorer
            .reachable_positions(&self.map)
            .into_iter()
            .filter(|next_pos| self.map.includes(&next_pos) && !explorer.just_visited(&next_pos))
            .map(|next_pos| explorer.visit(next_pos))
            .collect()
    }
    fn explore(&mut self) {
        let mut explorers_on_keys = self.move_to_keys_with(&self.explorer);
        if !explorers_on_keys.is_empty() {
            self.explorer = explorers_on_keys.remove(0);

            // println!("{:?}", self.explorer);

            // Move player the to the key position, replacing the key
            let curr_path = self.explorer.curr_path();
            let player_pos = curr_path[0];
            // println!("player_pos = {:?}", player_pos);
            self.map.set(player_pos, Obj::EmptySpace);

            let key_pos = curr_path.last().unwrap();
            // println!("key_pos = {:?}", key_pos);

            let key = self.map.object_at(key_pos);
            // println!("key = {:?}", key);
            // remove the corresponding door
            if let Some((door_pos, _door)) = self
                .map
                .point_obj_pairs()
                .iter()
                .find(|(_pos, obj)| obj == &Obj::Door(key.matching_door()))
            {
                // println!("door_pos = {:?}", door_pos);
                self.map.set(*door_pos, Obj::EmptySpace);
            } else {
                // println!("door for key {:?} not found", key);
            }

            self.map.set(*key_pos, Obj::Player);

            self.explorer.paths.push(vec![*key_pos]); // Start of next leg
        } else {
            println!("All keys found");
        }
    }
    fn shortest_path_collecting_all_keys(&self) -> usize {
        let key_count = self.map.key_count();
        // println!("key count = {}", key_count);
        let mut finished_explorers = vec![];

        let mut explorers_on_keys = self.move_to_keys_with(&self.explorer);
        while !explorers_on_keys.is_empty() {
            let mut next_explorers = vec![];
            explorers_on_keys.drain(0..).for_each(|mut explorer| {
                // println!("{}/{} keys {:?}", explorer.keys.len(), key_count, explorer);
                if explorer.keys.len() == key_count {
                    finished_explorers.push(explorer);
                } else {
                    explorer
                        .paths
                        .push(vec![*explorer.curr_path().last().unwrap()]); // Start of next leg
                    let new_explorers = self.move_to_keys_with(&explorer);
                    // println!("New explorers = {:?}", new_explorers);
                    next_explorers.extend(new_explorers);
                }
            });
            explorers_on_keys = next_explorers;
        }
        println!("Finished with {} explorers", finished_explorers.len());
        let shortest = finished_explorers
            .iter()
            .min_by_key(|explorer| explorer.path_length())
            .unwrap();
        // println!("Shortest = {:?}", shortest);
        shortest.path_length() - shortest.paths.len() // Don't count the starting position for each leg
    }
    fn initial_explorer(&self) -> Explorer {
        Explorer::initial(self.map.player_pos())
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Explorer {
    pos: Vec2,        // Current position
    paths: Vec<Path>, // Key collection paths. Last one is current search path, may be incomplete
    keys: Vec<char>,  // Collected keys
}
impl Explorer {
    fn initial(pos: Vec2) -> Self {
        let paths = vec![vec![pos]];
        let keys = vec![];
        Explorer { pos, keys, paths }
    }
    fn curr_path(&self) -> Path {
        self.paths.last().unwrap().clone()
    }
    fn visit(&self, pos: Vec2) -> Explorer {
        let mut paths = self.paths.clone();
        paths.last_mut().unwrap().push(pos);
        let keys = self.keys.clone();
        Explorer { pos, paths, keys }
    }
    fn just_visited(&self, pos: &Vec2) -> bool {
        self.curr_path().contains(pos)
    }
    fn reachable_positions(&self, map: &Map) -> Vec<Vec2> {
        let mut dirs = vec![];
        let pos = &self.pos;
        let keys = &self.keys;
        if pos.x + 1 < map.width && map.can_be_moved_onto(&pos.offset_by(1, 0), keys) {
            dirs.push(Vec2::new(pos.x + 1, pos.y));
        }
        if pos.y + 1 < map.height && map.can_be_moved_onto(&pos.offset_by(0, 1), keys) {
            dirs.push(Vec2::new(pos.x, pos.y + 1));
        }
        if pos.x > 0 && map.can_be_moved_onto(&pos.offset_by(-1, 0), keys) {
            dirs.push(Vec2::new(pos.x - 1, pos.y));
        }
        if pos.y > 0 && map.can_be_moved_onto(&pos.offset_by(0, -1), keys) {
            dirs.push(Vec2::new(pos.x, pos.y - 1));
        }
        dirs
    }
    fn path_length(&self) -> usize {
        self.paths.iter().map(|path| path.len()).sum()
    }
    fn new_key_at_current_pos(&self, map: &Map) -> Option<char> {
        if let Obj::Key(key) = map.object_at(&Vec2::new(self.pos.x, self.pos.y)) {
            if self.keys.contains(&key) {
                None
            } else {
                Some(*key)
            }
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Vec2 {
    x: usize,
    y: usize,
}
impl Vec2 {
    fn new(x: usize, y: usize) -> Self {
        Vec2 { x, y }
    }
    fn offset_by(&self, x_offset: isize, y_offset: isize) -> Vec2 {
        Vec2::new(
            (self.x as isize + x_offset) as usize,
            (self.y as isize + y_offset) as usize,
        )
    }
}
impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
type Path = Vec<Vec2>;

fn day_18_example_1() -> &'static str {
    "#########
#b.A.@.a#
#########"
}

fn day_18_example_2() -> &'static str {
    "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"
}

fn day_18_example_3() -> &'static str {
    "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"
}

fn day_18_example_4() -> &'static str {
    "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################"
}

fn day_18_example_5() -> &'static str {
    "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################"
}

fn challenging_input() -> &'static str {
    "##########
#.a###.Ab#
#.B..@.###
#...######
##########"
}

fn day_18_puzzle_input() -> &'static str {
    "#################################################################################
#.....#........q..#...........#.........#.#...#.......#.#...#...#.....P...#.....#
###E#.#####.#######.###G#####.#########.#.#.#.#.###.#.#.#.#.#.#.#.#######.#.#.#.#
#...#.....#...#.....#.#.#i..#.#...J.....#...#.....#.#...#.#...#.#.......#...#.#.#
#.#######.###.#.#####.#.#.###.#.#######.#.#########.###.#.#####.#####.#######.#.#
#.......#.....#.#.....#.#...#...#.....#.#.#.....#...#...#.#...#.#...#.#.......#.#
#.#####.###.###.#.#.###.###.#########.#.#.###.#.#.#.#####.#.###.#.#.#.#.#########
#.....#.#.#.#...#.#...#...#.....#.....#.#...#.#.#.#t#.....#...#...#.#.#.#.......#
#####.#.#.#.#.#######.###.#.###.#.###.#.###.#.###.###.#######.#####.#.#.#.#####.#
#...#.#.#...#.........#...#.#.....#.#.#.#.#...#...#...#...........#.#.#.#.#...#.#
#.#.#.#.###.#########.#.###.#######.#.#.#.#.###.#.#.#######.#####.#.###.#.###.#.#
#.#p#.#...#...#...#...#.#.#.......#...#.#.#.#...#.#.....#...#...#.#.#...#.....#.#
#.#.#.###.###.#.###.###.#.#.#####.#.###.#.#.#.#########.#.###.#.###.#.#####.###.#
#.#...#...#...#...#.#...#...#.#...#.#...#...#.#.......#...#...#.....#.#...#.#...#
#####.#.###.#####.#.#.###.###.#.###.#.###.###.#.###.#####.#.#########.#.#.#.#.#.#
#...#.#r#...#.....#.#...#.....#.#...#...#.#.#.#.#.#.....#.#.#.......#...#...#.#.#
#.#.###.#.#####.#.#.###.#####.#.#.#####.#.#.#.#.#.###.#.#.#.###.###.#.#######.#.#
#.#.....#.#...#.#.#.#.#.....#.#.#...#...#...#...#...#.#.#.#...#.#...#...#.....#.#
#.#######.#.#.#.#.#.#.#####.###.###.#.#####.#####.#.#.#.#.###.#.#.#####.#.#####.#
#.....#.#...#...#.#.......#...#...#.#.#.#.#.#.....#.#.#.#.#.#.#.#.#.....#.#...#.#
#####.#.#####.#####.#####.###Y#.#.#.#.#.#.#.###.#.#.#.###.#.#.#.###.#####.#.#.#.#
#.....#.#.....#...#.#...#...#.#.#.#.#.#.#.#...#.#.#.#...#...#.#.#...#...#.#.#.#.#
#.#####.#.#####.#.#.###.#.###.#.#.###.#.#.###.###.#.###.###.###.#.#####.#.###.#.#
#...#.......#.#.#.....#.#.#...#.#...#...#...#..b..#.#.#.#...#...........#...#...#
#.#.#######.#.#.#######.###.#######.###.#.#########.#.#.#.#####.#######.###.###.#
#.#.#...#.D...#v#.....#...#.#.....#.....#.#.......#...#.#.#...#.#...#.....#...#.#
#.#.#L#.#######.#.###.###.#.#.###.#.#####.#.#.#######.#.#.#.#.#.#.#.#########.#.#
#.#...#.#.....#.#...#.....#.....#.#...#.#.#.#.........#...#.#.#.#.#.#.......#.#.#
#.#####.#.###.#.###.#############.###.#.#.#.###############.#.###.#.#.#####.#.#.#
#.#...#...#.#...#.#.........#.....#...#.#.#.#.#.......#.....#...#.#.#.#.....V.#.#
#.#.#.#####.#####.#########.#.#######.#.#.#.#.#.###X#.#.#.#####.#.#.#.#########.#
#...#.#...#.........#.....#.#.......#...#...#.#...#.#.#.#.#...#.#.#.#.....#.R.#.#
#####.#.###.#.#####.#.#####.#.#####.#####.###.###.#.#.###.#.###.#.#.#.###.#.###.#
#...#.#.....#.....#.#.....#.#.....#.....#.#.......#.#....y#.#...#.#.#.#...#.#...#
#.###.#.#########.#.#####.#.#####.#####.#.###.#####.#######.#.###.#.###.###.#.###
#.....#...#.......#.#.....#...#.....#.#.#...#.....#.#.....#...#...#...#.#.....#.#
#.#########.#######.#.#######.#####.#.#.###.#######.#.###.#.#####.###.#.#.#####.#
#.........#.#.#...#.#.......#.#...#...#.#...#.....#.#.#...#.#...#...#...#.....#.#
#########.#.#.#.#.#.#####.#.#.#.#.#####.#.###.###.#.#.#.###.#.#.###.#########.#.#
#...........#...#.........#.#...#.............#.....#.#.......#.....#...........#
#######################################.@.#######################################
#.........#.....#...#...........#.............#.......#.......#.............#...#
#.#.#######.#.###.#.###.#######.###.###.###.#.#.#####.#.#.###.#####.#####.#.#.#.#
#.#m#z......#.....#..o#.....#.......#...#...#...#.....#.#...#.#...#...#...#.#.#.#
#.#.#.###############.#####.#########.###.#########.#######.#.#.#.#.###.###.#.#.#
#.#.#.....#.........#.....#.#...#...#.#.#.........#.#.......#...#.#.#...#.#.#.#.#
#.#.#####.###.###.#.#####.#.#.#.#.#.#.#.#.#######.#.###.#########.###.###.#.###.#
#.#.#...#...#...#.#...#...#...#...#.#...#.....#...#.....#...#.....#...#...#...#.#
#.###.#.###.###.#.###.#.###########.###.#######.#########.###.#####.###.#.###.#.#
#.....#.....#...#.#...#...#.....W.#...#.#.......#.........#...#.....#...#...#...#
#.###########.###.#.#####.#.#####.###.#.#.#######.###.#####.###.#.###.###.#####.#
#.....#.........#.#...#...#.#.....#...#.#.#...#.....#.....#.#...#.#...#.#.....#.#
#####.###########.#.###.###.#.#####.###.#.#.#.#.#####.###.#.#####.#.###.#####.#.#
#...#.....#.......#.#...#...#..n#.....#x#.#.#.#.#...#...#.......#.#.#...#...#...#
#.#######.#.#######.#.#.#.#####.#####.#.#.#.#.###.#.###########.#.#.#.#.#.#.#####
#...........#c....#.#.#.#...#.#.#.....#.#...#.#...#...#.......#...#l..#...#.....#
#.###########.###.#.#.#.###.#.#.#.#####.#####.#.#####.#.#####.#################.#
#.........#.F.#.#.#.#.#...#.#.#.#.....#.#...#.#.....#.#...#.#.#.....#.#.......#.#
#########.#.###.#.#.#H###.#.#.#.#.###.###.#.#.#####.#.###.#.#.###.#.#.#.#####.#.#
#.......#.#.....#.#.#...#.#...#.#...#...#.#.......#.#...#.#.#...#.#...#.#...#...#
#.###.###.#####.#.#####.#.###.#.###.###.#.#####.###.#.###T#.###.#.#####.###.#####
#...#.........#.#...#.#.#...#.#.#...#...#...#...#...#.....#.#...#.#.....#.......#
#.#############.###.#C#.#####.#.###.#.#####.#.###.#########.#####.#.#####.#######
#.#........f....#.#...#.......#...#g#...#...#...#.#...............#.#...#......u#
#.#K#############.#######.#######N#.###.#.#####B#.###########.###.#.#.#.#####.#.#
#.#.....#.....A.#....a..#.#.U...#.#...#.#.#.....#...#.........#...#...#.....#.#.#
#.#####.###.###.###.###.#.#.###.#.#.###.#.#########.#.#########.###########.###.#
#.#...#..k..#.#.....#.#.#...#w..#.#.#...#...........#.#...#...#.......#.....#...#
#.#.#########.#######.#.#########.#.#.#.#############.#.#.#.#.#######.#.#####.###
#.#.........#.........#.#......s..#.#.#.#.#.......#...#.#...#.......#.#.#.#.....#
#.#.###.###.#####.###.#S#.#########.#.###.#.#.###.#.###.###########.###.#.#.###.#
#.#...#.#.#.#...#.#...#...#...#...#.#...#.#.#.#.....#...#.....#...#...#.#...#...#
#.#####.#.#.#.#.#.#.#######.#.#.###.###.#.#.#.###.###.###.#.#.###.###.#.#.###.###
#..e....#.#.#.#...#...#.....#.#.......#.#...#...#.#...#...#.#.......#...#...#...#
#########.#.#.#######.#####.#.#.#######.#.#####.#M#.#####.#.#######.###########.#
#.......I.#.#.#.......#.....#.#..j#...#.#...#...#.#.#...#.#...#...#.#...#.......#
#.#########.#.#.#######.#####.#####.#.#.#####.#####Q#.#.#####.###.#.#.#.#.#####.#
#.....#...#...#.......#.#...#.......#.#.#.....#.....#.#.....#...#.#.Z.#.#.O.#.#.#
#.###.#.#.###########.#.#.###########.#.#.#####.#####.#####.###.#.#####.###.#.#.#
#...#...#...............#...............#..d..........#.........#...........#..h#
#################################################################################"
}
mod tests {
    use crate::{
        challenging_input, day_18_example_1, day_18_example_2, day_18_example_3, day_18_example_4,
        day_18_example_5, day_18_puzzle_input, Explorer, Map, Obj, UndergroundVault, Vec2,
    };

    #[test]
    fn object_from_char() {
        assert_eq!(Obj::from('@'), Obj::Player);
        assert_eq!(Obj::from('#'), Obj::Wall);
        assert_eq!(Obj::from('d'), Obj::Key('d'));
        assert_eq!(Obj::from('F'), Obj::Door('F'));
    }
    #[test]
    fn door_matches_key() {
        assert_eq!(Obj::Door('A').matching_key(), 'a');
    }
    #[test]
    fn non_door_does_not_match_key() {
        assert_ne!(Obj::Key('A').matching_key(), 'a');
    }
    #[test]
    fn key_matches_door() {
        assert_eq!(Obj::Key('a').matching_door(), 'A');
    }
    #[test]
    fn non_key_does_not_match_dor() {
        assert_ne!(Obj::Door('a').matching_door(), 'A');
    }
    fn simple_3x3_map() -> Map {
        Map {
            map: vec![
                vec![Obj::Wall, Obj::Wall, Obj::Wall],
                vec![Obj::Key('a'), Obj::Player, Obj::Door('A')],
                vec![Obj::Wall, Obj::Wall, Obj::Wall],
            ],
            width: 3,
            height: 3,
        }
    }
    #[test]
    fn map_from_str() {
        assert_eq!(
            Map::from(
                "###
a@A
###"
            ),
            simple_3x3_map()
        );
    }
    #[test]
    fn object_at_pos() {
        let vault = simple_3x3_map();
        assert_eq!(vault.object_at(&Vec2::new(0, 0)), &Obj::Wall);
        assert_eq!(vault.object_at(&Vec2::new(0, 1)), &Obj::Key('a'));
        assert_eq!(vault.object_at(&Vec2::new(1, 1)), &Obj::Player);
        assert_eq!(vault.object_at(&Vec2::new(2, 1)), &Obj::Door('A'));
        assert_eq!(vault.object_at(&Vec2::new(2, 2)), &Obj::Wall);
    }
    #[test]
    fn player_pos() {
        assert_eq!(simple_3x3_map().player_pos(), Vec2::new(1, 1));
    }

    #[test]
    fn reachable_positions_without_key() {
        let vault = simple_3x3_map();
        assert_eq!(
            Explorer::initial(vault.player_pos()).reachable_positions(&vault),
            vec![Vec2::new(0, 1)]
        );
    }
    #[test]
    fn reachable_positions_with_key() {
        let vault = simple_3x3_map();
        let pos = Vec2::new(1, 1);
        let paths = vec![vec![]];
        let keys = vec!['a'];
        assert_eq!(
            Explorer { pos, paths, keys }.reachable_positions(&vault),
            vec![Vec2::new(2, 1), Vec2::new(0, 1)]
        );
    }

    #[test]
    fn reachable_key_path_simple() {
        let vault = UndergroundVault::from(simple_3x3_map());
        assert_eq!(
            vault.move_to_keys_with(&vault.initial_explorer())[0].curr_path(),
            vec![Vec2::new(1, 1), Vec2::new(0, 1)]
        );
    }
    #[test]
    fn reachable_key_path_example_1() {
        let vault = UndergroundVault::from(day_18_example_1());
        assert_eq!(vault.map.player_pos(), Vec2::new(5, 1));
        assert_eq!(
            vault.move_to_keys_with(&vault.initial_explorer())[0].curr_path(),
            vec![Vec2::new(5, 1), Vec2::new(6, 1), Vec2::new(7, 1)]
        );
    }
    #[test]
    fn collect_all_keys_simple() {
        assert_eq!(
            UndergroundVault::from(simple_3x3_map()).shortest_path_collecting_all_keys(),
            1
        );
    }
    #[test]
    fn can_be_moved_onto_example_1() {
        assert_eq!(
            Map::from(day_18_example_1()).can_be_moved_onto(&Vec2::new(3, 1), &vec!['a']),
            true
        );
    }
    #[test]
    fn collect_all_keys_example_1() {
        assert_eq!(
            UndergroundVault::from(day_18_example_1()).shortest_path_collecting_all_keys(),
            8
        );
    }
    #[test]
    fn collect_all_keys_example_2() {
        assert_eq!(
            UndergroundVault::from(day_18_example_2()).shortest_path_collecting_all_keys(),
            86
        );
    }
    #[test]
    fn collect_all_keys_example_3() {
        assert_eq!(
            UndergroundVault::from(day_18_example_3()).shortest_path_collecting_all_keys(),
            132
        );
    }
    #[test]
    fn collect_all_keys_example_4() {
        assert_eq!(
            UndergroundVault::from(day_18_example_4()).shortest_path_collecting_all_keys(),
            136
        );
    }
    #[test]
    fn collect_all_keys_example_5() {
        assert_eq!(
            UndergroundVault::from(day_18_example_5()).shortest_path_collecting_all_keys(),
            81
        );
    }
    #[test]
    fn collect_all_keys_with_challenging_input() {
        // from https://www.reddit.com/r/adventofcode/comments/ecj4e7/2019_day_18_challenging_input/
        assert_eq!(
            UndergroundVault::from(challenging_input()).shortest_path_collecting_all_keys(),
            20
        );
    }
    #[test]
    fn collect_all_keys_part_1() {
        assert_eq!(
            UndergroundVault::from(day_18_puzzle_input()).shortest_path_collecting_all_keys(),
            1
        );
    }
}
