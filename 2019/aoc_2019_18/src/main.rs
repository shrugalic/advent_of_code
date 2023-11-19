use coffee::graphics::{Color, Frame, Mesh, Point, Rectangle, Shape, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::FromIterator;
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
        Task::new(|| UndergroundVault::from(day_18_larger_example_3()))
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

        self.ongoing
            .iter()
            .for_each(|search| mesh.fill(ct.square_at(&search.start), BLUE));

        mesh.draw(&mut frame.as_target());
    }

    fn update(&mut self, _window: &Window) {
        if !self.ongoing.is_empty() {
            println!("\n\nRunning {} ongoing searches", self.ongoing.len());
            self.run_searches();
            if self.ongoing.len() > 300_000 {
                panic!("Breaking because of too many searches()");
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(1_000));
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
struct Graph {
    // distances from key to key
    distances: BTreeMap<(char, char), usize>,
    // keys needed to get from a key to another key
    req_keys: BTreeMap<(char, char), BTreeSet<char>>,
    keys: BTreeSet<char>,
}
impl From<Map> for Graph {
    fn from(map: Map) -> Self {
        let (distances, req_keys) = map.all_distances_and_keys();
        let mut keys = BTreeSet::from_iter(
            distances
                .keys()
                .flat_map(|(a, b)| vec![*a, *b])
                .filter(|&a| a != '@'),
        );
        Graph {
            distances,
            req_keys,
            keys,
        }
    }
}
impl From<&str> for Graph {
    fn from(input: &str) -> Self {
        Graph::from(Map::from(input))
    }
}
impl Graph {
    fn keys_other_than(&self, others: &BTreeSet<char>) -> BTreeSet<char> {
        self.keys.difference(others).cloned().collect()
    }
    fn req_keys(&self, key1: &char, key2: &char) -> &BTreeSet<char> {
        // self.req_keys pairs are stored with keys in alphabetical order
        if key1 < key2 {
            self.req_keys.get(&(*key1, *key2)).unwrap()
        } else if key1 > key2 {
            self.req_keys.get(&(*key2, *key1)).unwrap()
        } else {
            panic!("req_keys called two of the same key '{}'", key1);
        }
    }
}
#[derive(PartialEq, Debug)]
struct Map {
    // map[y[x]] = object
    // Example:
    // x: 012345789
    // y: 0 [[#########],
    // y: 1  [#b.A.@.a#],
    // y: 2  [#########]]
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
    fn can_be_moved_onto_with_optional_key(&self, pos: &Vec2) -> (bool, Option<char>) {
        if pos.x < self.width && pos.y < self.height {
            match self.object_at(pos) {
                Obj::Player | Obj::EmptySpace | Obj::Key(_) => (true, None),
                Obj::Wall => (false, None),
                door @ Obj::Door(_) => (true, Some(door.matching_key())),
            }
        } else {
            (false, None)
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
    fn key_at(&self, pos: &Vec2) -> Option<char> {
        if let Obj::Key(key) = self.object_at(pos) {
            Some(*key)
        } else {
            None
        }
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
    fn sorted_keys(&self) -> Vec<char> {
        let mut keys: Vec<char> = self
            .point_obj_pairs()
            .into_iter()
            .filter_map(|(_pos, obj)| match obj {
                Obj::Key(key) => Some(key),
                _ => None,
            })
            .collect();
        keys.sort();
        keys
    }
    /// Set of all keys
    fn key_set(&self) -> BTreeSet<char> {
        BTreeSet::from_iter(self.sorted_keys())
    }
    fn pos_of_key(&self, wanted: char) -> Vec2 {
        self.point_obj_pairs()
            .iter()
            .find_map(|(pos, obj)| {
                if let Obj::Key(key) = obj {
                    if *key == wanted {
                        Some(pos.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .expect(&format!("Map contains no key '{}'", wanted))
    }
    /// Distances from the player/all keys to all other keys
    fn all_distances(&self) -> BTreeMap<(char, char), usize> {
        let (distances, _keys) = self.all_distances_and_keys();
        distances
    }
    /// Required keys to go from the player/all keys to all other keys
    fn required_keys(&self) -> BTreeMap<(char, char), BTreeSet<char>> {
        let (_distances, keys) = self.all_distances_and_keys();
        keys
    }
    /// Distances and required keys from the player/all keys to all other keys
    fn all_distances_and_keys(
        &self,
    ) -> (
        BTreeMap<(char, char), usize>,
        BTreeMap<(char, char), BTreeSet<char>>,
    ) {
        // Distances from a key or player a to another key b.
        // The two chars are different (distance to self is 0)
        // The first char is smaller than the second char alphabetically to
        // half the map size, as distance ('a', 'b') == dist('b', 'a')
        let mut distances: BTreeMap<(char, char), usize> = BTreeMap::new();
        let mut req_keys: BTreeMap<(char, char), BTreeSet<char>> = BTreeMap::new();

        let mut source = '@';
        let mut keys_wanted = self.key_set();
        while !keys_wanted.is_empty() {
            println!("Distances from {} to {:?}", source, keys_wanted);
            for (target, (dist, keys)) in self.distances_and_keys(source, &keys_wanted) {
                //                println!(" to {}: {}", target, dist);
                distances.insert((source, target), dist);
                req_keys.insert((source, target), keys);
            }
            // Remove first element from the set (is there no better method?) to be the next source
            source = keys_wanted.iter().next().cloned().unwrap();
            keys_wanted.take(&source).unwrap();
        }
        (distances, req_keys)
    }
    /// Distances from the player/a key to some other keys
    fn distances_and_keys(
        &self,
        source: char,
        keys_wanted: &BTreeSet<char>,
    ) -> Vec<(char, (usize, BTreeSet<char>))> {
        // Distance to key
        let mut distances: Vec<(char, (/*dist*/ usize, /*req_keys*/ BTreeSet<char>))> = vec![];
        let mut keys_found = BTreeSet::new();
        // Shortest distance to given position
        let mut path_lengths: HashMap<Vec2, usize> = HashMap::new();

        let start = if source == '@' {
            self.player_pos()
        } else {
            self.pos_of_key(source)
        };
        let mut explorers: Vec<Explorer> = vec![Explorer {
            start,
            pos: start,
            path_len: 0,
            coll_keys: BTreeSet::new(),
            req_keys: BTreeSet::new(),
        }];
        while !explorers.is_empty() {
            // Move current explorers to unexplored neighboring positions
            explorers = explorers
                .iter() // Note: `par_iter` not worth it for puzzle input (3.1s parallel vs 2.5s serial)
                .flat_map(|explorer| explorer.next_explorers(self, &path_lengths, false))
                .collect();
            // Update path_lengths, found_keys and distances while also removing duplicate explorers
            explorers = explorers
                .drain(0..)
                .filter_map(|mut explorer| {
                    // println!("{:?}", explorer);
                    // Only keep better ones (this gets rid of duplicates as well)
                    if path_lengths.contains_key(&explorer.pos) {
                        // Position already visited by an earlier (=faster) explorer
                        None
                    } else {
                        // First explorer on this position -> store
                        path_lengths.insert(explorer.pos, explorer.path_len);
                        match self.object_at(&explorer.pos) {
                            Obj::Key(key) => {
                                if keys_wanted.contains(key) {
                                    keys_found.insert(*key);
                                    distances.push((
                                        *key,
                                        (explorer.path_len, explorer.req_keys.clone()),
                                    ));
                                }
                            }
                            Obj::Door(door) => {
                                explorer.req_keys.insert(Obj::from(*door).matching_key());
                            }
                            _ => {}
                        }
                        Some(explorer)
                    }
                })
                .collect();
            // Are we done yet?
            if keys_found == *keys_wanted {
                return distances;
            }
        }
        distances
    }
}
#[derive(Debug, PartialEq)]
struct GraphSearch {
    start: char,               // Starting char
    coll_keys: BTreeSet<char>, // Collected keys
    tot_distance: usize,       // Total distance (sum of individual distances)
}
impl From<char> for GraphSearch {
    fn from(start: char) -> Self {
        GraphSearch {
            start,
            coll_keys: BTreeSet::new(),
            tot_distance: 0,
        }
    }
}
impl GraphSearch {
    fn init_from_start() -> Self {
        GraphSearch {
            start: '@',
            coll_keys: BTreeSet::new(),
            tot_distance: 0,
        }
    }
    fn init_from_key(key: char) -> Self {
        GraphSearch {
            start: key,
            coll_keys: BTreeSet::from_iter(vec![key]),
            tot_distance: 0,
        }
    }
    fn expanded_to(&self, new_key: char, path_len: usize) -> GraphSearch {
        let mut coll_keys = self.coll_keys.clone();
        coll_keys.insert(new_key);
        GraphSearch {
            start: new_key,
            coll_keys,
            tot_distance: self.tot_distance + path_len,
        }
    }
    fn new_searches_from_reachable_keys(&self, graph: &Graph) -> Vec<GraphSearch> {
        graph
            .keys_other_than(&self.coll_keys)
            .iter()
            .filter(|other_key| self.can_reach(other_key, graph))
            .map(|reachable_key| {
                let dist = graph.distances.get(&(self.start, *reachable_key)).unwrap();
                self.expanded_to(*reachable_key, *dist)
            })
            .collect()
    }
    fn can_reach(&self, other_key: &char, graph: &Graph) -> bool {
        let req_keys = graph.req_keys(&self.start, &other_key);
        self.coll_keys.is_superset(req_keys)
    }
}
#[derive(Debug)]
struct GraphSearchBot {
    graph: Graph,               // Graph to search
    ongoing: Vec<GraphSearch>,  // Ongoing searches
    complete: Vec<GraphSearch>, // Complete searches
}
impl GraphSearchBot {
    /// Search repeatedly until all keys are collected
    fn shortest_path_collecting_all_keys(&mut self) -> usize {
        while !self.ongoing.is_empty() {
            self.run_searches();
        }
        println!("Finished with {} searches", self.complete.len());
        let shortest = self
            .complete
            .iter()
            .min_by_key(|search| search.tot_distance)
            .unwrap();
        // println!("Shortest = {:?}", shortest);
        shortest.tot_distance
    }
    fn run_searches(&mut self) {
        let key_count = self.graph.keys.len();
        // println!("key count = {}", key_count);
        // Next line avoids "cannot borrow `self` as immutable because it is also borrowed as mutable"
        let graph = &self.graph;
        // Next line avoids "closure requires unique access to `self` but it is already borrowed"
        let mut complete = vec![];
        let mut ongoing: Vec<GraphSearch> = self
            .ongoing
            .drain(0..)
            .flat_map(|curr| curr.new_searches_from_reachable_keys(graph))
            .filter_map(|next| {
                if next.coll_keys.len() == key_count {
                    println!("Search finished with length {}", next.tot_distance);
                    complete.push(next);
                    None
                } else {
                    // ongoing
                    Some(next)
                }
            })
            .collect();
        self.complete.extend(complete);
        ongoing.sort_unstable_by_key(|s| s.start);
        //        println!("\n{} ongoing searches:", ongoing.len());
        let unmerged_len = ongoing.len();
        ongoing
            .iter()
            .take(30)
            .for_each(|s| println!("{}, {:?}, {}", s.start, s.coll_keys, s.tot_distance));

        let merged: Vec<GraphSearch> = ongoing
            .into_iter()
            .coalesce(|prev, curr| {
                if prev.start == curr.start && prev.coll_keys == curr.coll_keys {
                    if prev.tot_distance <= curr.tot_distance {
                        Ok(prev)
                    } else {
                        Ok(curr)
                    }
                } else {
                    Err((prev, curr))
                }
            })
            .collect();
        println!(
            "\n{} merged (from {}) ongoing searches:",
            merged.len(),
            unmerged_len
        );
        //        merged
        //            .iter()
        //            .take(10)
        //            .for_each(|s| println!("{}, {:?}, {}", s.start, s.keys, s.path_len));

        self.ongoing = merged;

        // TODO check does this work yet?
    }
}
#[derive(Debug)]
struct Search {
    start: Vec2,               // Starting position
    coll_keys: BTreeSet<char>, // Collected keys
    path_len: usize,           // Total path length (sum of individual paths)
}
impl From<Vec2> for Search {
    fn from(start: Vec2) -> Self {
        Search {
            start,
            coll_keys: BTreeSet::new(),
            path_len: 0,
        }
    }
}
impl From<Explorer> for Search {
    fn from(explorer: Explorer) -> Self {
        Search {
            start: explorer.pos,
            coll_keys: explorer.coll_keys,
            path_len: explorer.path_len,
        }
    }
}
impl From<&Map> for Search {
    fn from(map: &Map) -> Self {
        Search {
            start: map.player_pos(),
            coll_keys: BTreeSet::new(),
            path_len: 0,
        }
    }
}
impl From<&UndergroundVault> for Search {
    fn from(vault: &UndergroundVault) -> Self {
        Search::from(&vault.map)
    }
}
impl Search {
    // Return new Searches that can be started from keys reachable by this search
    fn new_searches_from_reachable_keys(&self, map: &Map) -> Vec<Search> {
        //        println!(
        //            "\nStarting new search from {} with path_len {} and keys {:?}",
        //            self.start, self.path_len, self.keys
        //        );
        let mut path_lengths: HashMap<Vec2, usize> = HashMap::new();
        let mut steps = 0;
        let mut next_searches: Vec<Search> = vec![];
        let mut explorers: Vec<Explorer> = vec![Explorer::from(self)];
        while !explorers.is_empty() {
            let (finished, searching) = self.expand_search(explorers, map, &mut path_lengths);
            next_searches.extend(
                finished
                    .into_iter()
                    .map(|on_key| Search {
                        start: on_key.pos,
                        coll_keys: on_key.coll_keys,
                        path_len: self.path_len + on_key.path_len,
                    })
                    .collect::<Vec<Search>>(),
            );
            explorers = searching;

            steps += 1;
            if steps > 40 {
                //                println!("breaking. explorers = {:?}", explorers);
                //                return next_searches;
            }
        }
        next_searches
    }
    fn expand_search(
        &self,
        explorers: Vec<Explorer>,
        map: &Map,
        path_lengths: &mut HashMap<Vec2, usize>,
    ) -> (
        Vec<Explorer>, // Finished (found a key)
        Vec<Explorer>, // Still searching for a key
    ) {
        //        println!("Expanding key search with {} explorers.", explorers.len()); //, collectors);
        let mut finished: Vec<Explorer> = vec![];
        let mut searching: Vec<Explorer> = vec![];

        for mut explorer in explorers {
            if !path_lengths.contains_key(&explorer.pos) {
                path_lengths.insert(explorer.pos, explorer.path_len);
                if let Some(key) = explorer.new_key_at_current_pos(&map) {
                    // This explorer reached a key and is finished
                    //                    println!("New key {} at pos {:?}", key, explorer.pos);
                    explorer.coll_keys.insert(key);
                    finished.push(explorer);
                } else {
                    // Continue searching for keys
                    searching.extend(
                        explorer
                            .next_explorers(map, &path_lengths, true)
                            .drain(0..)
                            .filter(|next| // only better ones
                                        !path_lengths.contains_key(&next.pos))
                            .collect::<Vec<Explorer>>(),
                    )
                }
            }
            // Else position already visited by an earlier/faster explorer
        }
        (finished, searching)
    }
}
#[derive(Debug)]
struct UndergroundVault {
    map: Map,              // Map of the tunnels
    ongoing: Vec<Search>,  // Ongoing searches
    complete: Vec<Search>, // Complete searches
}
impl From<&str> for UndergroundVault {
    fn from(input: &str) -> Self {
        let map = Map::from(input);
        let search = Search::from(&map);
        UndergroundVault {
            map,
            ongoing: vec![search],
            complete: vec![],
        }
    }
}
impl From<Map> for UndergroundVault {
    fn from(map: Map) -> Self {
        let search = Search::from(&map);
        UndergroundVault {
            map,
            ongoing: vec![search],
            complete: vec![],
        }
    }
}
impl UndergroundVault {
    /// Search repeatedly until all keys are collected
    fn shortest_path_collecting_all_keys(&mut self) -> usize {
        while !self.ongoing.is_empty() {
            self.run_searches();
        }
        println!("Finished with {} searches", self.complete.len());
        let shortest = self
            .complete
            .iter()
            .min_by_key(|search| search.path_len)
            .unwrap();
        // println!("Shortest = {:?}", shortest);
        shortest.path_len
    }
    fn run_searches(&mut self) {
        let key_count = self.map.key_count();
        // println!("key count = {}", key_count);
        // Next line avoids "cannot borrow `self` as immutable because it is also borrowed as mutable"
        let map = &self.map;
        // Next line avoids "closure requires unique access to `self` but it is already borrowed"
        let mut complete = vec![];
        let mut ongoing: Vec<Search> = self
            .ongoing
            .drain(0..)
            .flat_map(|curr| curr.new_searches_from_reachable_keys(map))
            .filter_map(|next| {
                if next.coll_keys.len() == key_count {
                    println!("Search finished with length {}", next.path_len);
                    complete.push(next);
                    None
                } else {
                    // ongoing
                    Some(next)
                }
            })
            .collect();
        self.complete.extend(complete);
        ongoing.sort_unstable_by_key(|s| s.start);
        //        println!("\n{} ongoing searches:", ongoing.len());
        let unmerged_len = ongoing.len();
        ongoing
            .iter()
            .take(30)
            .for_each(|s| println!("{}, {:?}, {}", s.start, s.coll_keys, s.path_len));

        let merged: Vec<Search> = ongoing
            .into_iter()
            .coalesce(|prev, curr| {
                if prev.start == curr.start && prev.coll_keys == curr.coll_keys {
                    if prev.path_len <= curr.path_len {
                        Ok(prev)
                    } else {
                        Ok(curr)
                    }
                } else {
                    Err((prev, curr))
                }
            })
            .collect();
        println!(
            "\n{} merged (from {}) ongoing searches:",
            merged.len(),
            unmerged_len
        );
        //        merged
        //            .iter()
        //            .take(10)
        //            .for_each(|s| println!("{}, {:?}, {}", s.start, s.keys, s.path_len));

        self.ongoing = merged;
    }
}

#[derive(Debug, PartialEq, Clone)]
/// Key collectors
struct Explorer {
    start: Vec2,               // Starting pos
    pos: Vec2,                 // Current position
    path_len: usize,           // Path length
    coll_keys: BTreeSet<char>, // Collected keys
    req_keys: BTreeSet<char>,  // Required keys
}
impl From<&Search> for Explorer {
    fn from(search: &Search) -> Self {
        Explorer {
            start: search.start,
            pos: search.start,
            path_len: 0,
            coll_keys: search.coll_keys.clone(),
            req_keys: BTreeSet::new(),
        }
    }
}
impl Explorer {
    fn new_explorer_at(&self, pos: Vec2) -> Explorer {
        Explorer {
            start: self.start,
            pos,
            path_len: self.path_len + 1,
            coll_keys: self.coll_keys.clone(),
            req_keys: self.req_keys.clone(),
        }
    }
    fn reachable_positions(&self, map: &Map) -> Vec<(Vec2, Option<char>)> {
        let cross_positions = vec![
            self.pos.offset_by(1, 0),
            self.pos.offset_by(0, 1),
            self.pos.offset_by(-1, 0),
            self.pos.offset_by(0, -1),
        ];
        cross_positions
            .into_iter()
            .filter_map(|pos| match map.can_be_moved_onto_with_optional_key(&pos) {
                (false, _) => None,
                (true, key) => Some((pos, key)),
            })
            .collect()
    }
    fn new_key_at_current_pos(&self, map: &Map) -> Option<char> {
        if let Some(key) = map.key_at(&self.pos) {
            if !self.coll_keys.contains(&key) {
                return Some(key);
            }
        }
        None
    }
    fn next_explorers(
        &self,
        map: &Map,
        path_lengths: &HashMap<Vec2, usize>,
        require_key_to_pass_door: bool,
    ) -> Vec<Explorer> {
        self.reachable_positions(map)
            .into_iter()
            .filter_map(|(next_pos, optional_key)| match optional_key {
                None => Some(self.new_explorer_at(next_pos)),
                Some(required_key) => {
                    if require_key_to_pass_door {
                        if self.coll_keys.contains(&required_key) {
                            Some(self.new_explorer_at(next_pos))
                        } else {
                            None
                        }
                    } else {
                        let mut next = self.new_explorer_at(next_pos);
                        next.coll_keys.insert(required_key);
                        Some(next)
                    }
                }
            })
            .filter(|next| map.includes(&next.pos) && !path_lengths.contains_key(&next.pos))
            .collect()
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, PartialOrd, Ord)]
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

fn day_18_example_1() -> &'static str {
    "#########
#b.A.@.a#
#########"
}

fn day_18_larger_example_1() -> &'static str {
    "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"
}

fn day_18_larger_example_2() -> &'static str {
    "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"
}

fn day_18_larger_example_3() -> &'static str {
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
fn rotated_h_map() -> &'static str {
    "#########
#eCa.bDf#
####@####
#gBc.dAh#
#########"
}

fn day_18_larger_example_4() -> &'static str {
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

fn almost_empty_map() -> &'static str {
    "##########
#a.......#
#........#
#........#
#........#
#........#
#........#
#........#
#.......@#
##########"
}
fn short_and_long_way() -> &'static str {
    "#########
#a......#
#.#####.#
#.#.....#
#.#.#####
#.#.....#
#.#####.#
#......@#
#########"
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
        almost_empty_map, challenging_input, day_18_example_1, day_18_larger_example_1,
        day_18_larger_example_2, day_18_larger_example_3, day_18_larger_example_4,
        day_18_puzzle_input, rotated_h_map, short_and_long_way, Explorer, Graph, GraphSearch, Map,
        Obj, Search, UndergroundVault, Vec2,
    };
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use std::iter::FromIterator;

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
    fn need_key_to_reach_position_with_door() {
        let start = Vec2::new(1, 1);
        assert_eq!(
            Explorer {
                start,
                pos: start,
                path_len: 0,
                coll_keys: set_with_key_a(),
                req_keys: BTreeSet::new(),
            }
            .reachable_positions(&simple_3x3_map()),
            vec![(Vec2::new(2, 1), Some('a')), (Vec2::new(0, 1), None)]
        );
    }
    fn set_with_key_a() -> BTreeSet<char> {
        let mut keys = BTreeSet::new();
        keys.insert('a');
        keys
    }
    #[test]
    fn next_searches_simple() {
        let map = simple_3x3_map();
        let next = &Search::from(&map).new_searches_from_reachable_keys(&map)[0];
        assert_eq!(next.path_len, 1);
    }
    #[test]
    fn next_searches_example_1() {
        let vault = UndergroundVault::from(day_18_example_1());
        assert_eq!(vault.map.player_pos(), Vec2::new(5, 1));
        let next = &Search::from(&vault).new_searches_from_reachable_keys(&vault.map)[0];
        assert_eq!(next.path_len, 2);
        assert_eq!(next.coll_keys, set_with_key_a());
    }

    #[test]
    fn search_almost_empty_map() {
        let vault = UndergroundVault::from(almost_empty_map());
        assert_eq!(vault.map.player_pos(), Vec2::new(8, 8));
        let next = &Search::from(&vault).new_searches_from_reachable_keys(&vault.map)[0];
        assert_eq!(next.path_len, 14);
        assert_eq!(next.coll_keys, set_with_key_a());
    }

    #[test]
    fn search_short_and_long_way() {
        let vault = UndergroundVault::from(short_and_long_way());
        assert_eq!(vault.map.player_pos(), Vec2::new(7, 7));
        let next = &Search::from(&vault).new_searches_from_reachable_keys(&vault.map)[0];
        assert_eq!(next.path_len, 12);
        assert_eq!(next.coll_keys, set_with_key_a());
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
            Map::from(day_18_example_1()).can_be_moved_onto_with_optional_key(&Vec2::new(3, 1)),
            (true, Some('a'))
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
    fn collect_all_keys_larger_example_1() {
        assert_eq!(
            UndergroundVault::from(day_18_larger_example_1()).shortest_path_collecting_all_keys(),
            86
        );
    }
    #[test]
    fn collect_all_keys_larger_example_2() {
        assert_eq!(
            UndergroundVault::from(day_18_larger_example_2()).shortest_path_collecting_all_keys(),
            132
        );
    }
    #[ignore]
    #[test]
    fn collect_all_keys_larger_example_3() {
        assert_eq!(
            UndergroundVault::from(day_18_larger_example_3()).shortest_path_collecting_all_keys(),
            136
        );
    }
    #[test]
    fn collect_all_keys_larger_example_4() {
        assert_eq!(
            UndergroundVault::from(day_18_larger_example_4()).shortest_path_collecting_all_keys(),
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
    #[ignore]
    #[test]
    fn collect_all_keys_part_1() {
        assert_eq!(
            UndergroundVault::from(day_18_puzzle_input()).shortest_path_collecting_all_keys(),
            1
        );
    }
    #[test]
    fn collect_all_keys_rotated_h_map() {
        assert_eq!(
            UndergroundVault::from(rotated_h_map()).shortest_path_collecting_all_keys(),
            30
        );
    }
    #[test]
    fn unsorted_vec_equals() {
        // OK then, better use a set for the keys
        assert_ne!(vec!['g', 'c', 'i'], vec!['c', 'g', 'i']);
    }
    #[test]
    fn set_equals() {
        let mut ab = HashSet::new();
        ab.insert('a');
        ab.insert('b');
        let mut ba = HashSet::new();
        ba.insert('b');
        ba.insert('a');
        assert_eq!(ab, ba);
    }
    #[test]
    fn ascii() {
        assert_eq!('a' as u8, 97);
        assert_eq!('@' as u8, 64);
        assert_eq!(96 as char, '`');
    }
    #[test]
    fn keys_rotated_h_map() {
        let map = Map::from(rotated_h_map());
        assert_eq!(
            map.key_set(),
            BTreeSet::from_iter(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].into_iter())
        );
    }
    #[test]
    fn distances_rotated_h_map() {
        let map = Map::from(rotated_h_map());
        let mut distances: BTreeMap<(char, char), usize> = BTreeMap::new();
        // @
        distances.insert(('@', 'a'), 2);
        distances.insert(('@', 'b'), 2);
        distances.insert(('@', 'c'), 2);
        distances.insert(('@', 'd'), 2);
        distances.insert(('@', 'e'), 4);
        distances.insert(('@', 'f'), 4);
        distances.insert(('@', 'g'), 4);
        distances.insert(('@', 'h'), 4);
        // a
        distances.insert(('a', 'b'), 2);
        distances.insert(('a', 'c'), 4);
        distances.insert(('a', 'd'), 4);
        distances.insert(('a', 'e'), 2);
        distances.insert(('a', 'f'), 4);
        distances.insert(('a', 'g'), 6);
        distances.insert(('a', 'h'), 6);
        // b
        distances.insert(('b', 'c'), 4);
        distances.insert(('b', 'd'), 4);
        distances.insert(('b', 'e'), 4);
        distances.insert(('b', 'f'), 2);
        distances.insert(('b', 'g'), 6);
        distances.insert(('b', 'h'), 6);
        // c
        distances.insert(('c', 'd'), 2);
        distances.insert(('c', 'e'), 6);
        distances.insert(('c', 'f'), 6);
        distances.insert(('c', 'g'), 2);
        distances.insert(('c', 'h'), 4);
        // d
        distances.insert(('d', 'e'), 6);
        distances.insert(('d', 'f'), 6);
        distances.insert(('d', 'g'), 4);
        distances.insert(('d', 'h'), 2);
        // e
        distances.insert(('e', 'f'), 6);
        distances.insert(('e', 'g'), 8);
        distances.insert(('e', 'h'), 8);
        // f
        distances.insert(('f', 'g'), 8);
        distances.insert(('f', 'h'), 8);
        // g
        distances.insert(('g', 'h'), 6);

        assert_eq!(map.all_distances(), distances)
    }
    #[test]
    fn required_keys_rotated_h_map() {
        let map = Map::from(rotated_h_map());
        let mut keys: BTreeMap<(char, char), BTreeSet<char>> = BTreeMap::new();
        // @
        keys.insert(('@', 'a'), BTreeSet::new());
        keys.insert(('@', 'b'), BTreeSet::new());
        keys.insert(('@', 'c'), BTreeSet::new());
        keys.insert(('@', 'd'), BTreeSet::new());
        keys.insert(('@', 'e'), BTreeSet::from_iter(vec!['c']));
        keys.insert(('@', 'f'), BTreeSet::from_iter(vec!['d']));
        keys.insert(('@', 'g'), BTreeSet::from_iter(vec!['b']));
        keys.insert(('@', 'h'), BTreeSet::from_iter(vec!['a']));
        // a
        keys.insert(('a', 'b'), BTreeSet::new());
        keys.insert(('a', 'c'), BTreeSet::new());
        keys.insert(('a', 'd'), BTreeSet::new());
        keys.insert(('a', 'e'), BTreeSet::from_iter(vec!['c']));
        keys.insert(('a', 'f'), BTreeSet::from_iter(vec!['d']));
        keys.insert(('a', 'g'), BTreeSet::from_iter(vec!['b']));
        keys.insert(('a', 'h'), BTreeSet::from_iter(vec!['a']));
        // b
        keys.insert(('b', 'c'), BTreeSet::new());
        keys.insert(('b', 'd'), BTreeSet::new());
        keys.insert(('b', 'e'), BTreeSet::from_iter(vec!['c']));
        keys.insert(('b', 'f'), BTreeSet::from_iter(vec!['d']));
        keys.insert(('b', 'g'), BTreeSet::from_iter(vec!['b']));
        keys.insert(('b', 'h'), BTreeSet::from_iter(vec!['a']));
        // c
        keys.insert(('c', 'd'), BTreeSet::new());
        keys.insert(('c', 'e'), BTreeSet::from_iter(vec!['c']));
        keys.insert(('c', 'f'), BTreeSet::from_iter(vec!['d']));
        keys.insert(('c', 'g'), BTreeSet::from_iter(vec!['b']));
        keys.insert(('c', 'h'), BTreeSet::from_iter(vec!['a']));
        // d
        keys.insert(('d', 'e'), BTreeSet::from_iter(vec!['c']));
        keys.insert(('d', 'f'), BTreeSet::from_iter(vec!['d']));
        keys.insert(('d', 'g'), BTreeSet::from_iter(vec!['b']));
        keys.insert(('d', 'h'), BTreeSet::from_iter(vec!['a']));
        // e
        keys.insert(('e', 'f'), BTreeSet::from_iter(vec!['c', 'd']));
        keys.insert(('e', 'g'), BTreeSet::from_iter(vec!['b', 'c']));
        keys.insert(('e', 'h'), BTreeSet::from_iter(vec!['a', 'c']));
        // f
        keys.insert(('f', 'g'), BTreeSet::from_iter(vec!['b', 'd']));
        keys.insert(('f', 'h'), BTreeSet::from_iter(vec!['a', 'd']));
        // g
        keys.insert(('g', 'h'), BTreeSet::from_iter(vec!['a', 'b']));

        assert_eq!(map.required_keys(), keys)
    }
    // Feasibility check only. It takes less about one second, which is acceptable.
    #[ignore]
    #[test]
    fn distances_puzzle_input() {
        let map = Map::from(day_18_puzzle_input());
        let distances: BTreeMap<(char, char), usize> = BTreeMap::new();
        assert_eq!(map.all_distances(), distances)
    }
    #[test]
    fn rotated_h_map_key_count() {
        let graph = Graph::from(rotated_h_map());
        assert_eq!(graph.keys.len(), 8);
    }

    #[test]
    fn keys_other_than_start() {
        let graph = Graph::from(rotated_h_map());
        assert_eq!(
            graph.keys_other_than(&BTreeSet::from_iter(vec!['@'])),
            BTreeSet::from_iter(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'])
        );
    }
    #[test]
    fn keys_other_than_a() {
        let graph = Graph::from(rotated_h_map());
        assert_eq!(
            graph.keys_other_than(&BTreeSet::from_iter(vec!['a'])),
            BTreeSet::from_iter(vec!['b', 'c', 'd', 'e', 'f', 'g', 'h'])
        );
    }

    #[test]
    fn req_keys_right_order() {
        let graph = Graph::from(rotated_h_map());
        assert_eq!(
            graph.req_keys(&'e', &'h'),
            &BTreeSet::from_iter(vec!['a', 'c'])
        );
    }
    #[test]
    fn req_keys_wrong_order() {
        let graph = Graph::from(rotated_h_map());
        assert_eq!(
            graph.req_keys(&'h', &'e'),
            &BTreeSet::from_iter(vec!['a', 'c'])
        );
    }
    #[test]
    fn initial_new_searches_from_reachable_keys() {
        let search = GraphSearch::init_from_start();
        let graph = Graph::from(rotated_h_map());
        assert_eq!(
            search.new_searches_from_reachable_keys(&graph),
            vec![
                search.expanded_to('a', 2),
                search.expanded_to('b', 2),
                search.expanded_to('c', 2),
                search.expanded_to('d', 2)
            ]
        );
    }
    #[test]
    fn new_searches_from_reachable_keys() {
        let search = GraphSearch::init_from_key('a');
        let graph = Graph::from(rotated_h_map());
        assert_eq!(
            search.new_searches_from_reachable_keys(&graph),
            vec![
                search.expanded_to('b', 2),
                search.expanded_to('c', 4),
                search.expanded_to('d', 4),
                search.expanded_to('h', 6)
            ]
        );
    }
}
