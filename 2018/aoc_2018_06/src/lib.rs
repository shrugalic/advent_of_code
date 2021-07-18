use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};

#[cfg(test)]
mod tests;

type X = usize;
type Y = usize;
#[derive(PartialEq, Eq, Hash, Clone)]
struct Loc {
    x: X,
    y: Y,
}

impl<T: AsRef<str>> From<T> for Loc {
    fn from(s: T) -> Self {
        if let Some((x, y)) = s.as_ref().split_once(", ") {
            Loc {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        } else {
            panic!("Illegal coord {}", s.as_ref());
        }
    }
}

impl Debug for Loc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Display for Loc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

type Distance = usize;
type Index = usize;
impl Loc {
    fn min(coords: &[Loc]) -> Loc {
        Loc {
            x: coords.iter().map(|c| c.x).min().unwrap(),
            y: coords.iter().map(|c| c.y).min().unwrap(),
        }
    }
    fn max(coords: &[Loc]) -> Loc {
        Loc {
            x: coords.iter().map(|c| c.x).max().unwrap(),
            y: coords.iter().map(|c| c.y).max().unwrap(),
        }
    }
    fn distance_to(&self, x: X, y: Y) -> Distance {
        self.x.max(x) - self.x.min(x) + self.y.max(y) - self.y.min(y)
    }

    fn distances_from_loc_to_coords(x: X, y: Y, coords: &[Loc]) -> Vec<(Distance, Index)> {
        coords
            .iter()
            .enumerate()
            .map(|(idx, coord)| (coord.distance_to(x, y), idx))
            .collect()
    }

    fn matches_row_or_col(&self, other_x: X, other_y: Y) -> bool {
        self.x == other_x || self.y == other_y
    }
}

pub fn size_of_largest_finite_area(input: Vec<String>) -> usize {
    let coords: Vec<_> = input.iter().map(Loc::from).collect();
    let (min, max) = (Loc::min(&coords), Loc::max(&coords));
    // println!("min = {}, max = {}", min, max);

    // Calculate the manhattan distances from all locations within the min/max rectangle
    // to the closest coordinate. It will be None if it's equally close to multiple locations.
    let closest_coord_idx_by_loc = indices_of_closest_coordinate(&coords, &min, &max);
    // Remove coordinate indices of infinite areas
    let finite_area_coord_indices =
        remove_infinite_areas(coords, min, max, &closest_coord_idx_by_loc);
    // println!("Finite area coord indices = {:?}", finite_area_coord_indices);

    let mut count_by_index: HashMap<Index, usize> = HashMap::new();
    closest_coord_idx_by_loc.iter().for_each(|(_, idx)| {
        if let Some(idx) = idx {
            if finite_area_coord_indices.contains(idx) {
                *count_by_index.entry(*idx).or_insert(0) += 1;
            }
        }
    });
    // println!("{:?}", count_by_index);
    *count_by_index
        .iter()
        .max_by_key(|(_idx, count)| *count)
        .unwrap()
        .1
}

fn indices_of_closest_coordinate(
    coords: &[Loc],
    min: &Loc,
    max: &Loc,
) -> HashMap<(X, Y), Option<Index>> {
    let mut closest_coord_idx_by_loc = HashMap::new();
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let distances: Vec<(Distance, Index)> =
                Loc::distances_from_loc_to_coords(x, y, &coords);
            let loc_idx: Option<Index> = index_of_closest_coord(&distances);
            closest_coord_idx_by_loc.insert((x, y), loc_idx);
        }
    }
    closest_coord_idx_by_loc
}

fn remove_infinite_areas(
    coords: Vec<Loc>,
    min: Loc,
    max: Loc,
    closest_coord_idx_by_pos: &HashMap<(X, Y), Option<Index>>,
) -> HashSet<Index> {
    // Border locations that are closest to any coordinates belong to infinite areas
    let border_indices: HashSet<Index> = closest_coord_idx_by_pos
        .iter()
        .filter(|((x, y), _)| min.matches_row_or_col(*x, *y) || max.matches_row_or_col(*x, *y))
        .filter_map(|(_, possible_idx)| possible_idx.as_ref())
        .cloned()
        .collect();
    coords
        .into_iter()
        .enumerate()
        .filter(|(idx, _)| !border_indices.contains(idx))
        .map(|(idx, _)| idx)
        .collect()
}

fn index_of_closest_coord(distances: &[(Distance, Index)]) -> Option<Index> {
    let closest: &(Distance, Index) = distances.iter().min().unwrap();
    let another_is_just_as_close = distances
        .iter()
        .any(|other| other.0 == closest.0 && other.1 != closest.1);
    if another_is_just_as_close {
        None
    } else {
        Some(closest.1)
    }
}

pub fn size_of_area_with_max_total_distance_to_all_coords(
    input: Vec<String>,
    total: Distance,
) -> usize {
    let coords: Vec<_> = input.iter().map(Loc::from).collect();
    let (min, max) = (Loc::min(&coords), Loc::max(&coords));
    count_locations_with_sum_of_distances_to_all_cords_within_total(&coords, &min, &max, total)
}

fn count_locations_with_sum_of_distances_to_all_cords_within_total(
    coords: &[Loc],
    min: &Loc,
    max: &Loc,
    total: Distance,
) -> usize {
    (min.y..=max.y)
        .flat_map(|y| (min.x..=max.x).map(move |x| (x, y)))
        .filter(|(x, y)| sum_of_distances_to_coords(*x, *y, &coords) < total)
        .count()
}

fn sum_of_distances_to_coords(x: X, y: Y, coords: &[Loc]) -> Distance {
    Loc::distances_from_loc_to_coords(x, y, &coords)
        .iter()
        .map(|(dist, _idx)| dist)
        .sum::<Distance>()
}
