type Mass = isize;

pub fn fuel_needed_for(mass: &Mass) -> Mass {
    let fuel = (mass / 3) - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + fuel_needed_for(&fuel)
    }
}

pub(crate) fn sum_of_fuel_needed_for(masses: &[Mass]) -> Mass {
    masses.iter().map(fuel_needed_for).sum()
}

pub(crate) fn day01input() -> Vec<Mass> {
    vec![
        95815, 58493, 77277, 57491, 124211, 134530, 86842, 63308, 139649, 75958, 74312, 63413,
        128293, 118123, 108576, 105474, 50366, 63203, 119792, 147054, 110863, 51551, 101243,
        108123, 108229, 76988, 126344, 81759, 74582, 131239, 143408, 53126, 134275, 142797, 61548,
        104641, 134200, 103371, 67804, 53892, 94285, 115017, 61553, 66873, 103186, 108708, 71366,
        63572, 137981, 72784, 140697, 125710, 121386, 131305, 61645, 81485, 82042, 148145, 75070,
        72671, 146981, 124797, 85756, 62383, 147575, 56740, 103299, 63511, 145914, 114995, 73657,
        118481, 105351, 102848, 118796, 139936, 112388, 80794, 128850, 92493, 65409, 60445, 124267,
        110438, 145208, 96697, 116439, 71484, 71588, 89813, 81525, 88200, 86443, 79786, 131067,
        105919, 126045, 135292, 117451, 67730,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_needed_for_mass_12() {
        assert_eq!(fuel_needed_for(&12), 2);
    }

    #[test]
    fn fuel_needed_for_mass_14() {
        assert_eq!(fuel_needed_for(&14), 2);
    }

    #[test]
    fn fuel_needed_for_mass_1969() {
        assert_eq!(fuel_needed_for(&1969), 966);
    }

    #[test]
    fn fuel_needed_for_mass_100756() {
        assert_eq!(fuel_needed_for(&100756), 50346);
    }

    #[test]
    fn calc_fuel_sum_of_12_and_14() {
        assert_eq!(sum_of_fuel_needed_for(&[12, 14]), 4);
    }

    #[test]
    fn calc_actual_fuel_sum() {
        assert_eq!(sum_of_fuel_needed_for(&day01input()), 4943994);
    }
}
