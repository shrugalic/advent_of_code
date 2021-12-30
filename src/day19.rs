use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: &str = include_str!("../input/day19.txt");

pub(crate) fn day19_part1() -> usize {
    System::from(INPUT).beacon_count()
}

pub(crate) fn day19_part2() -> usize {
    System::from(INPUT).max_manhattan_distance()
}

type Coordinate = isize;
type Vector = [Coordinate; 3];

#[derive(Debug)]
struct System {
    scanners: Vec<Scanner>,
}
impl From<&str> for System {
    fn from(input: &str) -> Self {
        let scanners = input.trim().split("\n\n").map(Scanner::from).collect();
        System { scanners }
    }
}
impl System {
    fn beacon_count(self) -> usize {
        self.align_beacons().0
    }
    fn max_manhattan_distance(self) -> usize {
        self.align_beacons().1
    }
    fn align_beacons(mut self) -> (usize, usize) {
        let mut distances_to_ref = vec![[0, 0, 0]; self.scanners.len()];

        // The first scanner saves as the reference for position and orientation
        let reference = self.scanners.remove(0);
        let mut unique_beacons: HashSet<Vector> = reference.beacons.iter().cloned().collect();

        let mut aligned = VecDeque::new();
        aligned.push_back(reference);
        let mut unaligned: Vec<Scanner> = self.scanners.drain(..).collect();

        while let Some(reference) = aligned.pop_front() {
            let mut still_unaligned = vec![];
            while let Some(mut scanner) = unaligned.pop() {
                // println!(
                //     "Trying to align {} to reference {}",
                //     scanner.id, reference.id
                // );

                if let Some(offset) = scanner.align_with(&reference.beacons) {
                    unique_beacons.extend(scanner.beacons.clone());
                    distances_to_ref[scanner.id] = offset;
                    aligned.push_back(scanner);
                    // +1 is for reference scanner 0, which is aligned by definition
                    // println!("{}/{} aligned", aligned.len() + 1, self.scanners.len());
                } else {
                    still_unaligned.push(scanner);
                }
            }
            unaligned.append(&mut still_unaligned);
            // At this point, there should be some scanners aligned to the current reference.
            // Those that are still unaligned have too few overlapping beacons.
            // Add the current reference to the back so others are tried first
            aligned.push_back(reference);

            if unaligned.is_empty() {
                break;
            }
        }

        // println!("distances to ref scanner 0: {:?}", distances_to_ref);
        let distances_between_scanners = Scanner::offsets_between(&distances_to_ref)
            .into_iter()
            .map(|(d, _)| d);
        // println!("distances {:?}", distances_between_scanners);
        let max_manhattan_distance = distances_between_scanners
            .map(|[a, b, c]| (a.abs() + b.abs() + c.abs()) as usize)
            .max()
            .unwrap();

        (unique_beacons.len(), max_manhattan_distance)
    }
}

#[derive(Debug)]
struct Scanner {
    id: usize,
    beacons: Vec<Vector>,
}
impl From<&str> for Scanner {
    fn from(lines: &str) -> Self {
        let to_position = |line: &str| {
            let pos: Vec<Coordinate> = line.split(',').map(|n| n.parse().unwrap()).collect();
            [pos[0], pos[1], pos[2]]
        };
        let mut lines = lines.trim().lines();
        let header = lines.next().unwrap();
        let id = header
            .trim_start_matches("--- scanner ")
            .trim_end_matches(" ---")
            .parse()
            .unwrap();
        let beacons: Vec<_> = lines.map(to_position).collect();
        // println!("Scanner {} has {} beacons", id, beacons.len());
        Scanner { id, beacons }
    }
}

impl Scanner {
    fn align_with(&mut self, ref_beacons: &[Vector]) -> Option<Vector> {
        for orientation in Orientation::all() {
            let mut offset_frequencies: HashMap<Vector, usize> = HashMap::new();
            let aligned_beacons = self.aligned_beacons(&orientation);
            for own_beacon in &aligned_beacons {
                for ref_beacon in ref_beacons {
                    let offset = Scanner::offset_between(ref_beacon, own_beacon);
                    *offset_frequencies.entry(offset).or_default() += 1;
                }
            }
            if let Some((offset, _)) = offset_frequencies
                .into_iter()
                .find(|(_, count)| *count >= 12)
            {
                self.beacons = Scanner::translate_beacons(&aligned_beacons, offset);
                return Some(offset);
            }
        }
        None
    }
    fn aligned_beacons(&self, orientation: &Orientation) -> Vec<Vector> {
        self.beacons
            .iter()
            .map(|pos| orientation.align(*pos))
            .collect()
    }
    fn translate_beacons(beacons: &[Vector], offset: Vector) -> Vec<Vector> {
        beacons
            .iter()
            .map(|pos| [pos[0] + offset[0], pos[1] + offset[1], pos[2] + offset[2]])
            .collect()
    }
    fn offset_between(a: &Vector, b: &Vector) -> Vector {
        [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
    }
    fn offsets_between(beacons: &[Vector]) -> HashMap<Vector, Vec<Vector>> {
        let mut distances: HashMap<Vector, Vec<Vector>> = HashMap::new();
        for (i, a) in beacons.iter().enumerate().take(beacons.len() - 1) {
            for b in beacons.iter().skip(i + 1) {
                let diff = Scanner::offset_between(a, b);
                let positions = distances.entry(diff).or_default();
                positions.push(*a);
                positions.push(*b);
            }
        }
        // n positions have n*(n-1)/2 connections. This might happen if positions aren't unique enough
        assert_eq!(distances.len(), beacons.len() * (beacons.len() - 1) / 2);
        distances
    }
}
#[derive(Debug, Copy, Clone)]
struct Orientation {
    index: [usize; 3], // The order of the x, y and z coordinates
    multi: [isize; 3], // Multiplier of x, y and z coordinates
}
impl Orientation {
    fn new(index: [usize; 3], multi: [isize; 3]) -> Self {
        Orientation { index, multi }
    }
    fn all() -> Vec<Orientation> {
        vec![
            // + x
            Orientation::new([0, 1, 2], [1, 1, 1]),
            Orientation::new([0, 2, 1], [1, 1, -1]),
            Orientation::new([0, 1, 2], [1, -1, -1]),
            Orientation::new([0, 2, 1], [1, -1, 1]),
            // - x
            Orientation::new([0, 2, 1], [-1, 1, 1]),
            Orientation::new([0, 1, 2], [-1, 1, -1]),
            Orientation::new([0, 2, 1], [-1, -1, -1]),
            Orientation::new([0, 1, 2], [-1, -1, 1]),
            // + y
            Orientation::new([1, 2, 0], [1, 1, 1]),
            Orientation::new([1, 0, 2], [1, 1, -1]),
            Orientation::new([1, 2, 0], [1, -1, -1]),
            Orientation::new([1, 0, 2], [1, -1, 1]),
            // - y
            Orientation::new([1, 0, 2], [-1, 1, 1]),
            Orientation::new([1, 2, 0], [-1, 1, -1]),
            Orientation::new([1, 0, 2], [-1, -1, -1]),
            Orientation::new([1, 2, 0], [-1, -1, 1]),
            // + z
            Orientation::new([2, 0, 1], [1, 1, 1]),
            Orientation::new([2, 1, 0], [1, 1, -1]),
            Orientation::new([2, 0, 1], [1, -1, -1]),
            Orientation::new([2, 1, 0], [1, -1, 1]),
            // - z
            Orientation::new([2, 1, 0], [-1, 1, 1]),
            Orientation::new([2, 0, 1], [-1, 1, -1]),
            Orientation::new([2, 1, 0], [-1, -1, -1]),
            Orientation::new([2, 0, 1], [-1, -1, 1]),
        ]
    }
    fn align(&self, mut pos: Vector) -> Vector {
        pos = [pos[self.index[0]], pos[self.index[1]], pos[self.index[2]]];
        (0..3).for_each(|i| pos[i] *= self.multi[i]);
        pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orientations() {
        let pos = [5, 6, -4];
        let mut orientations: Vec<_> = Orientation::all()
            .into_iter()
            .map(|o| o.align(pos))
            .collect();
        orientations.sort_unstable();
        assert_eq!(
            24,
            orientations
                .iter()
                .cloned()
                .collect::<HashSet<Vector>>()
                .len()
        );
        assert!(orientations.contains(&[5, 6, -4]));
        assert!(orientations.contains(&[-5, 4, -6]));
        assert!(orientations.contains(&[4, 6, 5]));
        assert!(orientations.contains(&[-4, -6, 5]));
        assert!(orientations.contains(&[-6, -4, -5]));
        let expected = vec![
            [-6, -5, 4],
            [-6, -4, -5],
            [-6, 4, 5],
            [-6, 5, -4],
            [-5, -6, -4],
            [-5, -4, 6],
            [-5, 4, -6],
            [-5, 6, 4],
            [-4, -6, 5],
            [-4, -5, -6],
            [-4, 5, 6],
            [-4, 6, -5],
            [4, -6, -5],
            [4, -5, 6],
            [4, 5, -6],
            [4, 6, 5],
            [5, -6, 4],
            [5, -4, -6],
            [5, 4, 6],
            [5, 6, -4],
            [6, -5, -4],
            [6, -4, 5],
            [6, 4, -5],
            [6, 5, 4],
        ];
        assert_eq!(expected, orientations);
    }

    #[test]
    fn part1_example() {
        let system = System::from(EXAMPLE);
        assert_eq!(79, system.beacon_count());
    }

    #[test]
    fn part1() {
        assert_eq!(398, day19_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(3621, System::from(EXAMPLE).max_manhattan_distance());
    }

    #[test]
    fn part2() {
        assert_eq!(10965, day19_part2());
    }

    const EXAMPLE: &str = "\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
";
}
