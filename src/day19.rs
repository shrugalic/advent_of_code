use std::collections::{HashMap, HashSet, VecDeque};
use FirstAxis::*;

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
    fn beacon_count(mut self) -> usize {
        self.beacon_count2().0
    }
    fn max_manhattan_distance(mut self) -> usize {
        self.beacon_count2().1
    }
    fn beacon_count2(mut self) -> (usize, usize) {
        let count = self.scanners.len();
        let mut distances_to_scanner0 = vec![[0, 0, 0]; count];

        // The first scanner saves as the reference for position and orientation
        let reference = self.scanners.remove(0);
        let mut unique_beacons: HashSet<Vector> = reference.beacons.iter().cloned().collect();

        let mut aligned = VecDeque::new();
        aligned.push_back(reference);
        let mut unaligned: Vec<Scanner> = self.scanners.drain(..).collect();

        while let Some(reference) = aligned.pop_front() {
            let mut still_unaligned = vec![];
            while let Some(mut scanner) = unaligned.pop() {
                // if !tried.insert((reference.id, scanner.id)) {
                //     continue;
                // }
                println!(
                    "Trying to align {} to reference {}",
                    scanner.id, reference.id
                );

                /*
                if unique_beacons.len() == 65 && scanner.id == 2 {
                    let mut all: Vec<_> = aligned
                        .iter()
                        .flat_map(|s| s.beacons.iter())
                        .cloned()
                        .collect();
                    all.extend(&mut reference.beacons.clone().into_iter());
                    let all_len = all.len();
                    let all_set = all.into_iter().collect::<HashSet<_>>();
                    let all_set_len = all_set.len();
                    println!("unique {}/{}", all_set_len, all_len);
                    let all: Vec<_> = all_set.into_iter().collect();
                    if let Some((orientation, translation)) = scanner.find_alignment_with(&all) {
                        //
                        println!("found something")
                    } else {
                        println!("did not find anything")
                    }
                }
                 */

                if let Some((orientation, translation)) =
                    scanner.find_alignment_with(&reference.beacons)
                {
                    let aligned_beacons = scanner.aligned_beacons(&orientation);
                    let translated_beacons =
                        Scanner::translate_beacons(&aligned_beacons, translation);
                    let new_beacons: HashSet<Vector> = translated_beacons.iter().cloned().collect();
                    let ref_beacons: HashSet<Vector> = reference.beacons.iter().cloned().collect();
                    let common: Vec<_> = ref_beacons.intersection(&new_beacons).collect();
                    if common.len() == 12 {
                        scanner.beacons = translated_beacons;
                        distances_to_scanner0[scanner.id] = translation;

                        /*
                        if scanner.id == 1 && reference.id == 0 {
                            println!("------- checking 1 and 0 ------");
                            let expected: Vec<Vector> = vec![
                                [-618, -824, -621],
                                [-537, -823, -458],
                                [-447, -329, 318],
                                [404, -588, -901],
                                [544, -627, -890],
                                [528, -643, 409],
                                [-661, -816, -575],
                                [390, -675, -793],
                                [423, -701, 434],
                                [-345, -311, 381],
                                [459, -707, 401],
                                [-485, -357, 347],
                            ];
                            assert!(expected.iter().all(|pos| common.contains(&pos)));
                        }
                        if scanner.id == 3 && reference.id == 1 {
                            println!("------- checking 3 and 1 ------");
                            let expected: Vec<Vector> = vec![
                                [432, -2009, 850],
                                [408, -1815, 803],
                                [396, -1931, -563],
                                [568, -2007, -577],
                                [-518, -1681, -600],
                                [-601, -1648, -643],
                                [497, -1838, -617],
                                [-499, -1607, -770],
                                // the following are already in 4-1
                                [-739, -1745, 668],
                                [534, -1912, 768],
                                [-687, -1600, 576],
                                [-635, -1737, 486],
                            ];
                            assert!(expected.iter().all(|pos| common.contains(&pos)));
                        }
                        if scanner.id == 4 && reference.id == 1 {
                            println!("------- checking 4 and 1 ------");
                            let expected: Vec<Vector> = vec![
                                [459, -707, 401],
                                [-739, -1745, 668],
                                [-485, -357, 347],
                                [432, -2009, 850],
                                [528, -643, 409],
                                [423, -701, 434],
                                [-345, -311, 381],
                                [408, -1815, 803],
                                [534, -1912, 768],
                                [-687, -1600, 576],
                                [-447, -329, 318],
                                [-635, -1737, 486],
                            ];
                            assert!(expected.iter().all(|pos| common.contains(&pos)));
                        }
                        */

                        let prev_len = unique_beacons.len();
                        let new_len = new_beacons.len();
                        for beacon in new_beacons {
                            if !unique_beacons.insert(beacon) {
                                //
                            }
                        }
                        println!(
                            "Inserted {}/{} beacons for a total of {}",
                            unique_beacons.len() - prev_len,
                            new_len,
                            unique_beacons.len()
                        );

                        /*
                        if unique_beacons.len() == 65 {
                            let all: Vec<Vector> = vec![
                                [-892, 524, 684],
                                [-876, 649, 763],
                                [-838, 591, 734],
                                [-789, 900, -551],
                                [-739, -1745, 668],
                                [-706, -3180, -659],
                                [-697, -3072, -689],
                                [-689, 845, -530],
                                [-687, -1600, 576],
                                [-661, -816, -575],
                                [-654, -3158, -753],
                                [-635, -1737, 486],
                                [-631, -672, 1502],
                                [-624, -1620, 1868],
                                [-620, -3212, 371],
                                [-618, -824, -621],
                                [-612, -1695, 1788],
                                [-601, -1648, -643],
                                [-584, 868, -557],
                                [-537, -823, -458],
                                [-532, -1715, 1894],
                                [-518, -1681, -600],
                                [-499, -1607, -770],
                                [-485, -357, 347],
                                [-470, -3283, 303],
                                [-456, -621, 1527],
                                [-447, -329, 318],
                                [-430, -3130, 366],
                                [-413, -627, 1469],
                                [-345, -311, 381],
                                [-36, -1284, 1171],
                                [-27, -1108, -65],
                                [7, -33, -71],
                                [12, -2351, -103],
                                [26, -1119, 1091],
                                [346, -2985, 342],
                                [366, -3059, 397],
                                [377, -2827, 367],
                                [390, -675, -793],
                                [396, -1931, -563],
                                [404, -588, -901],
                                [408, -1815, 803],
                                [423, -701, 434],
                                [432, -2009, 850],
                                [443, 580, 662],
                                [455, 729, 728],
                                [456, -540, 1869],
                                [459, -707, 401],
                                [465, -695, 1988],
                                [474, 580, 667],
                                [496, -1584, 1900],
                                [497, -1838, -617],
                                [527, -524, 1933],
                                [528, -643, 409],
                                [534, -1912, 768],
                                [544, -627, -890],
                                [553, 345, -567],
                                [564, 392, -477],
                                [568, -2007, -577],
                                [605, -1665, 1952],
                                [612, -1593, 1893],
                                [630, 319, -379],
                                [686, -3108, -505],
                                [776, -3184, -501],
                                [846, -3110, -434],
                                [1135, -1161, 1235],
                                [1243, -1093, 1063],
                                [1660, -552, 429],
                                [1693, -557, 386],
                                [1735, -437, 1738],
                                [1749, -1800, 1813],
                                [1772, -405, 1572],
                                [1776, -675, 371],
                                [1779, -442, 1789],
                                [1780, -1548, 337],
                                [1786, -1538, 337],
                                [1847, -1591, 415],
                                [1889, -1729, 1762],
                                [1994, -1805, 1792],
                            ];
                            let missing: Vec<_> = all
                                .into_iter()
                                .filter(|v| !unique_beacons.contains(v))
                                .collect();
                            println!("missing {:?}", missing);

                            // these are the unique scanner 2 beacons that aren't shared
                            // let missing = vec![
                            //     [1135, -1161, 1235],
                            //     [1243, -1093, 1063],
                            //     [1660, -552, 429],
                            //     [1693, -557, 386],
                            //     [1735, -437, 1738],
                            //     [1749, -1800, 1813],
                            //     [1772, -405, 1572],
                            //     [1776, -675, 371],
                            //     [1779, -442, 1789],
                            //     [1780, -1548, 337],
                            //     [1786, -1538, 337],
                            //     [1847, -1591, 415],
                            //     [1889, -1729, 1762],
                            //     [1994, -1805, 1792]
                            // ];
                        }
                         */

                        aligned.push_back(scanner);
                        println!("{}/{} aligned", aligned.len() + 1, count); // +1 is for reference
                    } else {
                        println!("--------- only {} intersecting beacons", common.len());
                        still_unaligned.push(scanner);
                    }
                } else {
                    // println!("this does happen and should not"); // TODO
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

        println!("distances to scanner 0: {:?}", distances_to_scanner0);
        let distances_between_scanners: Vec<_> = Scanner::diffs_between(&distances_to_scanner0)
            .into_iter()
            .map(|(d, _)| d)
            .collect();
        println!("distances {:?}", distances_between_scanners);
        let max_manhattan_distance = distances_between_scanners
            .into_iter()
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
        println!("Scanner {} has {} beacons", id, beacons.len());
        Scanner { id, beacons }
    }
}

impl Scanner {
    fn find_alignment_with(&self, ref_beacons: &[Vector]) -> Option<(Orientation, Vector)> {
        for orientation in &Orientation::all() {
            let own_beacons = self.aligned_beacons(orientation);
            if let Some(translations) = Scanner::shared_beacons(&own_beacons, ref_beacons) {
                if let Some((translation, count)) = translations.iter().max_by_key(|(_, c)| *c) {
                    if *count >= 12 {
                        println!("translation for {} diffs found {:?}", count, translation);
                        return Some((*orientation, *translation));
                    } else {
                        panic!("all over the place translations {:?}", translations);
                    }
                }
            }
        }
        None
    }
    fn shared_beacons(
        own_beacons: &[Vector],
        ref_beacons: &[Vector],
    ) -> Option<HashMap<Vector, usize>> {
        // To align two scanners, we collect the diffs between each scanner's beacon positions,
        // and look for overlap. While beacon positions are relative to the scanner's position,
        // the diff between individual beacon position is absolute, and if two scanners share
        // beacons, they'll also share diffs
        let mut ref_map = Scanner::diffs_between(ref_beacons);
        let mut other_map = Scanner::diffs_between(own_beacons);
        // Apparently HashSet.keys() don't have an .intersection(…) function, hence this collect
        let shared_diffs: HashSet<_> = ref_map
            .keys()
            .cloned()
            .collect::<HashSet<Vector>>()
            .intersection(&other_map.keys().cloned().collect::<HashSet<Vector>>())
            .cloned()
            .collect();

        if shared_diffs.len() < 12 {
            // There must be at least 12 shared points.
            // 12 shared points have 11+10+9+8+7+6+5+4+3+2+1 = 66 shared connections TODO?
            None
        } else {
            // println!("intersection count {}", shared_diffs.len());
            let mut translations: HashMap<Vector, usize> = HashMap::new();
            for distance in shared_diffs {
                let own = ref_map.remove(&distance).unwrap();
                let other = other_map.remove(&distance).unwrap();

                assert_eq!(own.len(), 2);
                assert_eq!(other.len(), 2);
                let mut translation = Scanner::diff_between(&own[0], &other[0]);
                if translation == Scanner::diff_between(&own[1], &other[1]) {
                    assert_ne!(
                        Scanner::diff_between(&own[0], &other[1]),
                        Scanner::diff_between(&own[1], &other[0])
                    )
                } else {
                    translation = Scanner::diff_between(&own[0], &other[1]);
                    if translation != Scanner::diff_between(&own[1], &other[0]) {
                        unreachable!("maybe?")
                    }
                }
                // println!(
                //     "own {:?} other {:?} own diff {:?} other diff {:?} translation {:?}",
                //     own,
                //     other,
                //     Scanner::diff_between(&own[1], &own[0]),
                //     Scanner::diff_between(&other[1], &other[0]),
                //     translation
                // );

                *translations.entry(translation).or_default() += 1;
            }
            Some(translations)
        }
    }
    fn aligned_beacons(&self, orientation: &Orientation) -> Vec<Vector> {
        self.beacons
            .iter()
            .map(|pos| orientation.align(*pos))
            .collect()
    }
    fn translate_beacons(beacons: &[Vector], trans: Vector) -> Vec<Vector> {
        beacons
            .iter()
            .map(|pos| [pos[0] + trans[0], pos[1] + trans[1], pos[2] + trans[2]])
            .collect()
    }
    fn diff_between(a: &Vector, b: &Vector) -> Vector {
        [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
    }
    fn diffs_between(beacons: &[Vector]) -> HashMap<Vector, Vec<Vector>> {
        let mut distances: HashMap<Vector, Vec<Vector>> = HashMap::new();
        for (i, a) in beacons.iter().enumerate().take(beacons.len() - 1) {
            for b in beacons.iter().skip(i + 1) {
                let diff = Scanner::diff_between(a, b);
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
enum FirstAxis {
    X,
    Y,
    Z,
}
// #[derive(Debug, Copy, Clone)]
// struct Orientation {
//     first: FirstAxis,  // The order of the x, y and z coordinates
//     multi: [isize; 3], // Multiplier of coordinates x, y and z
// }
// impl Orientation {
//     fn align(&self, mut pos: Vector) -> Vector {
//         match self.first {
//             X => {}
//             Y => pos.rotate_left(1),
//             Z => pos.rotate_right(1),
//         }
//         (0..3).for_each(|i| pos[i] *= self.multi[i]);
//         pos
//     }
//     fn all() -> Vec<Orientation> {
//         let mut orientations = vec![];
//         for x in [1, -1] {
//             for y in [1, -1] {
//                 for z in [1, -1] {
//                     let multi = [x, y, z];
//                     for first in [X, Y, Z] {
//                         orientations.push(Orientation { first, multi })
//                     }
//                 }
//             }
//         }
//         orientations
//     }
// }
#[derive(Debug, Copy, Clone)]
struct Orientation {
    index: [usize; 3], // The order of the x, y and z coordinates
    multi: [isize; 3], // Multiplier of x, y and z coordinates
}
impl Orientation {
    // fn all() -> Vec<Orientation> {
    //     let mut orientations = vec![];
    //     for x in [1, -1] {
    //         for y in [1, -1] {
    //             for z in [1, -1] {
    //                 let multi = [x, y, z];
    //                 for index in [
    //                     [0, 1, 2],
    //                     [0, 2, 1],
    //                     [1, 0, 2],
    //                     [1, 2, 0],
    //                     [2, 0, 1],
    //                     [2, 1, 0],
    //                 ] {
    //                     orientations.push(Orientation { index, multi })
    //                 }
    //             }
    //         }
    //     }
    //     println!(
    //         "{} unique orientations",
    //         orientations.iter().cloned().collect::<HashSet<_>>().len()
    //     );
    //     orientations
    // }
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
        assert_eq!(1, day19_part2());
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
