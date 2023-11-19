use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input/day22.txt");

pub(crate) fn day22_part1() -> usize {
    ReactorCore::from(INPUT).turned_on_cubes_within_initialization_region()
}

pub(crate) fn day22_part2() -> usize {
    ReactorCore::from(INPUT).turned_on_cubes_anywhere()
}

type Instruction = (bool, Cuboid);
#[derive(Debug, PartialEq)]
struct ReactorCore {
    instructions: Vec<Instruction>,
}
impl ReactorCore {
    fn turned_on_cubes_within_initialization_region(&mut self) -> usize {
        let mut cubes = HashMap::new();
        for (state, cuboid) in &self.instructions {
            for x in cuboid.ranges[0].initial_only() {
                for y in cuboid.ranges[1].initial_only() {
                    for z in cuboid.ranges[2].initial_only() {
                        *cubes.entry(Vector { x, y, z }).or_default() = *state;
                    }
                }
            }
        }
        cubes.values().filter(|on| **on).count()
    }
    // Neal Wu solution takes ~90s
    fn turned_on_cubes_anywhere_2(&self) -> usize {
        let mut xs = HashSet::new();
        let mut ys = HashSet::new();
        let mut zs = HashSet::new();
        for (_, cuboid) in &self.instructions {
            xs.insert(*cuboid.ranges[0].start());
            xs.insert(*cuboid.ranges[0].end() + 1);
            ys.insert(*cuboid.ranges[1].start());
            ys.insert(*cuboid.ranges[1].end() + 1);
            zs.insert(*cuboid.ranges[2].start());
            zs.insert(*cuboid.ranges[2].end() + 1);
        }
        let mut xs: Vec<_> = xs.into_iter().collect();
        let mut ys: Vec<_> = ys.into_iter().collect();
        let mut zs: Vec<_> = zs.into_iter().collect();
        xs.sort_unstable();
        ys.sort_unstable();
        zs.sort_unstable();
        // println!("{} Xs: {:?}", xs.len(), xs);
        // println!("{} Ys: {:?}", ys.len(), ys);
        // println!("{} Zs: {:?}", zs.len(), zs);
        let mut cuboids = vec![vec![vec![false; zs.len()]; ys.len()]; xs.len()];

        let get_index = |coords: &[Coord], wanted: Coord| coords.binary_search(&wanted).unwrap();
        for (on, cuboid) in &self.instructions {
            let x_start = get_index(&xs, *cuboid.ranges[0].start());
            let x_end = get_index(&xs, *cuboid.ranges[0].end() + 1);
            let y_start = get_index(&ys, *cuboid.ranges[1].start());
            let y_end = get_index(&ys, *cuboid.ranges[1].end() + 1);
            let z_start = get_index(&zs, *cuboid.ranges[2].start());
            let z_end = get_index(&zs, *cuboid.ranges[2].end() + 1);

            // println!("instruction {} {:?}", on, cuboid);
            // for x in x_start..x_end {
            //     for y in y_start..y_end {
            //         for z in z_start..z_end {
            //             // println!("{} {} {} = {}", x, y, z, on);
            //             cuboids[x][y][z] = *on;
            //         }
            //     }
            // }
            for area in cuboids.iter_mut().skip(x_start).take(x_end - x_start) {
                for row in area.iter_mut().skip(y_start).take(y_end - y_start) {
                    for value in row.iter_mut().skip(z_start).take(z_end - z_start) {
                        // println!("{} {} {} = {}", x, y, z, on);
                        *value = *on;
                    }
                }
            }
        }

        let mut count = 0_usize;
        for (i, x) in xs.windows(2).enumerate() {
            for (j, y) in ys.windows(2).enumerate() {
                for (k, z) in zs.windows(2).enumerate() {
                    // println!("{} {} {} = {}", i, j, k, cuboids[i][j][k]);
                    if cuboids[i][j][k] {
                        count += (x[1] - x[0]) as usize
                            * (y[1] - y[0]) as usize
                            * (z[1] - z[0]) as usize;
                    }
                }
            }
        }
        count
    }
    fn turned_on_cubes_anywhere(&self) -> usize {
        let mut counts: HashMap<Cuboid, isize> = HashMap::new();
        for (turn_on, current) in self.instructions.iter() {
            // Find overlaps with previous cuboids
            let mut overlaps: HashMap<Cuboid, isize> = HashMap::new();
            for (previous, previous_count) in &counts {
                if let Some(overlap) = current.overlap(previous) {
                    *overlaps.entry(overlap).or_default() -= *previous_count;
                }
            }
            if *turn_on {
                *counts.entry(current.clone()).or_default() += 1;
            }
            // Overlapping cuboids will have to be subtracted, otherwise we'd count them twice
            for (sub, sub_count) in overlaps {
                *counts.entry(sub).or_default() += sub_count;
            }
        }
        // Optionally remove 0 counts, if only to clean up
        // counts = counts.into_iter().filter(|(_, cnt)| *cnt != 0).collect();

        // println!("{} counts", counts.len());
        // for (cube, count) in &counts {
        //     println!(
        //         "{} cuboids of expanse {}, total {}",
        //         count,
        //         cube.expanse(),
        //         count * cube.expanse() as isize
        //     );
        // }

        // Sum up all the counts
        counts
            .into_iter()
            .filter(|(_, cnt)| *cnt != 0)
            .map(|(cuboid, cnt)| cnt * cuboid.expanse() as isize)
            .sum::<isize>() as usize
    }
    fn instruction(line: &str) -> Instruction {
        let (turn_on, ranges) = line.split_once(' ').unwrap();
        (turn_on == "on", Cuboid::from(ranges))
    }
}
impl From<&str> for ReactorCore {
    fn from(input: &str) -> Self {
        let instructions = input.trim().lines().map(ReactorCore::instruction).collect();
        ReactorCore { instructions }
    }
}

trait Range {
    fn initial_only(&self) -> Self;
    fn overlap(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
    fn expanse(&self) -> usize;
}
impl Range for RangeInclusive<Coord> {
    fn initial_only(&self) -> RangeInclusive<Coord> {
        max(-50, *self.start())..=min(50, *self.end())
    }
    fn overlap(&self, other: &Self) -> Option<Self> {
        if self.start() <= other.end() && other.start() <= self.end() {
            Some(max(*self.start(), *other.start())..=min(*self.end(), *other.end()))
        } else {
            None
        }
    }
    fn expanse(&self) -> usize {
        (self.end() - self.start() + 1) as usize
    }
}

trait ToRange {
    fn to_range(&self) -> RangeInclusive<Coord>;
}
impl ToRange for &str {
    fn to_range(&self) -> RangeInclusive<Coord> {
        let (_, range) = self.split_once("=").unwrap();
        let (start, end) = range.split_once("..").unwrap();
        start.parse().unwrap()..=end.parse().unwrap()
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct Cuboid {
    ranges: [RangeInclusive<Coord>; 3],
}
impl Cuboid {
    fn overlap(&self, other: &Cuboid) -> Option<Cuboid> {
        let mut overlaps: Vec<_> = (0..3)
            .into_iter()
            .filter_map(|i| self.ranges[i].overlap(&other.ranges[i]))
            .collect();
        if overlaps.len() == 3 {
            let z = overlaps.remove(2);
            let y = overlaps.remove(1);
            let x = overlaps.remove(0);
            Some(Cuboid { ranges: [x, y, z] })
        } else {
            None
        }
    }
    fn expanse(&self) -> usize {
        self.ranges.iter().map(Range::expanse).product()
    }
}
impl From<&str> for Cuboid {
    fn from(ranges: &str) -> Self {
        let ranges: Vec<_> = ranges.split(',').collect();
        Cuboid {
            ranges: [
                ranges[0].to_range(),
                ranges[1].to_range(),
                ranges[2].to_range(),
            ],
        }
    }
}

type Coord = isize;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Vector {
    x: Coord,
    y: Coord,
    z: Coord,
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1_SMALL_EXAMPLE: &str = "\
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

    const PART1_LARGER_EXAMPLE: &str = "\
on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    const PART2_EXAMPLE: &str = "\
on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";

    #[test]
    fn part1_example() {
        assert_eq!(
            39,
            ReactorCore::from(PART1_SMALL_EXAMPLE).turned_on_cubes_within_initialization_region()
        );
    }

    #[test]
    fn part1_larger_example() {
        assert_eq!(
            590_784,
            ReactorCore::from(PART1_LARGER_EXAMPLE).turned_on_cubes_within_initialization_region()
        );
    }
    #[test]
    fn part1() {
        assert_eq!(576_028, day22_part1());
    }

    #[test]
    fn test_range_no_overlap() {
        assert_eq!(None, "x=1..3".to_range().overlap(&"x=4..7".to_range()));
    }

    #[test]
    fn test_range_overlaps_end() {
        assert_eq!(
            Some("x=3..4".to_range()),
            "x=1..4".to_range().overlap(&"x=3..7".to_range())
        );
    }

    #[test]
    fn test_range_overlaps_start() {
        assert_eq!(
            Some("x=3..4".to_range()),
            "x=3..7".to_range().overlap(&"x=1..4".to_range())
        );
    }

    #[test]
    fn test_range_overlap_enclosing() {
        assert_eq!(
            Some("x=3..4".to_range()),
            "x=1..7".to_range().overlap(&"x=3..4".to_range())
        );
    }

    #[test]
    fn test_range_overlap_enclosed() {
        assert_eq!(
            Some("x=3..4".to_range()),
            "x=3..4".to_range().overlap(&"x=1..7".to_range())
        );
    }

    #[test]
    fn test_range_overlap_identical() {
        assert_eq!(
            Some("x=0..0".to_range()),
            "x=0..0".to_range().overlap(&"x=0..0".to_range())
        );
    }

    #[test]
    fn test_cuboid_no_overlap() {
        assert_eq!(
            None,
            Cuboid::from("x=0..4,y=0..4,z=0..4").overlap(&Cuboid::from("x=5..6,y=5..6,z=5..6"))
        );
    }

    #[test]
    fn test_cuboid_overlaps() {
        assert_eq!(
            Some(Cuboid::from("on x=1..1,y=0..0,z=0..0")),
            Cuboid::from("on x=0..1,y=0..0,z=0..0")
                .overlap(&Cuboid::from("on x=1..2,y=0..0,z=0..0"))
        );
        assert_eq!(
            Some(Cuboid::from("x=3..4,y=3..4,z=3..4")),
            Cuboid::from("x=0..4,y=0..4,z=0..4").overlap(&Cuboid::from("x=3..5,y=3..5,z=3..5"))
        );
    }

    #[test]
    fn test_cuboid_overlap_enclosing() {
        assert_eq!(
            Some(Cuboid::from("x=1..3,y=1..3,z=1..3")),
            Cuboid::from("x=0..4,y=0..4,z=0..4").overlap(&Cuboid::from("x=1..3,y=1..3,z=1..3"))
        );
    }

    #[test]
    fn test_cuboid_overlap_enclosed() {
        assert_eq!(
            Some(Cuboid::from("x=1..3,y=1..3,z=1..3")),
            Cuboid::from("x=1..3,y=1..3,z=1..3").overlap(&Cuboid::from("x=0..4,y=0..4,z=0..4"))
        );
    }

    #[test]
    fn test_cuboid_expanse() {
        assert_eq!(3 * 3 * 3, Cuboid::from("x=1..3,y=1..3,z=1..3").expanse());
        assert_eq!(3 * 4 * 5, Cuboid::from("x=1..3,y=2..5,z=3..7").expanse());
    }

    #[test]
    fn part2_single_overlap() {
        let input = "\
on x=0..1,y=0..0,z=0..0
on x=1..2,y=0..0,z=0..0";
        assert_eq!(3, ReactorCore::from(input).turned_on_cubes_anywhere());
    }

    #[test]
    fn part2_dual_overlap() {
        let input = "\
on x=0..1,y=0..0,z=0..0
on x=1..2,y=0..0,z=0..0
on x=1..1,y=0..1,z=0..0";
        assert_eq!(4, ReactorCore::from(input).turned_on_cubes_anywhere());
    }

    #[test]
    fn part2_triple_overlap() {
        let input = "\
on x=0..1,y=0..0,z=0..0
on x=1..2,y=0..0,z=0..0
on x=1..1,y=0..1,z=0..0
on x=1..1,y=-1..0,z=0..0";
        assert_eq!(5, ReactorCore::from(input).turned_on_cubes_anywhere());
    }

    #[test]
    fn part2_triple_overlap_with_sub() {
        let input = "\
on x=0..1,y=0..0,z=0..0
on x=1..2,y=0..0,z=0..0
off x=1..1,y=0..1,z=0..0
on x=1..1,y=-1..0,z=0..0";
        assert_eq!(4, ReactorCore::from(input).turned_on_cubes_anywhere());
    }

    #[test]
    fn part2_part1_example() {
        assert_eq!(
            39,
            ReactorCore::from(PART1_SMALL_EXAMPLE).turned_on_cubes_anywhere()
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            2_758_514_936_282_235,
            ReactorCore::from(PART2_EXAMPLE).turned_on_cubes_anywhere()
        );
    }

    #[test]
    fn part2() {
        assert_eq!(1_387_966_280_636_636, day22_part2());
    }
}
