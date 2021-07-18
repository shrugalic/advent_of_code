use std::ops::RangeInclusive;
extern crate rayon;
use rayon::prelude::*;

fn bla() {
    let v = vec![1];
}

trait IsValidPassword {
    fn is_valid_password(&self) -> bool;
}
impl IsValidPassword for usize {
    fn is_valid_password(&self) -> bool {
        self.is_six_digits_long() && self.has_double_digits() && self.has_no_decreasing_digits()
    }
}

trait IsSixDigitsLong {
    fn is_six_digits_long(&self) -> bool;
}
impl IsSixDigitsLong for usize {
    fn is_six_digits_long(&self) -> bool {
        (100000usize..=999999usize).contains(self)
    }
}
/// exact double digits that are not part of a larger same-digit-group, such as a triple
trait HasDoubleDigits {
    fn has_double_digits(&self) -> bool;
}
impl HasDoubleDigits for usize {
    fn has_double_digits(&self) -> bool {
        let v = self.to_string().chars().collect::<Vec<char>>();
        (v[0] == v[1] && v[1] != v[2])
            || (v[0] != v[1] && v[1] == v[2] && v[2] != v[3])
            || (v[1] != v[2] && v[2] == v[3] && v[3] != v[4])
            || (v[2] != v[3] && v[3] == v[4] && v[4] != v[5])
            || (v[3] != v[4] && v[4] == v[5])
    }
}

trait HasNoDecreasingDigits {
    fn has_no_decreasing_digits(&self) -> bool;
}
impl HasNoDecreasingDigits for usize {
    fn has_no_decreasing_digits(&self) -> bool {
        self.to_string()
            .chars()
            .collect::<Vec<char>>() // intermediate vec to make slice's windows(2) is available
            .windows(2)
            .all(|pair| pair[0] <= pair[1])
    }
}

#[cfg(test)]
mod tests {
    use crate::{HasDoubleDigits, HasNoDecreasingDigits, IsSixDigitsLong, IsValidPassword};
    use rayon::prelude::*;

    #[test]
    fn has_double_digits() {
        assert_eq!(122345usize.has_double_digits(), true);
    }
    #[test]
    fn misses_double_digits() {
        assert_eq!(123456usize.has_double_digits(), false);
    }

    #[test]
    fn is_six_digits_long() {
        assert_eq!(122345usize.is_six_digits_long(), true);
    }
    #[test]
    fn is_not_six_digits_long() {
        assert_eq!(12345usize.is_six_digits_long(), false);
    }

    #[test]
    fn has_no_decreasing_digits() {
        assert_eq!(123456usize.has_no_decreasing_digits(), true);
    }
    #[test]
    fn has_decreasing_digits() {
        assert_eq!(123450usize.has_no_decreasing_digits(), false);
    }

    #[test]
    fn test_range() {
        let range = 172851..=675869usize;
        let mut v = vec![];
        for pw in range {
            v.push(pw);
        }
        let valid_pw_count = v
            .par_iter()
            .filter(|pw| pw.is_valid_password())
            .collect::<Vec<&usize>>()
            .len();

        println!("Number of valid passwords in range = {}", valid_pw_count);
        assert_eq!(1, 0);
    }
}
