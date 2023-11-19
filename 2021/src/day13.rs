use std::collections::HashSet;
use Instruction::*;

const INPUT: &str = include_str!("../input/day13.txt");

pub(crate) fn day13_part1() -> usize {
    TransparentPaper::from(INPUT).fold_once().number_of_dots()
}

pub(crate) fn day13_part2() -> String {
    TransparentPaper::from(INPUT).fold_all_the_way().letters()
}

#[derive(Debug)]
struct TransparentPaper {
    dots: HashSet<(usize, usize)>,
    instructions: Vec<Instruction>,
}
impl TransparentPaper {
    fn fold_once(self) -> Self {
        self.follow_folding_instructions(true)
    }
    fn fold_all_the_way(self) -> Self {
        self.follow_folding_instructions(false)
    }
    fn follow_folding_instructions(mut self, fold_once_only: bool) -> Self {
        for instruction in self.instructions.iter() {
            self.dots = self
                .dots
                .drain()
                .map(|(x, y)| match instruction {
                    FoldVertical(edge) if y > *edge => (x, 2 * edge - y),
                    FoldHorizontal(edge) if x > *edge => (2 * edge - x, y),
                    _ => (x, y),
                })
                .collect();
            if fold_once_only {
                break;
            }
        }
        self
    }
    fn number_of_dots(&self) -> usize {
        self.dots.len()
    }
    fn letters(&self) -> String {
        let width = self.dots.iter().map(|(x, _)| x).max().unwrap() + 1;
        let height = self.dots.iter().map(|(_, y)| y).max().unwrap() + 1;
        const LETTER_WIDTH: usize = 5;
        let count = (width + 1) / LETTER_WIDTH;
        let mut letters = vec![vec![vec![' '; LETTER_WIDTH]; height]; count];
        self.dots
            .iter()
            .for_each(|(x, y)| letters[x / LETTER_WIDTH][*y][x % LETTER_WIDTH] = '#');
        letters
            .into_iter()
            .map(TransparentPaper::letter_vec_to_string)
            .map(|s| TransparentPaper::letter_string_to_char(&s))
            .collect()
    }
    fn letter_vec_to_string(letter: Vec<Vec<char>>) -> String {
        letter
            .into_iter()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
    fn letter_string_to_char(letter: &str) -> char {
        match letter {
            "\
#####
#   #
#   #
#   #
#####" => 'O',
            "\
###  
#  # 
#  # 
###  
#    
#    " => 'P',
            "\
#### 
#    
###  
#    
#    
#### " => 'E',
            "\
###  
#  # 
#  # 
###  
# #  
#  # " => 'R',
            " ##  
#  # 
#    
#    
#  # 
 ##  " => 'C',
            " ##  
#  # 
#    
# ## 
#  # 
 ### " => 'G',
            "  ## 
   # 
   # 
   # 
#  # 
 ##  " => 'J',
            "\
###  
#  # 
###  
#  # 
#  # 
###  " => 'B',
            _ => unimplemented!("Unknown letter\n{}", letter),
        }
    }
}
impl From<&str> for TransparentPaper {
    fn from(input: &str) -> Self {
        let (coordinates, instructions) = input.split_once("\n\n").unwrap();
        let mut dots = HashSet::new();
        for line in coordinates.trim().lines() {
            let (x, y) = line.split_once(',').unwrap();
            dots.insert((x.parse().unwrap(), y.parse().unwrap()));
        }
        let instructions = instructions.trim().lines().map(Instruction::from).collect();
        TransparentPaper { dots, instructions }
    }
}

#[derive(Debug)]
enum Instruction {
    FoldVertical(usize),
    FoldHorizontal(usize),
}
impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        if line.starts_with("fold along y=") {
            Instruction::FoldVertical(line.trim_start_matches("fold along y=").parse().unwrap())
        } else {
            Instruction::FoldHorizontal(line.trim_start_matches("fold along x=").parse().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn part1_example() {
        assert_eq!(
            TransparentPaper::from(EXAMPLE).fold_once().number_of_dots(),
            17
        );
    }

    #[test]
    fn part1() {
        assert_eq!(781, day13_part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            TransparentPaper::from(EXAMPLE).fold_all_the_way().letters(),
            "O"
        );
        /*
        #####
        #   #
        #   #
        #   #
        #####
         */
    }

    #[test]
    fn part2() {
        assert_eq!("PERCGJPB", day13_part2());
        /*
        ###  #### ###   ##   ##    ## ###  ###
        #  # #    #  # #  # #  #    # #  # #  #
        #  # ###  #  # #    #       # #  # ###
        ###  #    ###  #    # ##    # ###  #  #
        #    #    # #  #  # #  # #  # #    #  #
        #    #### #  #  ##   ###  ##  #    ###
        */
    }
}
