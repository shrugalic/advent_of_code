use crate::parse;
use std::collections::HashMap;
use std::ops::Range;

const INPUT: &str = include_str!("../input/day04.txt");

type SleepPhase = Range<usize>;
type SleepPhases = Vec<SleepPhase>;

pub(crate) fn day4_part1() -> usize {
    strategy_one(&parse(INPUT))
}
pub(crate) fn day4_part2() -> usize {
    strategy_two(&parse(INPUT))
}

fn strategy_one(input: &[&str]) -> usize {
    let shifts = split_into_guard_shifts(input);
    let sleep_phases_by_guard_id = sleep_phases_by_guard_id(shifts);
    let guard = find_longest_sleeping_guard(&sleep_phases_by_guard_id);
    // println!("Longest sleeping guard = {:?}", guard);

    let (minute, _count) = most_slept_minute_and_count(guard.1);
    *guard.0 * minute
}

fn strategy_two(input: &[&str]) -> usize {
    let shifts = split_into_guard_shifts(input);
    let sleep_phases_by_guard_id = sleep_phases_by_guard_id(shifts);
    let guard = find_most_often_sleeping_guard(&sleep_phases_by_guard_id);
    let (minute, _) = most_slept_minute_and_count(guard.1);
    *guard.0 * minute
}

fn find_most_often_sleeping_guard(
    sleep_phases_by_guard_id: &HashMap<usize, SleepPhases>,
) -> (&usize, &SleepPhases) {
    sleep_phases_by_guard_id
        .iter()
        // .inspect(|(i, p)| {
        //     if p.is_empty() {
        //         println!("{} has no sleep_phases", i);
        //     }
        // })
        .max_by_key(|guard| {
            let (_minute, count) = most_slept_minute_and_count(guard.1);
            count
        })
        .unwrap()
}

fn most_slept_minute_and_count(sleep_phases: &[SleepPhase]) -> (usize, usize) {
    let mut count_by_sleep_minute = HashMap::new();
    sleep_phases.iter().for_each(|r| {
        r.clone()
            .for_each(|minute| *count_by_sleep_minute.entry(minute).or_insert(0) += 1)
    });
    let (minute, count) = count_by_sleep_minute
        .iter()
        .max_by_key(|entry| entry.1)
        // Default to (0, 0) for guards without sleep phases, such as #1087, #1787 and #2657
        .unwrap_or((&0, &0));
    (*minute, *count)
}

fn sleep_phases_by_guard_id(shifts: Vec<Vec<String>>) -> HashMap<usize, SleepPhases> {
    let mut sleep_phases_by_guard_id = HashMap::new();
    shifts.iter().for_each(|shift| {
        let (id, sleep_phases) = id_and_sleep_phases_from(shift);
        sleep_phases_by_guard_id
            .entry(id)
            .or_insert_with(Vec::new)
            .extend(sleep_phases.iter().cloned());
    });
    sleep_phases_by_guard_id
}

fn split_into_guard_shifts(input: &[&str]) -> Vec<Vec<String>> {
    let mut sorted = input.to_vec();
    sorted.sort();
    let joined = sorted.join("\n");
    joined
        .split("Guard #")
        .skip(1)
        .map(|shift| shift.split('\n').map(str::to_string).collect())
        .collect()
}

fn find_longest_sleeping_guard(
    sleep_phases_by_guard_id: &HashMap<usize, SleepPhases>,
) -> (&usize, &SleepPhases) {
    let longest_sleeping_guard = sleep_phases_by_guard_id
        .iter()
        .max_by_key(|(_id, sleep_phases)| {
            sleep_phases
                .iter()
                .map(|phase| phase.end - phase.start)
                .sum::<usize>()
        })
        .unwrap();
    longest_sleeping_guard
}

fn id_and_sleep_phases_from(shift: &[String]) -> (usize, SleepPhases) {
    let (id, shift) = shift.split_first().unwrap();
    let id: usize = id.trim_end_matches(" begins shift").parse().unwrap();
    // let _date = shift
    //     .get(0)
    //     .unwrap()
    //     .split(' ')
    //     .next()
    //     .unwrap()
    //     .trim_start_matches('[');
    let sleep_phases = shift
        .windows(2)
        .step_by(2)
        .map(|sleep_recs| minute_range_from(sleep_recs))
        .collect();
    (id, sleep_phases)
}

fn minute_range_from(sleep_recs: &[String]) -> SleepPhase {
    assert_eq!(sleep_recs.len(), 2);
    assert!(sleep_recs[0].contains("falls asleep"));
    assert!(sleep_recs[1].contains("wakes up"));
    // A pair of lines looks like this:
    // [1518-11-01 00:05] falls asleep
    // [1518-11-01 00:25] wakes up
    let t1 = extract_minutes(&sleep_recs[0]);
    let t2 = extract_minutes(&sleep_recs[1]);
    t1..t2
}

fn extract_minutes(line: &str) -> usize {
    line.split(':')
        .nth(1)
        .unwrap()
        .split(']')
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    const EXAMPLE1: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn part_1_example_1() {
        assert_eq!(240, strategy_one(&parse(EXAMPLE1)));
    }

    #[test]
    fn part_1() {
        assert_eq!(65489, strategy_one(&parse(INPUT)));
    }

    #[test]
    fn part_2_example_1() {
        assert_eq!(4455, strategy_two(&parse(EXAMPLE1)));
    }

    #[test]
    fn part_2() {
        assert_eq!(3852, strategy_two(&parse(INPUT)));
    }
}
