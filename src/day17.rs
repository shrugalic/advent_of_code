use std::ops::RangeInclusive;

type Coord = usize;
type Loc = (Coord, Coord);
type TileRow = Vec<Tile>;

const SPRING: Loc = (500, 0);

#[derive(Clone)]
enum Tile {
    Sand,
    Clay,
    Spring,
    FlowingWater,
    TrappedWater,
}

pub(crate) struct Ground {
    // Rows of Tiles, indexed by [y][x]
    tile_rows: Vec<TileRow>,
    x_min: Coord,
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
        let depth = *horizontal_clays.iter().map(|(y, _)| y).max().unwrap();

        let mut tile_rows = vec![vec![Tile::Sand; width]; depth + 1];
        tile_rows[SPRING.1][SPRING.0 - x_min] = Tile::Spring;
        vertical_clays.into_iter().for_each(|(x, y_range)| {
            y_range
                .into_iter()
                .for_each(|y| tile_rows[y][x - x_min] = Tile::Clay)
        });
        horizontal_clays.into_iter().for_each(|(y, x_range)| {
            x_range
                .into_iter()
                .for_each(|x| tile_rows[y][x - x_min] = Tile::Clay)
        });

        Ground { tile_rows, x_min }
    }
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
            Tile::FlowingWater => '|',
            Tile::TrappedWater => '~',
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
}
