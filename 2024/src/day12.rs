use crate::tile_grid::TileGrid;
use crate::vec_2d::Vec2D;
use crate::vec_tile_grid::VecTileGrid;
use std::fmt::{Debug, Formatter};

const INPUT: &str = include_str!("../../2024/input/day12.txt");

pub(crate) fn part1() -> usize {
    solve_part1(INPUT)
}

pub(crate) fn part2() -> usize {
    solve_part2(INPUT)
}

fn solve_part1(input: &str) -> usize {
    Farm::from(input)
        .regions
        .iter()
        .map(Region::fence_price)
        .sum()
}

fn solve_part2(input: &str) -> usize {
    Farm::from(input)
        .regions
        .iter()
        .map(Region::discounted_fence_price)
        .sum()
}

type Plant = char;

type Plot = Vec2D;
struct Region {
    plant: Plant,
    plots: Vec<Plot>,
}

#[derive(Debug)]
struct Farm {
    regions: Vec<Region>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct PlotBorder {
    side: Side,
    pov: Vec2D,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Side {
    Top,
    Left,
    Bottom,
    Right,
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        self.region_border_segments().len()
    }

    fn fence_price(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn discounted_fence_price(&self) -> usize {
        self.area() * self.side_count()
    }

    /// Determine the number of sides, where a "side" is a "straight section of fence".
    /// This is equivalent to the number of turns made while walking the border.
    fn side_count(&self) -> usize {
        let mut border_segments = self.region_border_segments();
        let mut corner_count = 0;
        // Start with a random border segment, and follow this border, counting turns
        let mut candidate = border_segments.pop();
        let mut first = candidate;
        while let Some(segment) = candidate {
            // The next border segment can be straight in front, or to the left or right (in walking direction)
            let (straight, left, right) = segment.possible_next_segments();
            if let Some(pos) = border_segments
                .iter()
                .position(|&bs| bs == straight || bs == left || bs == right)
            {
                // If there is one, continue from there.
                let next = border_segments.remove(pos);
                if next.side != segment.side {
                    corner_count += 1;
                }
                candidate = Some(next);
            } else {
                // If there is no match, one loop is finished.

                // Account for the possibility of having started in a corner
                if first.is_some_and(|s| s == left || s == right) {
                    corner_count += 1;
                }

                // Try again, there could be other loops
                candidate = border_segments.pop();
                first = candidate;
            }
        }
        corner_count
    }

    /// Region borders are unique, whereas inside borders have a duplicate in alternate form
    /// See Border::alternate_form() for further explanation
    fn region_border_segments(&self) -> Vec<PlotBorder> {
        let plot_borders: Vec<_> = self
            .plots
            .iter()
            .flat_map(|plot| {
                [Side::Top, Side::Left, Side::Bottom, Side::Right]
                    .into_iter()
                    .map(|side| PlotBorder { side, pov: *plot })
            })
            .collect();
        plot_borders
            .iter()
            .filter(|border| !plot_borders.contains(&border.alternate_form()))
            .cloned()
            .collect()
    }
}

impl PlotBorder {
    /// The alternate form of a plot border is the same border viewed from the other side of it.
    /// This is useful to remove plot borders _within_ a region, as all of those are contained
    /// twice, in two different forms (from the point of view of two neighboring plots)
    fn alternate_form(mut self) -> PlotBorder {
        match self.side {
            Side::Top => {
                self.side = Side::Bottom;
                self.pov.y -= 1;
            }
            Side::Left => {
                self.side = Side::Right;
                self.pov.x -= 1;
            }
            Side::Bottom => {
                self.side = Side::Top;
                self.pov.y += 1;
            }
            Side::Right => {
                self.side = Side::Left;
                self.pov.x += 1;
            }
        }
        self
    }

    /// Possible next border segments when walking the border in CCW order
    fn possible_next_segments(
        &self,
    ) -> (
        /* going straight */ Self,
        /* left turn*/ Self,
        /* right turn */ Self,
    ) {
        match self.side {
            Side::Top => {
                // going left, next can continue left, turn down or up
                (
                    PlotBorder {
                        side: self.side,
                        pov: self.pov.left_neighbor(),
                    },
                    PlotBorder {
                        side: Side::Left,
                        pov: self.pov,
                    },
                    PlotBorder {
                        side: Side::Right,
                        pov: self.pov.left_above_neighbor(),
                    },
                )
            }
            Side::Left => {
                // going down, next can continue down, turn right or left
                (
                    PlotBorder {
                        side: self.side,
                        pov: self.pov.below_neighbor(),
                    },
                    PlotBorder {
                        side: Side::Bottom,
                        pov: self.pov,
                    },
                    PlotBorder {
                        side: Side::Top,
                        pov: self.pov.left_below_neighbor(),
                    },
                )
            }
            Side::Bottom => {
                // going right, next can continue right, turn up or down
                (
                    PlotBorder {
                        side: self.side,
                        pov: self.pov.right_neighbor(),
                    },
                    PlotBorder {
                        side: Side::Right,
                        pov: self.pov,
                    },
                    PlotBorder {
                        side: Side::Left,
                        pov: self.pov.right_below_neighbor(),
                    },
                )
            }
            Side::Right => {
                // going up, next can continue up, turn left or right
                (
                    PlotBorder {
                        side: self.side,
                        pov: self.pov.above_neighbor(),
                    },
                    PlotBorder {
                        side: Side::Top,
                        pov: self.pov,
                    },
                    PlotBorder {
                        side: Side::Bottom,
                        pov: self.pov.right_above_neighbor(),
                    },
                )
            }
        }
    }
}

impl From<&str> for Farm {
    fn from(input: &str) -> Self {
        let grid = VecTileGrid::from(input);
        let mut regions: Vec<Region> = vec![];
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let plot = Vec2D::new(x, y);
                let plant = *grid.char_at(&plot).unwrap();
                if !regions.iter().any(|region| region.plots.contains(&plot)) {
                    // Skip plots that are already in a region,
                    // otherwise create a region by flood-fill
                    let mut region = Region {
                        plant,
                        plots: vec![plot],
                    };
                    let mut candidates = same_plant_neighbors(&grid, &plot, &plant);
                    while let Some(neighbor) = candidates.pop() {
                        if region.plots.contains(&neighbor) {
                            continue;
                        }
                        region.plots.push(neighbor);
                        let neighbors = same_plant_neighbors(&grid, &neighbor, &plant)
                            .into_iter()
                            .filter(|neighbor| !region.plots.contains(neighbor));
                        candidates.extend(neighbors);
                    }
                    regions.push(region);
                }
            }
        }
        Farm { regions }
    }
}

fn same_plant_neighbors(
    grid: &VecTileGrid<Plant>,
    curr_plot: &Plot,
    curr_plant: &Plant,
) -> Vec<Plot> {
    curr_plot
        .crosswise_neighbors()
        .filter(|next_plot| {
            grid.char_at(next_plot)
                .is_some_and(|next_plant| next_plant == curr_plant)
        })
        .collect()
}

impl Debug for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} plots @ {:?}", self.plant, self.plots)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
AAAA
BBCD
BBCC
EEEC
";

    const EXAMPLE2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

    const EXAMPLE3: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    const EXAMPLE4: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

    const EXAMPLE5: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

    #[test]
    fn test_part1_example1() {
        assert_eq!(140, solve_part1(EXAMPLE1));
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(772, solve_part1(EXAMPLE2));
    }

    #[test]
    fn test_part1_example3() {
        assert_eq!(1930, solve_part1(EXAMPLE3));
    }

    #[test]
    fn test_part1() {
        assert_eq!(1_424_472, solve_part1(INPUT));
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(80, solve_part2(EXAMPLE1));
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(436, solve_part2(EXAMPLE2));
    }

    #[test]
    fn test_part2_example3() {
        assert_eq!(1206, solve_part2(EXAMPLE3));
    }

    #[test]
    fn test_part2_example4() {
        assert_eq!(236, solve_part2(EXAMPLE4));
    }

    #[test]
    fn test_part2_example5() {
        assert_eq!(368, solve_part2(EXAMPLE5));
    }

    #[test]
    fn test_part2() {
        assert_eq!(870_202, solve_part2(INPUT));
    }
}
