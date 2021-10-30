const PUZZLE_INPUT: usize = 3_004_953;

pub(crate) fn day19_part1() -> usize {
    index_of_elf_who_gets_all_the_presents_part1(PUZZLE_INPUT)
}

pub(crate) fn day19_part2() -> usize {
    index_of_elf_who_gets_all_the_presents_part2(PUZZLE_INPUT)
}

enum Method {
    #[allow(unused)]
    Naive,
    Fast,
}
fn index_of_elf_who_gets_all_the_presents_part1(initial_elf_count: usize) -> usize {
    let method = Method::Fast;
    match method {
        Method::Naive => {
            let mut elves: Vec<_> = (0..initial_elf_count).into_iter().collect();
            let mut turn_idx = 0;
            while elves.len() > 1 {
                let remove_idx = (turn_idx + 1) % elves.len();
                elves.remove(remove_idx);
                if remove_idx > turn_idx {
                    turn_idx += 1;
                }
                turn_idx %= elves.len();
            }
            elves[0] + 1 // convert to 1-based index
        }
        Method::Fast => {
            // in any round, the even-numbered remaining elves get robbed.
            // if the remaining count is odd, the last elf will rob the current round's first elf,
            // and reduce the count to an even number.
            // if the remaining count is even, the last elf is robbed

            let mut elf_count = initial_elf_count;
            let mut leftmost = 1; // first using a 1-based index
            let mut iterations = 0;
            while elf_count > 1 {
                if elf_count % 2 == 1 {
                    // odd -> the last elf robs the leftmost elf, so leftmost moves to the right
                    leftmost += 2usize.pow(iterations + 1);
                }
                // println!("elf count {} left {} right {}", elf_count, left, right);
                elf_count /= 2;
                iterations += 1;
            }
            leftmost
        }
    }
}
fn index_of_elf_who_gets_all_the_presents_part2(initial_elf_count: usize) -> usize {
    let method = Method::Fast;
    match method {
        Method::Naive => {
            // The vec contains the original 0-based index of each elf
            let mut elves: Vec<_> = (0..initial_elf_count).into_iter().collect();
            let mut turn_idx = 0;
            while elves.len() > 1 {
                let offset = elves.len() / 2;
                let remove_idx = (turn_idx + offset) % elves.len();
                elves.remove(remove_idx);
                if remove_idx > turn_idx {
                    turn_idx += 1;
                }
                turn_idx %= elves.len();
            }
            elves[0] + 1 // convert to 1-based index
        }
        Method::Fast => {
            // There's a pattern to the winning elf:
            // 1 elf:   1

            // 2 elves: 1 // enumeration up to half the elf count
            // 3 elves: 3 // skip 2 pattern for the next third

            // 4 elves: 1 // enumeration up to half the elf count
            // 5 elves: 2
            // 6 elves: 3
            // 7 elves: 5 // skip 2 pattern for the next third
            // 8 elves: 7
            // 9 elves: 9

            // 10 to 18 elves: 1 to 9 wins // enumeration up to half the elf count
            // 19 to 27 elves: 11, 13, 15, 17, 19, 21, 23, 25, 27 wins // skip 2 pattern

            // 28 to 54 elves: 1 to 27 wins // enumeration up to half the elf count
            // 55 to 81 elves: 29, 31, 33, … 81 wins // skip 2 pattern
            if initial_elf_count == 1 {
                return 1;
            }
            let power = (initial_elf_count as f64).log(3.0).ceil() as u32;
            // Now 3 ^ power >= initial_elf_count
            // Find out if we're in the middle third (enumeration part) or upper third (skip-2 part)
            let third = 3usize.pow(power - 1);
            let above_third = initial_elf_count - third;
            if above_third <= third {
                // middle third -> enumeration pattern
                above_third
            } else {
                // upper third -> skip 2 pattern
                2 * above_third - third // == third + 2 * (above_third - third)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_5_elves() {
        assert_eq!(3, index_of_elf_who_gets_all_the_presents_part1(5));
    }

    #[test]
    fn part1_example_6_elves() {
        assert_eq!(5, index_of_elf_who_gets_all_the_presents_part1(6));
    }

    #[test]
    fn part1_example_7_elves() {
        assert_eq!(7, index_of_elf_who_gets_all_the_presents_part1(7));
    }

    #[test]
    fn part1_example_11_elves() {
        assert_eq!(7, index_of_elf_who_gets_all_the_presents_part1(11));
    }

    #[test] // naive version takes 19 minutes, fast version 1 ms
    fn part1() {
        assert_eq!(1_815_603, day19_part1());
    }

    #[test]
    fn part2_example_1_elf() {
        assert_eq!(1, index_of_elf_who_gets_all_the_presents_part2(1));
    }

    #[test]
    fn part2_example_2_elves() {
        assert_eq!(1, index_of_elf_who_gets_all_the_presents_part2(2));
    }
    #[test]
    fn part2_example_3_elves() {
        assert_eq!(3, index_of_elf_who_gets_all_the_presents_part2(3));
    }

    #[test]
    fn part2_example_4_to_6_elves() {
        assert_eq!(1, index_of_elf_who_gets_all_the_presents_part2(4));
        assert_eq!(2, index_of_elf_who_gets_all_the_presents_part2(5));
        assert_eq!(3, index_of_elf_who_gets_all_the_presents_part2(6));
    }
    #[test]
    fn part2_example_7_to_9_elves() {
        assert_eq!(5, index_of_elf_who_gets_all_the_presents_part2(7));
        assert_eq!(7, index_of_elf_who_gets_all_the_presents_part2(8));
        assert_eq!(9, index_of_elf_who_gets_all_the_presents_part2(9));
    }

    #[test]
    fn part2_example_10_to_18_elves() {
        assert_eq!(1, index_of_elf_who_gets_all_the_presents_part2(10));
        assert_eq!(2, index_of_elf_who_gets_all_the_presents_part2(11));
        assert_eq!(3, index_of_elf_who_gets_all_the_presents_part2(12));
        assert_eq!(4, index_of_elf_who_gets_all_the_presents_part2(13));
        assert_eq!(5, index_of_elf_who_gets_all_the_presents_part2(14));
        assert_eq!(6, index_of_elf_who_gets_all_the_presents_part2(15));
        assert_eq!(7, index_of_elf_who_gets_all_the_presents_part2(16));
        assert_eq!(8, index_of_elf_who_gets_all_the_presents_part2(17));
        assert_eq!(9, index_of_elf_who_gets_all_the_presents_part2(18));
    }
    #[test]
    fn part2_example_19_to_27_elves() {
        assert_eq!(11, index_of_elf_who_gets_all_the_presents_part2(19));
        assert_eq!(13, index_of_elf_who_gets_all_the_presents_part2(20));
        assert_eq!(15, index_of_elf_who_gets_all_the_presents_part2(21));
        assert_eq!(17, index_of_elf_who_gets_all_the_presents_part2(22));
        assert_eq!(19, index_of_elf_who_gets_all_the_presents_part2(23));
        assert_eq!(21, index_of_elf_who_gets_all_the_presents_part2(24));
        assert_eq!(23, index_of_elf_who_gets_all_the_presents_part2(25));
        assert_eq!(25, index_of_elf_who_gets_all_the_presents_part2(26));
        assert_eq!(27, index_of_elf_who_gets_all_the_presents_part2(27));
    }

    #[test]
    fn part2_example_28_to_54_elves() {
        assert_eq!(1, index_of_elf_who_gets_all_the_presents_part2(28));
        assert_eq!(2, index_of_elf_who_gets_all_the_presents_part2(29));
        assert_eq!(3, index_of_elf_who_gets_all_the_presents_part2(30));
        assert_eq!(4, index_of_elf_who_gets_all_the_presents_part2(31));
        assert_eq!(5, index_of_elf_who_gets_all_the_presents_part2(32));
        assert_eq!(6, index_of_elf_who_gets_all_the_presents_part2(33));
        assert_eq!(7, index_of_elf_who_gets_all_the_presents_part2(34));
        assert_eq!(8, index_of_elf_who_gets_all_the_presents_part2(35));
        assert_eq!(9, index_of_elf_who_gets_all_the_presents_part2(36));
        assert_eq!(10, index_of_elf_who_gets_all_the_presents_part2(37));
        assert_eq!(11, index_of_elf_who_gets_all_the_presents_part2(38));
        assert_eq!(12, index_of_elf_who_gets_all_the_presents_part2(39));
        assert_eq!(13, index_of_elf_who_gets_all_the_presents_part2(40));
        assert_eq!(14, index_of_elf_who_gets_all_the_presents_part2(41));
        assert_eq!(15, index_of_elf_who_gets_all_the_presents_part2(42));
        assert_eq!(16, index_of_elf_who_gets_all_the_presents_part2(43));
        assert_eq!(17, index_of_elf_who_gets_all_the_presents_part2(44));
        assert_eq!(18, index_of_elf_who_gets_all_the_presents_part2(45));
        assert_eq!(19, index_of_elf_who_gets_all_the_presents_part2(46));
        assert_eq!(20, index_of_elf_who_gets_all_the_presents_part2(47));
        assert_eq!(21, index_of_elf_who_gets_all_the_presents_part2(48));
        assert_eq!(22, index_of_elf_who_gets_all_the_presents_part2(49));
        assert_eq!(23, index_of_elf_who_gets_all_the_presents_part2(50));
        assert_eq!(24, index_of_elf_who_gets_all_the_presents_part2(51));
        assert_eq!(25, index_of_elf_who_gets_all_the_presents_part2(52));
        assert_eq!(26, index_of_elf_who_gets_all_the_presents_part2(53));
        assert_eq!(27, index_of_elf_who_gets_all_the_presents_part2(54));
    }
    #[test]
    fn part2_example_55_to_57_and_81_elves() {
        assert_eq!(29, index_of_elf_who_gets_all_the_presents_part2(55));
        assert_eq!(31, index_of_elf_who_gets_all_the_presents_part2(56));
        assert_eq!(33, index_of_elf_who_gets_all_the_presents_part2(57));

        assert_eq!(81, index_of_elf_who_gets_all_the_presents_part2(81));
    }

    #[test] // naive version takes 12 minutes, fast version 1 ms
    fn part2() {
        assert_eq!(1_410_630, day19_part2());
    }
}
