use line_reader::read_file_to_lines;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::{Debug, Display, Formatter};

pub(crate) fn day20_part1() -> usize {
    shortest_path(read_file_to_lines("input/day20.txt"), Part::One)
}

pub(crate) fn day20_part2() -> usize {
    shortest_path(read_file_to_lines("input/day20.txt"), Part::Two)
}

fn shortest_path(input: Vec<String>, part: Part) -> usize {
    let maze = Maze::from(input);
    maze.length_of_shortest_path_from_start_to_end(part)
}

#[derive(PartialEq)]
enum Part {
    One,
    Two,
}

type Coord = usize;
type X = Coord;
type Y = Coord;
type Grid = Vec<Vec<Tile>>;

#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
struct Loc {
    x: X,
    y: Y,
}
impl Display for Loc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Debug for Loc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Loc {
    fn x_offset(&self, offset: isize) -> Loc {
        self.offset(offset, 0)
    }
    fn y_offset(&self, offset: isize) -> Loc {
        self.offset(0, offset)
    }
    fn offset(&self, x: isize, y: isize) -> Loc {
        Loc {
            x: (self.x as isize + x) as usize,
            y: (self.y as isize + y) as usize,
        }
    }
}

struct Maze {
    grid: Grid,
}
impl From<Vec<String>> for Maze {
    fn from(input: Vec<String>) -> Self {
        let mut door_locations = vec![];
        let grid: Grid = input
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let tile = Tile::from(c);
                        if matches!(&tile, Tile::Door(_)) {
                            door_locations.push(Loc { x, y });
                        }
                        tile
                    })
                    .collect()
            })
            .collect();
        let mut maze = Maze { grid };
        maze.place_start_end_and_portals_on_paths_next_to_doors(door_locations);
        maze
    }
}
impl Maze {
    fn length_of_shortest_path_from_start_to_end(&self, part: Part) -> usize {
        let start_of_path = self.loc_of_tiles_matching(&|tile| tile == &Tile::Start)[0];
        // println!("start_of_path = {}", start_of_path);
        let mut steps_to_loc = vec![vec![vec![usize::MAX; self.grid[0].len()]; self.grid.len()]; 1];
        let mut to_visit = BinaryHeap::new();
        to_visit.push(Candidate::new(0, 0, start_of_path));
        'search: while let Some(Candidate { steps, level, loc }) = to_visit.pop() {
            // println!("{} (level {}) @ {} ({})", steps, level, loc, to_visit.len());
            steps_to_loc[level][loc.y][loc.x] = steps;
            for next in self.neighbors_of(&loc) {
                if steps + 1 < steps_to_loc[level][next.y][next.x] {
                    match self.get_tile(&next) {
                        Some(Tile::Path) => {
                            to_visit.push(Candidate::new(steps + 1, level, next));
                        }
                        Some(Tile::End) => {
                            if level == 0 {
                                steps_to_loc[level][next.y][next.x] = steps + 1;
                                break 'search;
                            }
                        }
                        Some(Tile::Portal(a, b)) => {
                            // Mark the portal entrance so we don't attempt traveling back right
                            // after exiting on the other side
                            steps_to_loc[level][next.y][next.x] = steps + 1;
                            self.loc_of_tiles_matching(&|tile| {
                                tile.is_matching_portal_to(&Tile::Portal(*a, *b))
                            })
                            .into_iter()
                            .filter(|entrance| entrance != &next)
                            .for_each(|exit| {
                                match part {
                                    Part::One => {
                                        to_visit.push(Candidate::new(steps + 2, level, exit));
                                    }
                                    Part::Two => {
                                        if self.is_outer(&next) {
                                            // outer portal -> decrease level
                                            if level > 0 {
                                                // println!("Up {}{} to level {}", a, b, level - 1);
                                                to_visit.push(Candidate::new(
                                                    steps + 2,
                                                    level - 1,
                                                    exit,
                                                ));
                                            }
                                        } else {
                                            // inner portal -> increase level
                                            if level + 1 == steps_to_loc.len() {
                                                steps_to_loc.push(vec![
                                                    vec![
                                                        usize::MAX;
                                                        self.grid[0].len()
                                                    ];
                                                    self.grid.len()
                                                ]);
                                            }
                                            // println!("Down {}{} to level {}", a, b, level + 1);
                                            to_visit.push(Candidate::new(
                                                steps + 2,
                                                level + 1,
                                                exit,
                                            ));
                                        }
                                    }
                                }
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
        let end_of_path = self.loc_of_tiles_matching(&|tile| tile == &Tile::End)[0];
        // println!("end_of_path {}", end_of_path);
        steps_to_loc[0][end_of_path.y][end_of_path.x]
    }
    fn is_outer(&self, loc: &Loc) -> bool {
        loc.x == 2 || loc.x == self.grid[0].len() - 3 || loc.y == 2 || loc.y == self.grid.len() - 3
    }
    fn loc_of_tiles_matching(&self, filter: &dyn Fn(&Tile) -> bool) -> Vec<Loc> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, tile)| filter(tile))
                    .map(move |(x, _)| Loc { x, y })
            })
            .collect()
    }
    fn get_tile(&self, loc: &Loc) -> Option<&Tile> {
        self.grid.get(loc.y).and_then(|row| row.get(loc.x))
    }
    fn set_tile(&mut self, loc: &Loc, tile: Tile) {
        self.grid[loc.y][loc.x] = tile
    }
    fn neighbors_of(&self, loc: &Loc) -> Vec<Loc> {
        let mut neighbors = vec![];
        if loc.x > 0 {
            neighbors.push(loc.x_offset(-1));
        }
        if loc.y > 0 {
            neighbors.push(loc.y_offset(-1));
        }
        if loc.x + 1 < self.grid[0].len() {
            neighbors.push(loc.x_offset(1));
        }
        if loc.y + 1 < self.grid.len() {
            neighbors.push(loc.y_offset(1));
        }
        neighbors
    }
    fn place_start_end_and_portals_on_paths_next_to_doors(&mut self, door_locations: Vec<Loc>) {
        for loc in door_locations {
            let is_path_on = |tile| matches!(tile, Some(&Tile::Path));
            let above_the_doors = || loc.y_offset(-1);
            let below_door = || loc.y_offset(1);
            let below_the_doors = || loc.y_offset(2);
            let the_tile = |loc| self.get_tile(loc);
            let to_the_left_of_the_doors = || loc.x_offset(-1);
            let to_the_right = || loc.x_offset(1);
            let to_the_right_of_the_doors = || loc.x_offset(2);
            if let Some(&Tile::Door(a)) = self.get_tile(&loc) {
                if let Some(&Tile::Door(b)) = self.get_tile(&below_door()) {
                    // Two doors above each other == vertical portal
                    if is_path_on(the_tile(&below_the_doors())) {
                        // Portal entrance is below the two doors
                        self.set_tile(&below_the_doors(), Tile::from_doors(a, b));
                    } else {
                        // Portal entrance is above the two doors
                        self.set_tile(&above_the_doors(), Tile::from_doors(a, b));
                    }
                } else if let Some(&Tile::Door(b)) = self.get_tile(&to_the_right()) {
                    // Two doors next to each other == horizontal portal
                    if is_path_on(the_tile(&to_the_right_of_the_doors())) {
                        // Portal entrance is to the right of the doors
                        self.set_tile(&to_the_right_of_the_doors(), Tile::from_doors(a, b));
                    } else {
                        // Portal entrance is to the left of the doors
                        self.set_tile(&to_the_left_of_the_doors(), Tile::from_doors(a, b));
                    }
                }
            }
        }
    }
}
#[derive(PartialEq, Eq)]
struct Candidate {
    steps: usize,
    level: usize,
    loc: Loc,
}
impl Candidate {
    fn new(steps: usize, level: usize, loc: Loc) -> Self {
        Candidate { steps, level, loc }
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.steps.cmp(&other.steps) {
            Ordering::Equal => {
                match self.level.cmp(&other.level) {
                    Ordering::Equal => self.loc.cmp(&other.loc),
                    level => level.reverse(), // shallower level is better
                }
            }
            steps => steps.reverse(), // fewer steps is better
        }
    }
}
#[derive(Debug, PartialEq)]
enum Tile {
    Nothing,
    Wall,
    Path,
    Door(char),
    Portal(char, char),
    Start,
    End,
}
impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            ' ' => Tile::Nothing,
            '#' => Tile::Wall,
            '.' => Tile::Path,
            'A'..='Z' => Tile::Door(c),
            _ => panic!("Illegal tile {}", c),
        }
    }
}
impl Tile {
    fn from_doors(a: char, b: char) -> Tile {
        match (a, b) {
            ('A', 'A') => Tile::Start,
            ('Z', 'Z') => Tile::End,
            (_, _) => {
                if a < b {
                    Tile::Portal(a, b)
                } else {
                    Tile::Portal(b, a)
                }
            }
        }
    }
    fn is_matching_portal_to(&self, other: &Tile) -> bool {
        match (self, other) {
            (Tile::Portal(a, b), Tile::Portal(c, d)) => a == c && b == d,
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "         A         
         A         
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z     
             Z      ";

    const LARGER_EXAMPLE: &str = "                   A             
                   A             
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";

    #[test]
    fn example_parse() {
        let maze = Maze::from(read_str_to_lines(EXAMPLE));

        assert_eq!(&maze.grid[0][9], &Tile::Door('A'));
        assert_eq!(&maze.grid[1][9], &Tile::Door('A'));
        assert_eq!(&maze.grid[2][9], &Tile::Start);

        assert_eq!(&maze.grid[8][0], &Tile::Door('B'));
        assert_eq!(&maze.grid[8][1], &Tile::Door('C'));
        assert_eq!(&maze.grid[8][2], &Tile::Portal('B', 'C'));

        assert_eq!(&maze.grid[10][6], &Tile::Portal('D', 'E'));
        assert_eq!(&maze.grid[10][7], &Tile::Door('D'));
        assert_eq!(&maze.grid[10][8], &Tile::Door('E'));

        assert_eq!(&maze.grid[12][11], &Tile::Portal('F', 'G'));
        assert_eq!(&maze.grid[15][2], &Tile::Portal('F', 'G'));

        assert_eq!(&maze.grid[16][13], &Tile::End);
        assert_eq!(&maze.grid[17][13], &Tile::Door('Z'));
        assert_eq!(&maze.grid[18][13], &Tile::Door('Z'));
    }

    #[test]
    fn part1_example() {
        let maze = Maze::from(read_str_to_lines(EXAMPLE));
        assert_eq!(
            23,
            maze.length_of_shortest_path_from_start_to_end(Part::One)
        );
    }

    #[test]
    fn part1_larger_example() {
        let maze = Maze::from(read_str_to_lines(LARGER_EXAMPLE));
        assert_eq!(
            58,
            maze.length_of_shortest_path_from_start_to_end(Part::One)
        );
    }

    #[test]
    fn part1() {
        assert_eq!(686, day20_part1());
    }

    const PART2_INTERESTING_EXAMPLE: &str = "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";

    #[test]
    fn part2_interesting_example() {
        let maze = Maze::from(read_str_to_lines(PART2_INTERESTING_EXAMPLE));
        assert_eq!(
            396,
            maze.length_of_shortest_path_from_start_to_end(Part::Two)
        );
    }

    #[test]
    fn part2() {
        assert_eq!(8384, day20_part2());
    }
}
