use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;
use Tile::*;

const INPUT: &str = include_str!("../input/day18.txt");

pub(crate) fn day18_part1() -> usize {
    let voxels = parse(INPUT);
    total_surface_area_of(voxels)
}

pub(crate) fn day18_part2() -> usize {
    let mut voxels = Voxels::from(parse(INPUT));
    voxels.outer_surface_area()
}

fn total_surface_area_of(voxels: Vec<Voxel>) -> usize {
    let distinct_xs: HashSet<_> = voxels.iter().map(|Voxel { x, .. }| x).collect();
    let distinct_ys: HashSet<_> = voxels.iter().map(|Voxel { y, .. }| y).collect();
    let distinct_zs: HashSet<_> = voxels.iter().map(|Voxel { z, .. }| z).collect();

    let mut surface_count = 6 * voxels.len();

    fn count_direct_neighbors(it: impl Iterator<Item = Coord>) -> usize {
        let mut list: Vec<_> = it.collect::<HashSet<_>>().into_iter().collect();
        list.sort_unstable();
        list.windows(2).filter(|el| el[0] + 1 == el[1]).count()
    }
    for xd in &distinct_xs {
        for yd in &distinct_ys {
            let touching_z_count = count_direct_neighbors(
                // with same x and y
                voxels
                    .iter()
                    .filter(|Voxel { x, y, .. }| &x == xd && &y == yd)
                    .map(|Voxel { z, .. }| *z),
            );
            surface_count -= 2 * touching_z_count;
        }
    }
    for xd in &distinct_xs {
        for zd in &distinct_zs {
            let touching_y_count = count_direct_neighbors(
                // with same x and z
                voxels
                    .iter()
                    .filter(|Voxel { x, z, .. }| &x == xd && &z == zd)
                    .map(|Voxel { y, .. }| *y),
            );
            surface_count -= 2 * touching_y_count;
        }
    }
    for yd in &distinct_ys {
        for zd in &distinct_zs {
            let touching_x_count = count_direct_neighbors(
                // with same y and z
                voxels
                    .iter()
                    .filter(|Voxel { y, z, .. }| &y == yd && &z == zd)
                    .map(|Voxel { x, .. }| *x),
            );
            surface_count -= 2 * touching_x_count;
        }
    }

    surface_count
}

impl Voxels {
    fn outer_surface_area(&mut self) -> usize {
        self.fill_with_water();

        // println!("{}", &self);

        /*
        Layer z = 8:       Layer z = 9:         Layer z = 10:         Layer z = 11:
        ~~~~~~####~~#~~~~~~   ~~~~~~##~####~~~~~~   ~~~~~~#######~~~~~~   ~~~~~~~######~~~~~~
        ~~~~############~~~   ~~~~~#####.####~~~~   ~~~~###########~~~~   ~~~~~##########~~~~
        ~~~###.#.######~~~~   ~~~~########.####~~   ~~~######.#######~~   ~~~~#############~~
        ~~~####......####~~   ~~~####...##..####~   ~~######..#..####~~   ~~~#######..#####~~
        ~~####.........#.#~   ~~~##......#..####~   ~~##.##......######   ~~####........###~~
        ~~##...........###~   ~~####..........##~   ~####..........####   ~###...........###~
        ~##.#...........##~   ~###............###   ~#.##..........#.##   ~###............###
        ~####...........###   ~####..........####   ~###..........#.###   ~###............###
        ~~##.............##   ####.............##   ####...........#.##   ~###............###
        ~###............###   ###.............###   ~##.............###   ~####...........###
        ~###............###   ~####...........###   ~##.............###   ~###............###
        ~###...........####   ~####..........#.##   ~###............###   ~~###..........###~
        ~####.........#####   ~~##...........###~   ~~####.........#.##   ~~####........####~
        ~~####.......#####~   ~~####..........##~   ~~###.........###~~   ~~####........####~
        ~~~####.#..#.#.###~   ~~#.####.#..#.###~~   ~~~#####....######~   ~~~~####.....###~~~
        ~~~###############~   ~~~#####...#..###~~   ~~~~####.#######~~~   ~~~~#############~~
        ~~~~~###########~~~   ~~~~##########~#~~~   ~~~~~#########.#~~~   ~~~~###########~~~~
        ~~~~~~###~###~~~~~~   ~~~~~~~##~####~~~~~   ~~~~~~~########~~~~   ~~~~~~~~~#####~~~~~
        */

        // Do it again, to fill any spots missed during the first pass, such as the one
        // in layer 10, second to last row, 5-th char from the right.
        // That is covered in both x- and y-, but not z-direction, so the z-pass _should_ fill it.
        // However, during the z-pass, the same spot in layer 9 was not yet filled with water.

        self.fill_with_water();

        let lava = self.voxels_filled_with(Lava);
        let total_surface_area = total_surface_area_of(lava);

        let holes = self.voxels_filled_with(Air);
        let inner_surface_area = total_surface_area_of(holes);

        total_surface_area - inner_surface_area
    }

    fn fill_with_water(&mut self) {
        for z in self.z_range() {
            for y in self.y_range() {
                for x in self.x_range() {
                    if (z == 0
                        || y == 0
                        || x == 0
                        || z == self.max_z()
                        || y == self.max_y()
                        || x == self.max_x())
                        && self.tiles[z][y][x] == Air
                    {
                        self.tiles[z][y][x] = Water;
                    }
                    if z > 0 {
                        // Fill z from below
                        if self.tiles[z][y][x] == Air && self.tiles[z - 1][y][x] == Water {
                            self.tiles[z][y][x] = Water;
                        }
                        // Mirror to fill z from above
                        let mz = self.max_z() - z;
                        if self.tiles[mz][y][x] == Air && self.tiles[mz + 1][y][x] == Water {
                            self.tiles[mz][y][x] = Water;
                        }
                    }
                    if y > 0 {
                        // Fill y from below
                        if self.tiles[z][y][x] == Air && self.tiles[z][y - 1][x] == Water {
                            self.tiles[z][y][x] = Water;
                        }
                        // Mirror to fill y from above
                        let my = self.max_y() - y;
                        if self.tiles[z][my][x] == Air && self.tiles[z][my + 1][x] == Water {
                            self.tiles[z][my][x] = Water;
                        }
                    }
                    if x > 0 {
                        // Fill x from below
                        if self.tiles[z][y][x] == Air && self.tiles[z][y][x - 1] == Water {
                            self.tiles[z][y][x] = Water;
                        }
                        // Mirror to fill x from above
                        let mx = self.max_x() - x;
                        if self.tiles[z][y][mx] == Air && self.tiles[z][y][mx + 1] == Water {
                            self.tiles[z][y][mx] = Water;
                        }
                    }
                }
            }
        }
    }

    fn voxels_filled_with(&self, wanted: Tile) -> Vec<Voxel> {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(z, layer)| {
                layer.iter().enumerate().flat_map(move |(y, line)| {
                    line.iter().enumerate().filter_map(move |(x, tile)| {
                        if *tile == wanted {
                            Some(Voxel {
                                x: x + self.min_x,
                                y: y + self.min_y,
                                z: z + self.min_z,
                            })
                        } else {
                            None
                        }
                    })
                })
            })
            .collect()
    }

    fn x_range(&self) -> RangeInclusive<usize> {
        0..=self.max_x()
    }
    fn y_range(&self) -> RangeInclusive<usize> {
        0..=self.max_y()
    }
    fn z_range(&self) -> RangeInclusive<usize> {
        0..=self.max_z()
    }

    fn max_x(&self) -> usize {
        self.tiles[0][0].len() - 1
    }
    fn max_y(&self) -> usize {
        self.tiles[0].len() - 1
    }
    fn max_z(&self) -> usize {
        self.tiles.len() - 1
    }
}
impl Display for Voxels {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.tiles
                .iter()
                .enumerate()
                .map(|(z, layer)| format!(
                    "Layer z = {z}:\n{}",
                    layer
                        .iter()
                        .map(|line| line.iter().map(|t| t.to_char()).collect::<String>())
                        .collect::<Vec<String>>()
                        .join("\n")
                ))
                .collect::<Vec<String>>()
                .join("\n\n")
        )
    }
}
struct Voxels {
    tiles: Vec<Vec<Vec<Tile>>>,
    min_x: Coord,
    min_y: Coord,
    min_z: Coord,
}
impl From<Vec<Voxel>> for Voxels {
    fn from(voxels: Vec<Voxel>) -> Self {
        let min_x = *voxels.iter().map(|Voxel { x, .. }| x).min().unwrap();
        let max_x = *voxels.iter().map(|Voxel { x, .. }| x).max().unwrap();
        let x_len = 1 + max_x - min_x;

        let min_y = *voxels.iter().map(|Voxel { y, .. }| y).min().unwrap();
        let max_y = *voxels.iter().map(|Voxel { y, .. }| y).max().unwrap();
        let y_len = 1 + max_y - min_y;

        let min_z = *voxels.iter().map(|Voxel { z, .. }| z).min().unwrap();
        let max_z = *voxels.iter().map(|Voxel { z, .. }| z).max().unwrap();
        let z_len = 1 + max_z - min_z;

        let mut tiles = vec![vec![vec![Air; x_len]; y_len]; z_len];
        voxels
            .iter()
            .for_each(|Voxel { x, y, z }| tiles[*z - min_z][*y - min_y][*x - min_x] = Lava);
        Voxels {
            tiles,
            min_x,
            min_y,
            min_z,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Water,
    Lava,
    Air,
}
impl Tile {
    fn to_char(self) -> char {
        match self {
            Water => '~',
            Lava => '#',
            Air => '.',
        }
    }
}

fn parse(input: &str) -> Vec<Voxel> {
    input.trim().lines().map(Voxel::from).collect()
}

type Coord = usize;
#[derive(Eq, PartialEq, Hash)]
struct Voxel {
    x: Coord,
    y: Coord,
    z: Coord,
}
impl From<&str> for Voxel {
    fn from(line: &str) -> Self {
        let voxels: Vec<_> = line.split(',').map(|c| c.parse().unwrap()).collect();
        Voxel {
            x: voxels[0],
            y: voxels[1],
            z: voxels[2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part1_example() {
        let voxels = parse(EXAMPLE);
        assert_eq!(64, total_surface_area_of(voxels));
    }

    #[test]
    fn part1() {
        assert_eq!(3_454, day18_part1());
    }

    #[test]
    fn part2_example() {
        let mut voxels = Voxels::from(parse(EXAMPLE));
        assert_eq!(58, voxels.outer_surface_area());
    }

    #[test]
    fn part2() {
        assert_eq!(2_014, day18_part2());
    }
}
