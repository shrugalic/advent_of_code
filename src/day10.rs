use line_reader::read_file_to_lines;
use std::collections::HashMap;

pub(crate) fn day10_part1() -> usize {
    let input = read_file_to_lines("input/day10.txt");
    let mut bot_traders = BotTraders::from(input);
    bot_traders.bot_responsible_for_handling(61, 17)
}

pub(crate) fn day10_part2() -> usize {
    let input = read_file_to_lines("input/day10.txt");
    let mut bot_traders = BotTraders::from(input);
    bot_traders.trade(&|_| false)
}

type Number = usize;
type Bot = Number;
type Value = Number;
type Output = Number;

struct BotTraders {
    values_by_bot: HashMap<Bot, Vec<Value>>,
    values_by_output: HashMap<Output, Vec<Value>>,
    recipients_by_bot: HashMap<Bot, Recipients>,
}
impl BotTraders {
    fn bot_responsible_for_handling(&mut self, chip_a: Number, chip_b: Number) -> Bot {
        let stop_filter = |values: &Vec<Value>| {
            values[0] == chip_a && values[1] == chip_b || values[0] == chip_b && values[1] == chip_a
        };
        self.trade(&stop_filter)
    }
    fn trade(&mut self, stop_filter: &dyn Fn(&Vec<Value>) -> bool) -> Bot {
        while let Some((bot, values)) = self
            .values_by_bot
            .iter()
            .find(|(bot, values)| values.len() == 2 && self.recipients_by_bot.contains_key(bot))
        {
            let bot = *bot;
            // println!("bot {} has values {:?}", bot, values);
            if stop_filter(values) {
                return bot;
            }
            let values = self.values_by_bot.remove(&bot).unwrap();

            if let Some(recipients) = self.recipients_by_bot.get(&bot) {
                let (lo, hi) = if values[0] < values[1] {
                    (values[0], values[1])
                } else {
                    (values[1], values[0])
                };
                match recipients.lo {
                    Recipient::Bot(b) => self.values_by_bot.entry(b),
                    Recipient::Output(o) => self.values_by_output.entry(o),
                }
                .or_insert_with(Vec::new)
                .push(lo);
                match recipients.hi {
                    Recipient::Bot(b) => self.values_by_bot.entry(b),
                    Recipient::Output(o) => self.values_by_output.entry(o),
                }
                .or_insert_with(Vec::new)
                .push(hi);
            }
        }
        self.values_by_output.get(&0).unwrap().first().unwrap()
            * self.values_by_output.get(&1).unwrap().first().unwrap()
            * self.values_by_output.get(&2).unwrap().first().unwrap()
    }
}
impl From<Vec<String>> for BotTraders {
    fn from(input: Vec<String>) -> Self {
        let (initial_values, bots): (Vec<_>, Vec<_>) =
            input.into_iter().partition(|l| l.starts_with("value"));

        let mut values_by_bot: HashMap<Bot, Vec<Value>> = HashMap::new();
        initial_values.into_iter().for_each(|s| {
            // Example: value 2 goes to bot 171
            let s: Vec<_> = s.split_ascii_whitespace().collect();
            values_by_bot
                .entry(s[5].parse().unwrap())
                .or_insert_with(Vec::new)
                .push(s[1].parse().unwrap());
        });
        // println!("values {:?}", values);

        let recipients_by_bot: HashMap<Bot, Recipients> = bots
            .into_iter()
            .map(|s| {
                // Example: bot 51 gives low to bot 19 and high to bot 176
                // Index:   0   1  2     3   4  5   6  7   8    9  10  11
                let s: Vec<_> = s.split_ascii_whitespace().collect();
                (s[1].parse::<usize>().unwrap(), Recipients::from(&s[3..=11]))
            })
            .collect();
        // println!("bots {:?}", bots);

        BotTraders {
            values_by_bot,
            values_by_output: HashMap::new(),
            recipients_by_bot,
        }
    }
}

#[derive(Debug)]
struct Recipients {
    lo: Recipient,
    hi: Recipient,
}
impl From<&[&str]> for Recipients {
    fn from(s: &[&str]) -> Self {
        // Example: low to bot 14 and high to bot 111
        // Index:   0   1  2   3  4   5    6  7   8
        Recipients {
            lo: Recipient::from(&s[0..=3]),
            hi: Recipient::from(&s[5..=8]),
        }
    }
}

#[derive(Debug)]
enum Recipient {
    Bot(Bot),
    Output(Value),
}
impl From<&[&str]> for Recipient {
    fn from(s: &[&str]) -> Self {
        // Examples: bot 0 gives low to output 2
        //           high to output 0
        // Index:    0    1  2      3
        let number = s[3].parse().unwrap();
        match s[2] {
            "bot" => Recipient::Bot(number),
            "output" => Recipient::Output(number),
            _ => panic!("Invalid Recipient '{:?}'", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE: &str = "\
value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";

    #[test]
    fn part1_example_initial() {
        let input = read_str_to_lines(EXAMPLE);
        let mut bot_traders = BotTraders::from(input);
        assert_eq!(2, bot_traders.bot_responsible_for_handling(2, 5));
    }

    #[test]
    fn part1_example_after_first_distribution() {
        let input = read_str_to_lines(EXAMPLE);
        let mut bot_traders = BotTraders::from(input);
        assert_eq!(1, bot_traders.bot_responsible_for_handling(2, 3));
    }

    #[test]
    fn part1_example_after_second_distribution() {
        let input = read_str_to_lines(EXAMPLE);
        let mut bot_traders = BotTraders::from(input);
        assert_eq!(1, bot_traders.bot_responsible_for_handling(2, 3));
    }

    #[test]
    fn part1() {
        assert_eq!(86, day10_part1());
    }

    #[test]
    fn part2() {
        assert_eq!(67 * 11 * 31, day10_part2());
    }
}
