use std::ops::RangeInclusive;

type Coord = usize;
type TileRow = Vec<Tile>;

#[derive(PartialEq, Copy, Clone, Debug)]
struct Loc {
    x: Coord,
    y: Coord,
}
impl Loc {
    fn new(x: Coord, y: Coord) -> Self {
        Loc { x, y }
    }
    fn left(&self) -> Loc {
        self.offset_by(-1, 0)
    }
    fn right(&self) -> Loc {
        self.offset_by(1, 0)
    }
    fn below(&self) -> Loc {
        self.offset_by(0, 1)
    }
    fn above(&self) -> Loc {
        self.offset_by(0, -1)
    }
    fn offset_by(&self, x_offset: isize, y_offset: isize) -> Loc {
        Loc {
            x: (self.x as isize + x_offset) as usize,
            y: (self.y as isize + y_offset) as usize,
        }
    }
}

const SPRING: Loc = Loc { x: 500, y: 0 };

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Sand,   // . where water can flow through
    Clay,   // # where water rests on
    Spring, // + where water flows from
    Flow,   // | sand through which water flowed (or is currently flowing through)
    Water,  // ~ where water has settled to rest
}

impl From<&[String]> for Ground {
    fn from(input: &[String]) -> Self {
        let vertical_clays: Vec<(Coord, RangeInclusive<Coord>)> = input
            .iter()
            .filter_map(|line| Ground::parse_line(line, "x=", "y="))
            .collect();
        let horizontal_clays: Vec<(Coord, RangeInclusive<Coord>)> = input
            .iter()
            .filter_map(|line| Ground::parse_line(line, "y=", "x="))
            .collect();

        let x_values = || vertical_clays.iter().map(|(x, _)| x);
        let x_range = x_values().min().unwrap()..=x_values().max().unwrap();
        // -1 because we need one extra Sand tile on the left side
        let x_min = **x_range.start() - 1;
        // +1 to adjust for the inclusive range, and
        // +2 because we need one extra Sand tile each on the left and right sides
        let width = *x_range.end() - *x_range.start() + 1 + 2;
        let y_min = usize::min(
            *vertical_clays
                .iter()
                .map(|(_, ys)| ys.start())
                .min()
                .unwrap(),
            *horizontal_clays.iter().map(|(y, _)| y).min().unwrap(),
        );
        let depth = *horizontal_clays.iter().map(|(y, _)| y).max().unwrap();

        let tile_rows = vec![vec![Tile::Sand; width]; depth + 1];
        let mut ground = Ground {
            tile_rows,
            x_min,
            y_min,
            rounds: 0,
        };
        ground.set_tile(&SPRING, Tile::Spring);
        vertical_clays.into_iter().for_each(|(x, y_range)| {
            y_range
                .into_iter()
                .for_each(|y| ground.set_tile(&Loc::new(x, y), Tile::Clay))
        });
        horizontal_clays.into_iter().for_each(|(y, x_range)| {
            x_range
                .into_iter()
                .for_each(|x| ground.set_tile(&Loc::new(x, y), Tile::Clay))
        });

        ground
    }
}

pub(crate) struct Ground {
    // Rows of Tiles, indexed by [y][x]
    tile_rows: Vec<TileRow>,
    x_min: Coord,
    y_min: Coord,
    rounds: usize,
}

impl ToString for Ground {
    fn to_string(&self) -> String {
        // Example header and grid:
        //    44444455555555
        //    99999900000000
        //    45678901234567

        //  0 ......+.......
        //  1 ............#.
        //  2 .#..#.......#.
        //  3 .#..#..#......
        //  4 .#..#..#......
        //  5 .#.....#......
        //  6 .#.....#......
        //  7 .#######......
        //  8 ..............
        //  9 ..............
        // 10 ....#.....#...
        // 11 ....#.....#...
        // 12 ....#.....#...
        // 13 ....#######...
        let row_width = self.tile_rows[0].len();
        let y_coord_len = self.tile_rows.len().to_string().len();
        let prefix = " ".repeat(y_coord_len + 1); // +1 for single space between y-coord and grid
        let x_coord_len = (self.x_min + row_width).to_string().len();
        let header = (0..x_coord_len).into_iter().map(|i| {
            let x_coords: String = (self.x_min..(self.x_min + row_width))
                .into_iter()
                .map(|x| x.to_string().chars().nth(i).unwrap())
                .collect();
            format!("{}{}", prefix, x_coords)
        });
        let grid = self.tile_rows.iter().enumerate().map(|(y, tiles)| {
            format!(
                "{:2$} {}",
                y,
                tiles.iter().map(Tile::to_char).collect::<String>(),
                y_coord_len
            )
        });
        header.chain(grid).collect::<Vec<String>>().join("\n")
    }
}
impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Sand => '.',
            Tile::Clay => '#',
            Tile::Spring => '+',
            Tile::Flow => '|',
            Tile::Water => '~',
        }
    }
}

impl Ground {
    fn parse_line(line: &str, coord: &str, range: &str) -> Option<(Coord, RangeInclusive<Coord>)> {
        // Example lines:
        // x=495, y=2..7
        // y=7, x=495..501
        if line.starts_with(coord) {
            let (coord_part, range_part) = line.split_once(", ").unwrap();
            let coord = coord_part.trim_start_matches(coord).parse().unwrap();
            let (lo, hi) = range_part
                .trim_start_matches(range)
                .split_once("..")
                .unwrap();
            let range = (lo.parse().unwrap())..=(hi.parse().unwrap());
            Some((coord, range))
        } else {
            None
        }
    }

    pub(crate) fn tiles_reachable_by_water(&mut self) -> usize {
        let (flow, water) = self.let_water_flow_until_stable(usize::MAX);
        flow + water
    }

    pub(crate) fn water_retained_when_spring_runs_dry(&mut self) -> usize {
        let (_flow, water) = self.let_water_flow_until_stable(usize::MAX);
        water
    }

    fn let_water_flow_until_stable(&mut self, max: usize) -> (usize, usize) {
        let mut flows = vec![SPRING];
        while !flows.is_empty() {
            let flow = flows.remove(0);
            match self.get_tile(&flow.below()) {
                Some(Tile::Sand) => {
                    // flow down
                    self.set_tile(&flow.below(), Tile::Flow);
                    flows.push(flow.below());
                    self.rounds += 1;
                }
                Some(Tile::Clay | Tile::Water) => {
                    // flow sideways
                    let next = self.flow_sideways(flow);
                    flows.extend(next);
                    self.rounds += 1;
                }
                Some(Tile::Flow) => {}
                Some(Tile::Spring) => unreachable!(),
                None => {} // this stream can't flow
            }
            if self.rounds == max {
                break;
            }
        }

        (
            self.all_tiles_matching(&|tile| tile == &Tile::Flow)
                .iter()
                .filter(|loc| loc.y >= self.y_min)
                .count(),
            self.all_tiles_matching(&|tile| tile == &Tile::Water)
                .iter()
                .filter(|loc| loc.y >= self.y_min)
                .count(),
        )
    }

    fn flow_sideways(&mut self, loc: Loc) -> Vec<Loc> {
        let left = &|edge: Loc| edge.left();
        let right = &|edge: Loc| edge.right();
        let mut next_flows = vec![];
        let left_next = self.flow_to_one_side(loc, &left, &right);
        if let Some(first) = left_next.get(0) {
            if first.y < loc.y {
                return left_next;
            }
        }
        next_flows.extend(left_next);
        let right_next = self.flow_to_one_side(loc, &right, &left);
        if let Some(first) = right_next.get(0) {
            if first.y < loc.y {
                return right_next;
            }
        }
        next_flows.extend(right_next);
        next_flows
    }

    fn flow_to_one_side(
        &mut self,
        loc: Loc,
        flow_dir: &dyn Fn(Loc) -> Loc,
        other_dir: &dyn Fn(Loc) -> Loc,
    ) -> Vec<Loc> {
        let mut next_flows = vec![];
        let next: Loc = flow_dir(loc);
        match self.get_tile(&next) {
            Some(Tile::Sand) => {
                // Flow into this sand, and remember this location to flow from next turn
                self.set_tile(&next, Tile::Flow);
                next_flows.push(next);
            }
            Some(Tile::Clay) => {
                // There's a wall on this side. If there's a wall on the other side, convert the
                // whole puddle into water, and return the above in-flows as the next flows
                if let Some(wall) = self.find_wall(loc, other_dir) {
                    self.convert_to_water(loc, wall, other_dir);
                    next_flows.extend(self.get_inflows(loc, wall, other_dir));
                }
            }
            Some(Tile::Flow) => {
                // This flow could be from another stream, both growing inward.
                // #|||->  <-|||#
                // In this case neither side would check for walls any more, so let's do it here
                if let Some(this_wall) = self.find_wall(loc, flow_dir) {
                    if let Some(other_wall) = self.find_wall(loc, other_dir) {
                        self.convert_to_water(other_dir(this_wall), other_wall, other_dir);
                        next_flows.extend(self.get_inflows(this_wall, other_wall, other_dir));
                    }
                }
            }
            Some(Tile::Water) => {}
            Some(Tile::Spring) | None => unreachable!(),
        }
        next_flows
    }

    fn find_wall(&mut self, loc: Loc, next: &dyn Fn(Loc) -> Loc) -> Option<Loc> {
        let mut edge: Loc = next(loc);
        while let Some(&Tile::Flow) = self.get_tile(&edge) {
            edge = next(edge);
        }
        if let Some(&Tile::Clay) = self.get_tile(&edge) {
            Some(edge)
        } else {
            None
        }
    }

    fn convert_to_water(&mut self, loc: Loc, wall: Loc, next: &dyn Fn(Loc) -> Loc) {
        let mut curr = loc;
        while curr != wall {
            self.set_tile(&curr, Tile::Water);
            curr = next(curr);
        }
    }

    fn get_inflows(&self, loc: Loc, wall: Loc, next: &dyn Fn(Loc) -> Loc) -> Vec<Loc> {
        let mut in_flows = vec![];
        let mut curr = loc;
        while curr != wall {
            let above = curr.above();
            if let Some(Tile::Flow) = self.get_tile(&above) {
                in_flows.push(above);
            }
            curr = next(curr);
        }
        in_flows
    }

    fn all_tiles_matching(&mut self, filter: &dyn Fn(&Tile) -> bool) -> Vec<Loc> {
        let x_min = self.x_min;
        self.tile_rows
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, tile)| filter(*tile))
                    .map(move |(x, _)| Loc::new(x + x_min, y))
            })
            .collect()
    }
    fn get_tile(&self, loc: &Loc) -> Option<&Tile> {
        self.tile_rows
            .get(loc.y)
            .and_then(|row: &TileRow| row.get(loc.x - self.x_min))
    }
    fn set_tile(&mut self, loc: &Loc, new_tile: Tile) {
        self.tile_rows[loc.y][loc.x - self.x_min] = new_tile
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    const EXAMPLE_IN: &str = "\
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    const EXAMPLE_OUT: &str = "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ............#.
 2 .#..#.......#.
 3 .#..#..#......
 4 .#..#..#......
 5 .#.....#......
 6 .#.....#......
 7 .#######......
 8 ..............
 9 ..............
10 ....#.....#...
11 ....#.....#...
12 ....#.....#...
13 ....#######...";

    #[test]
    fn example_output() {
        let ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        assert_eq!(EXAMPLE_OUT, ground.to_string());
    }

    #[test]
    fn water_flowed_once() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        ground.let_water_flow_until_stable(1);

        assert_eq!(
            ground.to_string(),
            "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ......|.....#.
 2 .#..#.......#.
 3 .#..#..#......
 4 .#..#..#......
 5 .#.....#......
 6 .#.....#......
 7 .#######......
 8 ..............
 9 ..............
10 ....#.....#...
11 ....#.....#...
12 ....#.....#...
13 ....#######..."
        );
    }

    #[test]
    fn water_flowed_6_times() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        ground.let_water_flow_until_stable(6);

        assert_eq!(
            ground.to_string(),
            "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ......|.....#.
 2 .#..#.|.....#.
 3 .#..#.|#......
 4 .#..#.|#......
 5 .#....|#......
 6 .#....|#......
 7 .#######......
 8 ..............
 9 ..............
10 ....#.....#...
11 ....#.....#...
12 ....#.....#...
13 ....#######..."
        );
    }

    #[test]
    fn water_flowed_8_times() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        ground.let_water_flow_until_stable(8);

        assert_eq!(
            ground.to_string(),
            "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ......|.....#.
 2 .#..#.|.....#.
 3 .#..#.|#......
 4 .#..#.|#......
 5 .#....|#......
 6 .#..|||#......
 7 .#######......
 8 ..............
 9 ..............
10 ....#.....#...
11 ....#.....#...
12 ....#.....#...
13 ....#######..."
        );
    }

    #[test]
    fn water_flowed_9_times() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        ground.let_water_flow_until_stable(9);

        assert_eq!(
            ground.to_string(),
            "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ......|.....#.
 2 .#..#.|.....#.
 3 .#..#.|#......
 4 .#..#.|#......
 5 .#....|#......
 6 .#.||||#......
 7 .#######......
 8 ..............
 9 ..............
10 ....#.....#...
11 ....#.....#...
12 ....#.....#...
13 ....#######..."
        );
    }

    #[test]
    fn water_flowed_10_times() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        ground.let_water_flow_until_stable(10);

        assert_eq!(
            ground.to_string(),
            "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ......|.....#.
 2 .#..#.|.....#.
 3 .#..#.|#......
 4 .#..#.|#......
 5 .#....|#......
 6 .#~~~~~#......
 7 .#######......
 8 ..............
 9 ..............
10 ....#.....#...
11 ....#.....#...
12 ....#.....#...
13 ....#######..."
        );
    }
    #[test]
    fn water_flowed_11_times() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        ground.let_water_flow_until_stable(11);

        assert_eq!(
            ground.to_string(),
            "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ......|.....#.
 2 .#..#.|.....#.
 3 .#..#.|#......
 4 .#..#.|#......
 5 .#...||#......
 6 .#~~~~~#......
 7 .#######......
 8 ..............
 9 ..............
10 ....#.....#...
11 ....#.....#...
12 ....#.....#...
13 ....#######..."
        );
    }

    #[test]
    fn water_flowed_15_times() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        ground.let_water_flow_until_stable(15);

        assert_eq!(
            ground.to_string(),
            "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ......|.....#.
 2 .#..#.|.....#.
 3 .#..#.|#......
 4 .#..#~~#......
 5 .#~~~~~#......
 6 .#~~~~~#......
 7 .#######......
 8 ..............
 9 ..............
10 ....#.....#...
11 ....#.....#...
12 ....#.....#...
13 ....#######..."
        );
    }

    #[test]
    fn water_flowed_16_times() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        ground.let_water_flow_until_stable(16);

        assert_eq!(
            ground.to_string(),
            "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ......|.....#.
 2 .#..#.|.....#.
 3 .#..#~~#......
 4 .#..#~~#......
 5 .#~~~~~#......
 6 .#~~~~~#......
 7 .#######......
 8 ..............
 9 ..............
10 ....#.....#...
11 ....#.....#...
12 ....#.....#...
13 ....#######..."
        );
    }

    #[test]
    fn water_flowed_17_times() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        ground.let_water_flow_until_stable(17);

        assert_eq!(
            ground.to_string(),
            "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ......|.....#.
 2 .#..#|||....#.
 3 .#..#~~#......
 4 .#..#~~#......
 5 .#~~~~~#......
 6 .#~~~~~#......
 7 .#######......
 8 ..............
 9 ..............
10 ....#.....#...
11 ....#.....#...
12 ....#.....#...
13 ....#######..."
        );
    }

    #[test]
    fn water_flowed_18_times() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        ground.let_water_flow_until_stable(18);

        assert_eq!(
            ground.to_string(),
            "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ......|.....#.
 2 .#..#|||....#.
 3 .#..#~~#......
 4 .#..#~~#......
 5 .#~~~~~#......
 6 .#~~~~~#......
 7 .#######......
 8 ..............
 9 ..............
10 ....#.....#...
11 ....#.....#...
12 ....#.....#...
13 ....#######..."
        );
    }

    #[test]
    fn water_flowed_19_times() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        ground.let_water_flow_until_stable(19);

        assert_eq!(
            ground.to_string(),
            "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ......|.....#.
 2 .#..#||||...#.
 3 .#..#~~#......
 4 .#..#~~#......
 5 .#~~~~~#......
 6 .#~~~~~#......
 7 .#######......
 8 ..............
 9 ..............
10 ....#.....#...
11 ....#.....#...
12 ....#.....#...
13 ....#######..."
        );
    }

    #[test]
    fn water_flowed_20_times() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        ground.let_water_flow_until_stable(20);

        assert_eq!(
            ground.to_string(),
            "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ......|.....#.
 2 .#..#||||...#.
 3 .#..#~~#|.....
 4 .#..#~~#......
 5 .#~~~~~#......
 6 .#~~~~~#......
 7 .#######......
 8 ..............
 9 ..............
10 ....#.....#...
11 ....#.....#...
12 ....#.....#...
13 ....#######..."
        );
    }

    #[test]
    fn water_flowed_29_times() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        ground.let_water_flow_until_stable(29);

        assert_eq!(
            ground.to_string(),
            "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ......|.....#.
 2 .#..#||||...#.
 3 .#..#~~#|.....
 4 .#..#~~#|.....
 5 .#~~~~~#|.....
 6 .#~~~~~#|.....
 7 .#######|.....
 8 ........|.....
 9 ........|.....
10 ....#...|.#...
11 ....#...|.#...
12 ....#...|.#...
13 ....#######..."
        );
    }

    #[test]
    fn water_flowed_33_times() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        ground.let_water_flow_until_stable(33);

        assert_eq!(
            ground.to_string(),
            "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ......|.....#.
 2 .#..#||||...#.
 3 .#..#~~#|.....
 4 .#..#~~#|.....
 5 .#~~~~~#|.....
 6 .#~~~~~#|.....
 7 .#######|.....
 8 ........|.....
 9 ........|.....
10 ....#...|.#...
11 ....#...|.#...
12 ....#~~~~~#...
13 ....#######..."
        );
    }

    #[test]
    fn water_flowed_41_times() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        ground.let_water_flow_until_stable(41);

        assert_eq!(
            ground.to_string(),
            "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ......|.....#.
 2 .#..#||||...#.
 3 .#..#~~#|.....
 4 .#..#~~#|.....
 5 .#~~~~~#|.....
 6 .#~~~~~#|.....
 7 .#######|.....
 8 ........|.....
 9 ........|.....
10 ....#~~~~~#...
11 ....#~~~~~#...
12 ....#~~~~~#...
13 ....#######..."
        );
    }

    #[test]
    fn water_flowed_56_times() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        ground.let_water_flow_until_stable(56);

        assert_eq!(
            ground.to_string(),
            "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ......|.....#.
 2 .#..#||||...#.
 3 .#..#~~#|.....
 4 .#..#~~#|.....
 5 .#~~~~~#|.....
 6 .#~~~~~#|.....
 7 .#######|.....
 8 ........|.....
 9 ...|||||||||..
10 ...|#~~~~~#|..
11 ...|#~~~~~#|..
12 ...|#~~~~~#|..
13 ...|#######|.."
        );
    }

    #[test]
    fn water_flowed_61_times() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        ground.let_water_flow_until_stable(61);

        assert_eq!(
            ground.to_string(),
            "   \
   44444455555555
   99999900000000
   45678901234567
 0 ......+.......
 1 ......|.....#.
 2 .#..#||||...#.
 3 .#..#~~#|.....
 4 .#..#~~#|.....
 5 .#~~~~~#|.....
 6 .#~~~~~#|.....
 7 .#######|.....
 8 ........|.....
 9 ...|||||||||..
10 ...|#~~~~~#|..
11 ...|#~~~~~#|..
12 ...|#~~~~~#|..
13 ...|#######|.."
        );
    }

    #[test]
    fn example_flow_until_stable() {
        let mut ground = Ground::from(read_str_to_lines(EXAMPLE_IN).as_slice());
        let tiles_reachable_by_water = ground.tiles_reachable_by_water();
        assert_eq!(57, tiles_reachable_by_water);
    }

    #[test]
    fn part1() {
        let mut ground = Ground::from(read_file_to_lines("input/day17.txt").as_slice());
        let tiles_reachable_by_water = ground.tiles_reachable_by_water();
        assert_eq!(31949, tiles_reachable_by_water);
    }

    #[test]
    fn part2() {
        let mut ground = Ground::from(read_file_to_lines("input/day17.txt").as_slice());
        let retained_water_count = ground.water_retained_when_spring_runs_dry();
        assert_eq!(26384, retained_water_count);
    }
}
