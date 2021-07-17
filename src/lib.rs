use std::collections::HashMap;
use std::ops::Range;

#[cfg(test)]
mod tests;

type SleepPhase = Range<usize>;
type SleepPhases = Vec<SleepPhase>;

pub fn strategy_one(input: &[String]) -> usize {
    let shifts = split_into_guard_shifts(input);
    let sleep_phases_by_guard_id = sleep_phases_by_guard_id(shifts);
    let guard = find_longest_sleeping_guard(&sleep_phases_by_guard_id);
    println!("Longest sleeping guard = {:?}", guard);

    let (minute, _count) = most_slept_minute_and_count(guard.1);
    *guard.0 * minute
}

pub fn strategy_two(input: &[String]) -> usize {
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
        .inspect(|(i, p)| {
            if p.is_empty() {
                println!("{} has no sleep_phases", i);
            }
        })
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

fn split_into_guard_shifts(input: &[String]) -> Vec<Vec<String>> {
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
