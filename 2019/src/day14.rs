use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

pub(crate) fn day14_part1() -> usize {
    let mut nf = NanoFactory::from(day14_puzzle_input());
    let target = Chemical::new(1, "FUEL");
    nf.count_reactant_to_make_wanted_product(&target, "ORE")
}

pub(crate) fn day14_part2() -> usize {
    // 158'482 ORE were needed for 1 FUEL, so the amount of fuel that can be made
    // with 1'000'000'000'000 is at least 1'000'000'000'000 / 158'482 = 6'309'864.8,
    // and likely more. Let's try a binary search to find the right amount
    let mut range = 6_309_864..10_000_000;
    let mut max_fuel = 0;
    while range.start + 1 < range.end {
        let mid = ((range.start + range.end) as f64 / 2.0).round() as usize;
        let fuel = Chemical::new(mid, "FUEL");
        let ore_count = NanoFactory::from(day14_puzzle_input())
            .count_reactant_to_make_wanted_product(&fuel, "ORE");
        match ore_count.cmp(&1_000_000_000_000usize) {
            Ordering::Less => {
                max_fuel = fuel.amount;
                range = mid..range.end;
            }
            Ordering::Equal => break,
            Ordering::Greater => {
                range = range.start..mid;
            }
        }
    }
    max_fuel
}
/// For example: 123 ORE
const INGREDIENT_PATTERN: &str = r"^((\d+) (\D+)(, )?)+$";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
/// Amount and name of a single ingredient, such as 123 ORE
struct Chemical {
    amount: usize,
    name: String,
}
impl From<&'static str> for Chemical {
    fn from(ingredient: &'static str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(INGREDIENT_PATTERN).unwrap();
        }
        if let Some(caps) = RE.captures(ingredient) {
            let count = caps[2].parse().unwrap();
            let name = caps[3].to_string();
            Chemical {
                amount: count,
                name,
            }
        } else {
            panic!("Unable to parse Ingredient from '{}'", ingredient);
        }
    }
}
impl Chemical {
    fn new(count: usize, name: &str) -> Self {
        Chemical {
            amount: count,
            name: name.to_string(),
        }
    }
    fn dependency_count(name: &str, formulae: &[Formula]) -> usize {
        formulae
            .iter()
            .filter(|other_formula| other_formula.reactants.contains_key(name))
            .count()
    }
}

#[derive(Debug, PartialEq, Clone)]
/// A formula, such as: 123 ORE, 456 WTR => 789 SAND
struct Formula {
    product: Chemical,
    // count by name
    reactants: HashMap<String, usize>,
}
impl From<&'static str> for Formula {
    fn from(formula: &'static str) -> Self {
        let parts: Vec<&str> = formula.split(" => ").collect();
        if parts.len() != 2 {
            panic!("More than one ' => ' in formula '{}'", formula);
        }

        let sources = parts[0]
            .split(", ")
            .map(|source| {
                let i = Chemical::from(source);
                (i.name, i.amount)
            })
            .collect();
        //        println!("sources = {:?}", sources);

        let target = Chemical::from(parts[1]);
        //        println!("target = {:?}", target);

        Formula {
            reactants: sources,
            product: target,
        }
    }
}
impl Formula {
    #[allow(unused)]
    fn new(reactants: Vec<Chemical>, product: Chemical) -> Self {
        Formula {
            reactants: Formula::reactant_list_to_map(reactants),
            product,
        }
    }
    #[allow(unused)]
    fn reactant_list_to_map(reactants: Vec<Chemical>) -> HashMap<String, usize> {
        reactants
            .iter()
            .map(|i| (i.name.clone(), i.amount))
            .collect()
    }
    /// Get the chemical that is depended on by the smallest number of the given formulae
    fn least_dependant_reactant(&self, formulae: &[Formula]) -> Chemical {
        self.reactants
            .iter()
            .map(|(name, &count)| Chemical::new(count, name))
            .min_by_key(|c| Chemical::dependency_count(&c.name, formulae))
            .unwrap()
    }
    /// Apply the given substitution to the reactants of this formula
    fn substitute_reactants(&mut self, substitution: &Formula) {
        // Remove the product to be substituted…
        if let Some(_removed_count) = self.reactants.remove(&substitution.product.name) {
            // …and put in the substitutes instead
            substitution
                .reactants
                .iter()
                // .inspect(|(name, count)| println!("  {} {}", count, name))
                .for_each(|(name, count)| {
                    let total = self.reactants.entry(name.clone()).or_insert(0);
                    *total += *count;
                });
        }
    }
}

#[derive(Debug, PartialEq)]
struct NanoFactory {
    formulae: Vec<Formula>,
}
impl NanoFactory {
    #[allow(unused)]
    fn using_formula(formula: Formula) -> Self {
        NanoFactory::using_formulae(&[formula])
    }
    #[allow(unused)]
    fn using_formulae(formulae: &[Formula]) -> Self {
        NanoFactory {
            formulae: formulae.to_vec(),
        }
    }
    fn formula_to_make(&self, wanted: &Chemical) -> Formula {
        if let Some(formula) = self
            .formulae
            .iter()
            .find(|&f| f.product.name == wanted.name)
        {
            let reactants = formula
                .reactants
                .iter()
                .map(|(name, &count)| {
                    let multiplier = if wanted.amount > formula.product.amount {
                        (wanted.amount as f64 / formula.product.amount as f64).ceil() as usize
                    } else {
                        1
                    };
                    (name.clone(), multiplier * count)
                })
                .collect();
            Formula {
                reactants,
                product: wanted.clone(),
            }
        } else {
            panic!("Missing formula to make {}", wanted.name);
        }
    }

    fn count_reactant_to_make_wanted_product(
        &mut self,
        wanted_product: &Chemical,
        wanted_reactant: &str,
    ) -> usize {
        let mut final_formula = self.formula_to_make(wanted_product);
        while final_formula.reactants.len() > 1
            || !final_formula.reactants.contains_key(wanted_reactant)
        {
            let least_dependent_reactant = final_formula.least_dependant_reactant(&self.formulae);
            let substitution_formula = self.formula_to_make(&least_dependent_reactant);
            final_formula.substitute_reactants(&substitution_formula);
            self.simplify_formulae(substitution_formula);
        }
        return *final_formula.reactants.get(wanted_reactant).unwrap();
    }
    fn simplify_formulae(&mut self, substitution: Formula) {
        // Substitute
        self.formulae
            .iter_mut()
            .filter(|formula| formula.reactants.contains_key(&substitution.product.name))
            .for_each(|f| {
                f.substitute_reactants(&substitution);
            });
        // Remove the formula itself
        if let Some(pos) = self
            .formulae
            .iter()
            .position(|f| f.product.name == substitution.product.name)
        {
            //            println!("Removing formula for {}", substitution.product.name);
            self.formulae.remove(pos);
        }
    }
}
impl From<&'static str> for NanoFactory {
    fn from(formulae: &'static str) -> Self {
        //        println!("{:?}", formulae);
        let sources_by_target = formulae.split('\n').map(Formula::from).collect();
        NanoFactory {
            formulae: sources_by_target,
        }
    }
}

fn day14_puzzle_input() -> &'static str {
    "1 FJFL, 1 BPVQN => 7 CMNH
6 FJFL, 2 KZJLT, 3 DZQJ => 2 NSPZ
11 TPZDN => 2 TNMC
1 NSPZ, 2 KQVL => 2 HPNWP
3 XHDVT => 9 LRBN
3 LRBN => 6 TPZDN
1 KPFLZ, 1 XVXCZ => 6 WHMLV
1 BDWQP, 1 JPGK, 1 MTWG => 5 GLHWQ
2 BGLTP, 1 HPNWP, 2 GLHWQ, 9 CRJZ, 22 QVQJ, 3 PHGWC, 1 BDWQP => 3 LKPNB
4 BDSB => 2 PNSD
2 BRJDJ, 13 THQR => 2 BGLTP
1 WHJKH, 2 JBTJ => 6 THQR
1 JBTJ => 9 WGVP
10 CTXHZ, 2 DGMN => 5 TNVC
7 LCSV, 1 LKPNB, 36 CMNH, 1 JZXPH, 20 DGJPN, 3 WDWB, 69 DXJKC, 3 WHJKH, 18 XSGP, 22 CGZL, 2 BNVB, 57 PNSD => 1 FUEL
13 CRCG, 8 NMQN => 1 JZXPH
2 FZVS, 2 ZPFBH => 9 SRPD
1 QPNTQ, 4 QVQJ, 1 XZKTG => 9 WDWB
6 SXZW => 5 FJFL
6 GVGZ => 6 ZPFBH
1 JPGK, 3 WDFXH, 22 FJFL => 7 BDSB
3 WHMLV => 4 JPGK
7 CGZL, 4 LRBN => 8 MTWG
11 SXZW, 33 ZTBFN => 4 XVXCZ
1 FZVS, 1 TNMC, 7 JPGK => 9 FLHW
2 XKFZ => 8 CGZL
5 WHMLV => 8 MQRS
1 QVSH, 6 TPZDN, 9 JQHCH => 2 BMNJ
3 CMNH, 10 XKFZ => 2 KQVL
119 ORE => 9 PSPQ
1 WGVP, 18 BRJDJ => 9 PHGWC
110 ORE => 6 NMQN
13 NMQN, 24 XVXCZ, 9 XHDVT => 6 KQVS
6 TNMC => 4 DXJKC
1 XZKTG => 8 WHJKH
1 KPFLZ, 1 LRBN, 7 KQVS => 9 JBTJ
1 XKFZ => 8 JVGD
152 ORE => 7 SXZW
1 BDWQP => 5 CTXHZ
2 JVGD, 8 DGMN, 2 MTWG => 6 QVQJ
1 KQVL => 2 BNVB
3 DZQJ, 37 MQRS => 4 CRJZ
1 KQVL, 5 WDFXH => 7 BDWQP
3 GVGZ => 3 BPVQN
4 PSPQ, 6 ZTBFN => 1 KPFLZ
34 FBTG => 9 XZKTG
14 TNMC, 4 FZVS, 3 MTWG => 9 KZJLT
157 ORE => 6 GVGZ
5 JVGD, 11 JPGK => 5 CRCG
1 SXZW, 1 NMQN => 3 XHDVT
1 FBTG => 5 JQHCH
3 WDFXH, 4 FZVS, 9 CGFML => 6 DZQJ
5 BDWQP, 3 TNVC, 7 SRPD, 1 WDFXH, 3 JQHCH, 4 QVQJ, 5 CRCG, 4 DGMN => 7 XSGP
1 KPFLZ, 3 TPZDN, 1 SRPD => 6 FBTG
1 WHMLV, 3 BDSB, 2 JVGD => 9 LCSV
13 XZKTG => 4 QVSH
1 XHDVT => 7 XKFZ
1 CMNH, 1 KQVS, 2 XVXCZ => 6 CGFML
6 FLHW => 4 BRJDJ
2 KQVL, 2 WGVP, 7 BMNJ, 11 KQVS, 1 HPNWP, 6 CRJZ => 4 DGJPN
2 DZQJ, 1 BDSB => 2 DGMN
1 XVXCZ, 4 MQRS => 3 WDFXH
5 FLHW, 10 JPGK, 1 XZKTG => 4 QPNTQ
2 LRBN => 9 FZVS
149 ORE => 8 ZTBFN"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_ingredient() {
        assert_eq!(Chemical::from("123 ORE"), Chemical::new(123, "ORE"));
    }
    #[test]
    fn single_ingredient_formula() {
        let formula = Formula::from("2 ORE => 1 FUEL");
        let reactants = Formula::reactant_list_to_map(vec![Chemical::new(2, "ORE")]);
        let product = Chemical::new(1, "FUEL");
        assert_eq!(formula, Formula { reactants, product });
    }
    #[test]
    fn dual_ingredient_formula() {
        let formula = Formula::from("1 ORE, 2 WTR => 3 FUEL");
        let reactants =
            Formula::reactant_list_to_map(vec![Chemical::new(1, "ORE"), Chemical::new(2, "WTR")]);
        let product = Chemical::new(3, "FUEL");
        assert_eq!(formula, Formula { reactants, product });
    }
    #[test]
    fn single_ingredient_factory() {
        let nf = NanoFactory::from("1 ORE => 1 FUEL");
        let target = Chemical::new(1, "FUEL");
        assert_eq!(
            nf,
            NanoFactory::using_formula(Formula::new(vec![Chemical::new(1, "ORE")], target))
        );
    }
    #[test]
    fn dual_ingredient_factory() {
        let nf = NanoFactory::from("1 ORE, 2 WTR => 3 FUEL");
        let target = Chemical::new(3, "FUEL");
        assert_eq!(
            nf,
            NanoFactory::using_formula(Formula::new(
                vec![Chemical::new(1, "ORE"), Chemical::new(2, "WTR")],
                target
            ))
        );
    }
    #[test]
    fn single_ingredient_direct_count() {
        let mut nf = NanoFactory::from("1 ORE => 1 FUEL");
        let target = Chemical::new(1, "FUEL");
        assert_eq!(nf.count_reactant_to_make_wanted_product(&target, "ORE"), 1);
    }
    #[test]
    fn single_ingredient_calculated_direct_count() {
        let mut nf = NanoFactory::from("1 ORE => 3 FUEL");
        let target = Chemical::new(4, "FUEL");
        assert_eq!(nf.count_reactant_to_make_wanted_product(&target, "ORE"), 2);
    }

    #[test]
    fn single_ingredient_indirect_count() {
        let mut nf = NanoFactory::from(
            "1 ORE => 1 IMT
3 IMT => 2 FUEL",
        );
        let target = Chemical::new(3, "FUEL");
        assert_eq!(nf.count_reactant_to_make_wanted_product(&target, "ORE"), 6);
    }
    #[test]
    fn dual_ingredient_indirect_count() {
        let mut nf = NanoFactory::from(
            "5 ORE => 1 A
3 ORE => 1 B
2 A, 2 B => 2 FUEL",
        );
        let target = Chemical::new(3, "FUEL");
        assert_eq!(nf.count_reactant_to_make_wanted_product(&target, "ORE"), 32);
    }
    #[test]
    fn multi_path() {
        let mut nf = NanoFactory::from(
            "5 ORE => 1 A
3 ORE => 1 B
1 A, 1 B => 1 C
2 A, 2 B, 2 C => 2 FUEL",
        );
        let target = Chemical::new(3, "FUEL");
        assert_eq!(nf.count_reactant_to_make_wanted_product(&target, "ORE"), 64);
    }

    #[test]
    fn example_1() {
        let mut nf = NanoFactory::from(example_1_input());
        let target = Chemical::new(1, "FUEL");
        assert_eq!(
            nf.count_reactant_to_make_wanted_product(&target, "ORE"),
            165
        );
    }
    #[test]
    fn larger_example_1() {
        let mut nf = NanoFactory::from(
            "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        );
        let target = Chemical::new(1, "FUEL");
        assert_eq!(
            nf.count_reactant_to_make_wanted_product(&target, "ORE"),
            13312
        );
    }
    #[test]
    fn larger_example_2() {
        let mut nf = NanoFactory::from(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF",
        );
        let target = Chemical::new(1, "FUEL");
        assert_eq!(
            nf.count_reactant_to_make_wanted_product(&target, "ORE"),
            180697
        );
    }
    #[test]
    fn larger_example_3() {
        let mut nf = NanoFactory::from(
            "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX",
        );
        let target = Chemical::new(1, "FUEL");
        assert_eq!(
            nf.count_reactant_to_make_wanted_product(&target, "ORE"),
            2210736
        );
    }

    #[test]
    fn day14_part_1() {
        assert_eq!(day14_part1(), 158482);
    }
    #[test]
    fn day14_part_2() {
        assert_eq!(day14_part2(), 7993831);
    }

    fn example_1_input() -> &'static str {
        "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"
    }
}
