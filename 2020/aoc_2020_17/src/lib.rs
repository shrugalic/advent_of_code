mod pocket_dimension_map_3d;
mod pocket_dimension_map_4d;
mod pocket_dimension_vec;
mod tests;

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::ops::RangeInclusive;

#[derive(PartialEq, Debug, Copy, Clone)]
enum State {
    Active,
    Inactive,
}

trait PocketDimension<POS> {
    fn active_cube_count(&self) -> usize;
    fn get_state_at(&self, pos: &POS) -> State;
    fn is_active(&self, pos: &POS) -> bool;
    fn set_state_at(&mut self, pos: &POS, state: State);
    fn safe_neighbors_of(&self, pos: &POS) -> Vec<POS>;
    fn active_neighbor_count_of(&self, pos: &POS) -> usize;
    fn offsets() -> Vec<POS>;
}

trait ExecutableCycle {
    fn execute_cycle(self) -> Self;
    fn trim(self) -> Self;
}

type Coord3 = (isize, isize, isize);

fn offsets_3d() -> Vec<Coord3> {
    let range: RangeInclusive<isize> = -1..=1;
    let mut offsets = vec![];
    for z in range.clone() {
        for y in range.clone() {
            for x in range.clone() {
                if z != 0 || y != 0 || x != 0 {
                    offsets.push((z, y, x))
                }
            }
        }
    }
    offsets
}

impl State {
    fn to_char(self) -> char {
        match self {
            State::Active => '#',
            State::Inactive => '.',
        }
    }
}

impl From<char> for State {
    fn from(c: char) -> Self {
        match c {
            '.' => State::Inactive,
            '#' => State::Active,
            _ => panic!("Invalid char {}", c),
        }
    }
}
