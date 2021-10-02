use line_reader::read_file_to_lines;
use std::collections::{HashMap, HashSet};

pub(crate) fn day19_part1() -> usize {
    let (replacements, molecule) = parse_day19_input();
    let dictionary = dictionary_from(sorted_atom_vec_from(&replacements));
    let replacements = replacements_vec_from(&replacements, &dictionary);
    let molecule = molecule_from(&molecule, &dictionary);
    results_of_one_replacement(&molecule, &replacements).len()
}

pub(crate) fn day19_part2() -> usize {
    let (replacements, medicine) = parse_day19_input();
    let dictionary = dictionary_from(sorted_atom_vec_from(&replacements));
    let replacements = replacements_vec_from(&replacements, &dictionary);
    let molecule = molecule_from(STARTING_MOLECULE, &dictionary);
    let medicine = molecule_from(&medicine, &dictionary);

    number_of_replacements(&molecule, &medicine, &replacements)
}

const STARTING_MOLECULE: &str = "e";

fn parse_day19_input() -> (Vec<String>, String) {
    let input = read_file_to_lines("input/day19.txt");
    let parts: Vec<_> = input.split(|element| element.is_empty()).collect();
    (parts[0].to_vec(), parts[1][0].to_string()) // (replacements, molecule)
}

type Atom = String;
type AtomIndex = usize;
type AtomVec = Vec<Atom>;
type AtomSet = HashSet<Atom>;
type Dictionary = HashMap<Atom, AtomIndex>;
type Molecule = Vec<AtomIndex>;
type Molecules = Vec<Molecule>;
type Replacements = Vec<Molecules>;
type Results = HashSet<Molecule>;

fn sorted_atom_vec_from(replacements: &[String]) -> AtomVec {
    let (sources, targets) = split_replacements(replacements);
    let sources: AtomSet = atom_set_from(&sources);
    let targets: AtomSet = atom_set_from(&targets);
    let mut targets_only: Vec<_> = targets.difference(&sources).into_iter().cloned().collect();
    targets_only.sort_unstable();

    let mut all_atoms = sort_to_vec(sources);
    all_atoms.append(&mut targets_only);

    all_atoms
}

fn replacements_vec_from(replacements: &[String], dictionary: &Dictionary) -> Replacements {
    let mut replacement_vec: Replacements = vec![Vec::new(); dictionary.len()];
    replacements.iter().for_each(|line| {
        let (from, to) = line.split_once(" => ").unwrap();
        let from_idx = *dictionary.get(from).unwrap();
        let to_indices = molecule_from(to, dictionary);
        replacement_vec[from_idx].push(to_indices);
    });
    replacement_vec
}

fn molecule_from(s: &str, dictionary: &Dictionary) -> Molecule {
    atom_vec_from(&[s])
        .iter()
        .map(|to| *dictionary.get(to).unwrap())
        .collect()
}

fn dictionary_from(atoms: AtomVec) -> Dictionary {
    atoms.into_iter().enumerate().map(|(v, k)| (k, v)).collect()
}

fn split_replacements(replacements: &[String]) -> (Vec<&str>, Vec<&str>) {
    replacements
        .iter()
        .map(|line| line.split_once(" => ").unwrap())
        .unzip()
}

fn atom_set_from<T: AsRef<str>>(molecules: &[T]) -> AtomSet {
    atom_set_from_v(atom_vec_from(molecules))
}

fn atom_set_from_v(atoms: AtomVec) -> AtomSet {
    atoms.into_iter().collect()
}

fn atom_vec_from<T: AsRef<str>>(molecules: &[T]) -> AtomVec {
    let mut atoms = vec![];
    let mut next_atom: Vec<char> = Vec::new();
    for molecule in molecules.iter() {
        for c in molecule.as_ref().chars().collect::<Vec<char>>() {
            // Lower case letter terminates an atom
            if c.is_lowercase() {
                next_atom.push(c);
            }
            // Insert complete atom if any
            if !next_atom.is_empty() {
                atoms.push(next_atom.drain(..).collect());
            }
            // Upper case letter starts a new atom
            if !c.is_lowercase() {
                next_atom.push(c);
            }
        }
        if !next_atom.is_empty() {
            atoms.push(next_atom.drain(..).collect());
        }
    }
    atoms
}

fn sort_to_vec(atoms: AtomSet) -> AtomVec {
    let mut atoms: AtomVec = atoms.into_iter().collect();
    atoms.sort_unstable();
    atoms
}

fn results_of_one_replacement(molecule: &[AtomIndex], replacements: &[Vec<Molecule>]) -> Results {
    let mut results = Results::new();
    for (pos, idx) in molecule.iter().enumerate() {
        replacements[*idx].iter().for_each(|r| {
            let mut result = molecule[..pos].to_vec();
            result.extend_from_slice(r);
            result.extend_from_slice(&molecule[pos + 1..]);
            results.insert(result);
        });
    }
    results
}

fn number_of_replacements(
    start: &[AtomIndex],
    target: &[AtomIndex],
    replacements: &[Vec<Molecule>],
) -> usize {
    replacement_count(start, target, replacements).unwrap()
}

fn replacement_count(
    start: &[AtomIndex],
    target: &[AtomIndex],
    replacements: &[Vec<Molecule>],
) -> Option<usize> {
    if start == target {
        Some(0)
    } else if start.len() > target.len() {
        None
    } else {
        results_of_one_replacement(start, replacements)
            .into_iter()
            .filter_map(|new_start| replacement_count(&new_start, target, replacements))
            .map(|c| c + 1)
            .min()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE1_REPLACEMENTS: &str = "\
H => HO
H => OH
O => HH";
    const EXAMPLE1_START: &str = "HOH";

    #[test]
    fn part1_example() {
        let replacements = read_str_to_lines(EXAMPLE1_REPLACEMENTS);
        let dictionary = dictionary_from(sorted_atom_vec_from(&replacements));
        let replacements = replacements_vec_from(&replacements, &dictionary);
        let molecule = molecule_from(EXAMPLE1_START, &dictionary);
        let results = results_of_one_replacement(&molecule, &replacements);
        assert_eq!(4, results.len());
    }

    #[test]
    fn part1() {
        assert_eq!(535, day19_part1());
    }

    const EXAMPLE2_REPLACEMENTS: &str = "\
e => H
e => O
H => HO
H => OH
O => HH";

    const EXAMPLE2_TARGET1: &str = "HOH";
    const EXAMPLE2_TARGET2: &str = "HOHOHO";

    #[test]
    fn part2_examples() {
        let replacements = read_str_to_lines(EXAMPLE2_REPLACEMENTS);
        let dictionary = dictionary_from(sorted_atom_vec_from(&replacements));
        let replacement_vec = replacements_vec_from(&replacements, &dictionary);

        let molecule = molecule_from(STARTING_MOLECULE, &dictionary);
        let target1 = molecule_from(EXAMPLE2_TARGET1, &dictionary);
        let target2 = molecule_from(EXAMPLE2_TARGET2, &dictionary);

        assert_eq!(
            3,
            number_of_replacements(&molecule, &target1, &replacement_vec)
        );
        assert_eq!(
            6,
            number_of_replacements(&molecule, &target2, &replacement_vec)
        );
    }

    #[test]
    fn test_atom_set_from() {
        assert_eq!(vec!["H", "O"], sort_to_vec(atom_set_from(&["HO"])));
        assert_eq!(vec!["H", "O", "e"], sort_to_vec(atom_set_from(&["eHOH"])));
        assert_eq!(
            vec!["Ar", "F", "Rn", "Si"],
            sort_to_vec(atom_set_from(&["SiRnFAr"]))
        );
    }

    #[test]
    fn atom_vec_from_replacements() {
        let replacements = read_str_to_lines("e => H");
        assert_eq!(vec!["e", "H"], sorted_atom_vec_from(&replacements));

        let replacements = read_str_to_lines(EXAMPLE2_REPLACEMENTS);
        assert_eq!(vec!["H", "O", "e"], sorted_atom_vec_from(&replacements));

        let (replacements, _molecule) = parse_day19_input();
        let atoms = sorted_atom_vec_from(&replacements);
        assert_eq!(
            vec![
                // These are source atoms (some of them might be targets too)
                "Al", "B", "Ca", "F", "H", "Mg", "N", "O", "P", "Si", "Th", "Ti", "e",
                // These are target-only atoms
                "Ar", "C", "Rn", "Y"
            ],
            atoms
        );
    }

    #[test]
    fn dictionary_from_replacements() {
        let expected_dictionary: Dictionary = [("H", 0), ("O", 1), ("e", 2)]
            .iter()
            .map(|(k, v)| (k.to_string(), *v as usize))
            .collect();
        let replacements = read_str_to_lines(EXAMPLE2_REPLACEMENTS);
        assert_eq!(
            expected_dictionary,
            dictionary_from(sorted_atom_vec_from(&replacements))
        );
    }

    #[test]
    fn replacement_vec_from_replacement_strings_and_dictionary() {
        let replacement_strings = read_str_to_lines(EXAMPLE2_REPLACEMENTS);
        let dictionary = dictionary_from(sorted_atom_vec_from(&replacement_strings));
        let replacement_vec = replacements_vec_from(&replacement_strings, &dictionary);
        assert_eq!(
            vec![
                vec![vec![0, 1], vec![1, 0]], // H => HO, H => OH
                vec![vec![0, 0]],             // O => HH
                vec![vec![0], vec![1]]        // e => H, e => O
            ],
            replacement_vec
        );
    }

    #[test]
    fn part2() {
        assert_eq!(0, day19_part2());
    }
}
