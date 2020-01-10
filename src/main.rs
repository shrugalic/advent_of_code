#[derive(PartialEq, Debug, Copy, Clone)]
enum Obj {
    Player,
    Wall,
    EmptySpace,
    Key(char),
    Door(char),
}
impl From<char> for Obj {
    fn from(c: char) -> Self {
        match c {
            '@' => Obj::Player,
            '#' => Obj::Wall,
            '.' => Obj::EmptySpace,
            'a'..='z' => Obj::Key(c),
            'A'..='Z' => Obj::Door(c),
            _ => panic!("Invalid object '{}'", c),
        }
    }
}
impl Obj {
    fn matching_key(&self) -> char {
        if let Obj::Door(door) = self {
            // assert_eq!(b'a' - b'A', 32);
            ((*door as u8) + 32) as char
        } else {
            ' '
        }
    }
    fn is_key(&self) -> bool {
        match self {
            Obj::Key(_) => true,
            Obj::Door(_) | Obj::Wall | Obj::Player | Obj::EmptySpace => false,
        }
    }
}
type Map = Vec<Vec<Obj>>;
#[derive(PartialEq, Debug)]
struct UndergroundVault {
    /// A map of the tunnels
    // map[y[x]] = object
    // Example:
    //     x: 012345789
    // y: 0 [[#########],
    // y: 1  [#b.A.@.a#],
    // y: 2  [#########]]
    map: Map,
}
impl From<&str> for UndergroundVault {
    fn from(input: &str) -> Self {
        UndergroundVault {
            map: input
                .split('\n')
                .map(|row| row.chars().map(|c| Obj::from(c)).collect())
                .collect(),
        }
    }
}
impl UndergroundVault {
    fn height(&self) -> usize {
        self.map.len()
    }
    fn width(&self) -> usize {
        self.map[0].len()
    }
    fn key_count(&self) -> usize {
        self.map
            .iter()
            .map(|row| row.iter().filter(|obj| obj.is_key()).count())
            .sum()
    }
    fn object_at(&self, pos: &Vec2) -> &Obj {
        &self.map[pos.y][pos.x]
    }
    fn is_reachable(&self, pos: &Vec2, keys: &[char]) -> bool {
        match self.object_at(pos) {
            Obj::Player | Obj::EmptySpace | Obj::Key(_) => true,
            Obj::Wall => false,
            d @ Obj::Door(_) => keys.contains(&d.matching_key()),
        }
    }
    fn player_pos(&self) -> Vec2 {
        for (y, row) in self.map.iter().enumerate() {
            for (x, obj) in row.iter().enumerate() {
                if obj == &Obj::Player {
                    return Vec2 { x, y };
                }
            }
        }
        panic!("The map contains no player object");
    }
    fn move_to_keys_with(&self, mut start: Explorer) -> Vec<Explorer> {
        let mut explorers_on_keys: Vec<Explorer> = vec![];
        let mut explorers = vec![start];
        while !explorers.is_empty() {
            let mut next_explorers: Vec<Explorer> = vec![];
            explorers.drain(0..).for_each(|mut pf| {
                if let Some(key) = pf.new_key_at_current_pos(&self.map) {
                    //                    println!("New key {} at pos {:?}", key, pf.pos);
                    pf.keys.push(key);
                    explorers_on_keys.push(pf);
                } else {
                    self.possible_next_positions(&pf)
                        .drain(0..)
                        .for_each(|next_pf| next_explorers.push(next_pf))
                }
            });
            explorers.extend(next_explorers);
        }
        explorers_on_keys
    }
    fn includes(&self, pos: &Vec2) -> bool {
        pos.x < self.width() && pos.y < self.height()
    }
    fn possible_next_positions(&self, explorer: &Explorer) -> Vec<Explorer> {
        explorer
            .reachable_positions(self)
            .into_iter()
            .filter(|next_pos| self.includes(&next_pos) && !explorer.just_visited(&next_pos))
            .map(|next_pos| explorer.visit(next_pos))
            .collect()
    }
    fn shortest_path_collecting_all_keys(&self) -> usize {
        let key_count = self.key_count();
        //        println!("key count = {}", key_count);
        let mut finished_explorers = vec![];

        let mut explorers_on_keys = self.move_to_keys_with(self.initial_explorer());
        while !explorers_on_keys.is_empty() {
            let mut next_explorers = vec![];
            explorers_on_keys.drain(0..).for_each(|mut explorer| {
                //                println!("{}/{} keys {:?}", explorer.keys.len(), key_count, explorer);
                if explorer.keys.len() == key_count {
                    finished_explorers.push(explorer);
                } else {
                    explorer.paths.push(vec![]); // Start of next leg
                    let new_explorers = self.move_to_keys_with(explorer);
                    //                    println!("New explorers = {:?}", new_explorers);
                    next_explorers.extend(new_explorers);
                }
            });
            explorers_on_keys = next_explorers;
        }
        //        println!("Finished = {:?}", finished_explorers);
        let shortest = finished_explorers
            .iter()
            .min_by_key(|explorer| explorer.path_length())
            .unwrap();
        //        println!("Shortest = {:?}", shortest);
        shortest.path_length() - 1 // Don't count the starting position
    }
    fn initial_explorer(&self) -> Explorer {
        Explorer::initial(self.player_pos())
    }
}

#[derive(Debug)]
struct Explorer {
    pos: Vec2,        // Current position
    paths: Vec<Path>, // Key collection paths. Last one is current search path, may be incomplete
    keys: Vec<char>,  // Collected keys
}
impl Explorer {
    fn initial(pos: Vec2) -> Self {
        let paths = vec![vec![pos]];
        let keys = vec![];
        Explorer { pos, keys, paths }
    }
    fn curr_path(&self) -> Path {
        self.paths.last().unwrap().clone()
    }
    fn visit(&self, pos: Vec2) -> Explorer {
        let mut paths = self.paths.clone();
        paths.last_mut().unwrap().push(pos);
        let keys = self.keys.clone();
        Explorer { pos, paths, keys }
    }
    fn just_visited(&self, pos: &Vec2) -> bool {
        self.curr_path().contains(pos)
    }
    fn reachable_positions(&self, vault: &UndergroundVault) -> Vec<Vec2> {
        let mut dirs = vec![];
        let pos = &self.pos;
        let keys = &self.keys;
        if pos.x + 1 < vault.width() && vault.is_reachable(&pos.offset_by(1, 0), keys) {
            dirs.push(Vec2::new(pos.x + 1, pos.y));
        }
        if pos.y + 1 < vault.height() && vault.is_reachable(&pos.offset_by(0, 1), keys) {
            dirs.push(Vec2::new(pos.x, pos.y + 1));
        }
        if pos.x > 0 && vault.is_reachable(&pos.offset_by(-1, 0), keys) {
            dirs.push(Vec2::new(pos.x - 1, pos.y));
        }
        if pos.y > 0 && vault.is_reachable(&pos.offset_by(0, -1), keys) {
            dirs.push(Vec2::new(pos.x, pos.y - 1));
        }
        dirs
    }
    fn path_length(&self) -> usize {
        self.paths.iter().map(|path| path.len()).sum()
    }
    fn new_key_at_current_pos(&self, map: &Map) -> Option<char> {
        if let Obj::Key(key) = map[self.pos.y][self.pos.x] {
            if self.keys.contains(&key) {
                None
            } else {
                Some(key)
            }
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Vec2 {
    x: usize,
    y: usize,
}
impl Vec2 {
    fn new(x: usize, y: usize) -> Self {
        Vec2 { x, y }
    }
    fn offset_by(&self, x_offset: isize, y_offset: isize) -> Vec2 {
        Vec2::new(
            (self.x as isize + x_offset) as usize,
            (self.y as isize + y_offset) as usize,
        )
    }
}
type Path = Vec<Vec2>;

fn main() {
    println!("Hello, world!");
}

fn day_18_example_1() -> &'static str {
    "#########
#b.A.@.a#
#########"
}

fn day_18_example_2() -> &'static str {
    "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################"
}

fn day_18_example_3() -> &'static str {
    "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################"
}

fn day_18_example_4() -> &'static str {
    "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################"
}

fn day_18_example_5() -> &'static str {
    "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################"
}

fn day_18_puzzle_input() -> &'static str {
    "#################################################################################
#.....#........q..#...........#.........#.#...#.......#.#...#...#.....P...#.....#
###E#.#####.#######.###G#####.#########.#.#.#.#.###.#.#.#.#.#.#.#.#######.#.#.#.#
#...#.....#...#.....#.#.#i..#.#...J.....#...#.....#.#...#.#...#.#.......#...#.#.#
#.#######.###.#.#####.#.#.###.#.#######.#.#########.###.#.#####.#####.#######.#.#
#.......#.....#.#.....#.#...#...#.....#.#.#.....#...#...#.#...#.#...#.#.......#.#
#.#####.###.###.#.#.###.###.#########.#.#.###.#.#.#.#####.#.###.#.#.#.#.#########
#.....#.#.#.#...#.#...#...#.....#.....#.#...#.#.#.#t#.....#...#...#.#.#.#.......#
#####.#.#.#.#.#######.###.#.###.#.###.#.###.#.###.###.#######.#####.#.#.#.#####.#
#...#.#.#...#.........#...#.#.....#.#.#.#.#...#...#...#...........#.#.#.#.#...#.#
#.#.#.#.###.#########.#.###.#######.#.#.#.#.###.#.#.#######.#####.#.###.#.###.#.#
#.#p#.#...#...#...#...#.#.#.......#...#.#.#.#...#.#.....#...#...#.#.#...#.....#.#
#.#.#.###.###.#.###.###.#.#.#####.#.###.#.#.#.#########.#.###.#.###.#.#####.###.#
#.#...#...#...#...#.#...#...#.#...#.#...#...#.#.......#...#...#.....#.#...#.#...#
#####.#.###.#####.#.#.###.###.#.###.#.###.###.#.###.#####.#.#########.#.#.#.#.#.#
#...#.#r#...#.....#.#...#.....#.#...#...#.#.#.#.#.#.....#.#.#.......#...#...#.#.#
#.#.###.#.#####.#.#.###.#####.#.#.#####.#.#.#.#.#.###.#.#.#.###.###.#.#######.#.#
#.#.....#.#...#.#.#.#.#.....#.#.#...#...#...#...#...#.#.#.#...#.#...#...#.....#.#
#.#######.#.#.#.#.#.#.#####.###.###.#.#####.#####.#.#.#.#.###.#.#.#####.#.#####.#
#.....#.#...#...#.#.......#...#...#.#.#.#.#.#.....#.#.#.#.#.#.#.#.#.....#.#...#.#
#####.#.#####.#####.#####.###Y#.#.#.#.#.#.#.###.#.#.#.###.#.#.#.###.#####.#.#.#.#
#.....#.#.....#...#.#...#...#.#.#.#.#.#.#.#...#.#.#.#...#...#.#.#...#...#.#.#.#.#
#.#####.#.#####.#.#.###.#.###.#.#.###.#.#.###.###.#.###.###.###.#.#####.#.###.#.#
#...#.......#.#.#.....#.#.#...#.#...#...#...#..b..#.#.#.#...#...........#...#...#
#.#.#######.#.#.#######.###.#######.###.#.#########.#.#.#.#####.#######.###.###.#
#.#.#...#.D...#v#.....#...#.#.....#.....#.#.......#...#.#.#...#.#...#.....#...#.#
#.#.#L#.#######.#.###.###.#.#.###.#.#####.#.#.#######.#.#.#.#.#.#.#.#########.#.#
#.#...#.#.....#.#...#.....#.....#.#...#.#.#.#.........#...#.#.#.#.#.#.......#.#.#
#.#####.#.###.#.###.#############.###.#.#.#.###############.#.###.#.#.#####.#.#.#
#.#...#...#.#...#.#.........#.....#...#.#.#.#.#.......#.....#...#.#.#.#.....V.#.#
#.#.#.#####.#####.#########.#.#######.#.#.#.#.#.###X#.#.#.#####.#.#.#.#########.#
#...#.#...#.........#.....#.#.......#...#...#.#...#.#.#.#.#...#.#.#.#.....#.R.#.#
#####.#.###.#.#####.#.#####.#.#####.#####.###.###.#.#.###.#.###.#.#.#.###.#.###.#
#...#.#.....#.....#.#.....#.#.....#.....#.#.......#.#....y#.#...#.#.#.#...#.#...#
#.###.#.#########.#.#####.#.#####.#####.#.###.#####.#######.#.###.#.###.###.#.###
#.....#...#.......#.#.....#...#.....#.#.#...#.....#.#.....#...#...#...#.#.....#.#
#.#########.#######.#.#######.#####.#.#.###.#######.#.###.#.#####.###.#.#.#####.#
#.........#.#.#...#.#.......#.#...#...#.#...#.....#.#.#...#.#...#...#...#.....#.#
#########.#.#.#.#.#.#####.#.#.#.#.#####.#.###.###.#.#.#.###.#.#.###.#########.#.#
#...........#...#.........#.#...#.............#.....#.#.......#.....#...........#
#######################################.@.#######################################
#.........#.....#...#...........#.............#.......#.......#.............#...#
#.#.#######.#.###.#.###.#######.###.###.###.#.#.#####.#.#.###.#####.#####.#.#.#.#
#.#m#z......#.....#..o#.....#.......#...#...#...#.....#.#...#.#...#...#...#.#.#.#
#.#.#.###############.#####.#########.###.#########.#######.#.#.#.#.###.###.#.#.#
#.#.#.....#.........#.....#.#...#...#.#.#.........#.#.......#...#.#.#...#.#.#.#.#
#.#.#####.###.###.#.#####.#.#.#.#.#.#.#.#.#######.#.###.#########.###.###.#.###.#
#.#.#...#...#...#.#...#...#...#...#.#...#.....#...#.....#...#.....#...#...#...#.#
#.###.#.###.###.#.###.#.###########.###.#######.#########.###.#####.###.#.###.#.#
#.....#.....#...#.#...#...#.....W.#...#.#.......#.........#...#.....#...#...#...#
#.###########.###.#.#####.#.#####.###.#.#.#######.###.#####.###.#.###.###.#####.#
#.....#.........#.#...#...#.#.....#...#.#.#...#.....#.....#.#...#.#...#.#.....#.#
#####.###########.#.###.###.#.#####.###.#.#.#.#.#####.###.#.#####.#.###.#####.#.#
#...#.....#.......#.#...#...#..n#.....#x#.#.#.#.#...#...#.......#.#.#...#...#...#
#.#######.#.#######.#.#.#.#####.#####.#.#.#.#.###.#.###########.#.#.#.#.#.#.#####
#...........#c....#.#.#.#...#.#.#.....#.#...#.#...#...#.......#...#l..#...#.....#
#.###########.###.#.#.#.###.#.#.#.#####.#####.#.#####.#.#####.#################.#
#.........#.F.#.#.#.#.#...#.#.#.#.....#.#...#.#.....#.#...#.#.#.....#.#.......#.#
#########.#.###.#.#.#H###.#.#.#.#.###.###.#.#.#####.#.###.#.#.###.#.#.#.#####.#.#
#.......#.#.....#.#.#...#.#...#.#...#...#.#.......#.#...#.#.#...#.#...#.#...#...#
#.###.###.#####.#.#####.#.###.#.###.###.#.#####.###.#.###T#.###.#.#####.###.#####
#...#.........#.#...#.#.#...#.#.#...#...#...#...#...#.....#.#...#.#.....#.......#
#.#############.###.#C#.#####.#.###.#.#####.#.###.#########.#####.#.#####.#######
#.#........f....#.#...#.......#...#g#...#...#...#.#...............#.#...#......u#
#.#K#############.#######.#######N#.###.#.#####B#.###########.###.#.#.#.#####.#.#
#.#.....#.....A.#....a..#.#.U...#.#...#.#.#.....#...#.........#...#...#.....#.#.#
#.#####.###.###.###.###.#.#.###.#.#.###.#.#########.#.#########.###########.###.#
#.#...#..k..#.#.....#.#.#...#w..#.#.#...#...........#.#...#...#.......#.....#...#
#.#.#########.#######.#.#########.#.#.#.#############.#.#.#.#.#######.#.#####.###
#.#.........#.........#.#......s..#.#.#.#.#.......#...#.#...#.......#.#.#.#.....#
#.#.###.###.#####.###.#S#.#########.#.###.#.#.###.#.###.###########.###.#.#.###.#
#.#...#.#.#.#...#.#...#...#...#...#.#...#.#.#.#.....#...#.....#...#...#.#...#...#
#.#####.#.#.#.#.#.#.#######.#.#.###.###.#.#.#.###.###.###.#.#.###.###.#.#.###.###
#..e....#.#.#.#...#...#.....#.#.......#.#...#...#.#...#...#.#.......#...#...#...#
#########.#.#.#######.#####.#.#.#######.#.#####.#M#.#####.#.#######.###########.#
#.......I.#.#.#.......#.....#.#..j#...#.#...#...#.#.#...#.#...#...#.#...#.......#
#.#########.#.#.#######.#####.#####.#.#.#####.#####Q#.#.#####.###.#.#.#.#.#####.#
#.....#...#...#.......#.#...#.......#.#.#.....#.....#.#.....#...#.#.Z.#.#.O.#.#.#
#.###.#.#.###########.#.#.###########.#.#.#####.#####.#####.###.#.#####.###.#.#.#
#...#...#...............#...............#..d..........#.........#...........#..h#
#################################################################################"
}
mod tests {
    use crate::{
        day_18_example_1, day_18_example_2, day_18_example_3, day_18_example_4, day_18_example_5,
        day_18_puzzle_input, Explorer, Obj, UndergroundVault, Vec2,
    };

    #[test]
    fn object_from_char() {
        assert_eq!(Obj::from('@'), Obj::Player);
        assert_eq!(Obj::from('#'), Obj::Wall);
        assert_eq!(Obj::from('d'), Obj::Key('d'));
        assert_eq!(Obj::from('F'), Obj::Door('F'));
    }
    #[test]
    fn door_matches_key() {
        assert_eq!(Obj::Door('A').matching_key(), 'a');
    }
    #[test]
    fn non_door_does_not_match_key() {
        assert_ne!(Obj::Key('A').matching_key(), 'a');
    }
    fn simple_3x3_vault() -> UndergroundVault {
        UndergroundVault {
            map: vec![
                vec![Obj::Wall, Obj::Wall, Obj::Wall],
                vec![Obj::Key('a'), Obj::Player, Obj::Door('A')],
                vec![Obj::Wall, Obj::Wall, Obj::Wall],
            ],
        }
    }
    #[test]
    fn map_from_str() {
        assert_eq!(
            UndergroundVault::from(
                "###
a@A
###"
            ),
            simple_3x3_vault()
        );
    }
    #[test]
    fn object_at_pos() {
        let vault = simple_3x3_vault();
        assert_eq!(vault.object_at(&Vec2::new(0, 0)), &Obj::Wall);
        assert_eq!(vault.object_at(&Vec2::new(0, 1)), &Obj::Key('a'));
        assert_eq!(vault.object_at(&Vec2::new(1, 1)), &Obj::Player);
        assert_eq!(vault.object_at(&Vec2::new(2, 1)), &Obj::Door('A'));
        assert_eq!(vault.object_at(&Vec2::new(2, 2)), &Obj::Wall);
    }
    #[test]
    fn player_pos() {
        assert_eq!(simple_3x3_vault().player_pos(), Vec2::new(1, 1));
    }

    #[test]
    fn reachable_positions_without_key() {
        let vault = simple_3x3_vault();
        assert_eq!(
            Explorer::initial(vault.player_pos()).reachable_positions(&vault),
            vec![Vec2::new(0, 1)]
        );
    }
    #[test]
    fn reachable_positions_with_key() {
        let vault = simple_3x3_vault();
        let pos = Vec2::new(1, 1);
        let paths = vec![vec![]];
        let keys = vec!['a'];
        assert_eq!(
            Explorer { pos, paths, keys }.reachable_positions(&vault),
            vec![Vec2::new(2, 1), Vec2::new(0, 1)]
        );
    }

    #[test]
    fn reachable_key_path_simple() {
        let vault = simple_3x3_vault();
        assert_eq!(
            vault.move_to_keys_with(vault.initial_explorer())[0].curr_path(),
            vec![Vec2::new(1, 1), Vec2::new(0, 1)]
        );
    }
    #[test]
    fn reachable_key_path_example_1() {
        let vault = UndergroundVault::from(day_18_example_1());
        assert_eq!(vault.player_pos(), Vec2::new(5, 1));
        assert_eq!(
            vault.move_to_keys_with(vault.initial_explorer())[0].curr_path(),
            vec![Vec2::new(5, 1), Vec2::new(6, 1), Vec2::new(7, 1)]
        );
    }
    #[test]
    fn collect_all_keys_simple() {
        assert_eq!(simple_3x3_vault().shortest_path_collecting_all_keys(), 1);
    }
    #[test]
    fn is_reachable_example_1() {
        assert_eq!(
            UndergroundVault::from(day_18_example_1()).is_reachable(&Vec2::new(3, 1), &vec!['a']),
            true
        );
    }
    #[test]
    fn collect_all_keys_example_1() {
        assert_eq!(
            UndergroundVault::from(day_18_example_1()).shortest_path_collecting_all_keys(),
            8
        );
    }
    #[test]
    fn collect_all_keys_example_2() {
        assert_eq!(
            UndergroundVault::from(day_18_example_2()).shortest_path_collecting_all_keys(),
            86
        );
    }
    #[test]
    fn collect_all_keys_example_3() {
        assert_eq!(
            UndergroundVault::from(day_18_example_3()).shortest_path_collecting_all_keys(),
            132
        );
    }
    #[test]
    fn collect_all_keys_example_4() {
        assert_eq!(
            UndergroundVault::from(day_18_example_4()).shortest_path_collecting_all_keys(),
            136
        );
    }
    #[test]
    fn collect_all_keys_example_5() {
        assert_eq!(
            UndergroundVault::from(day_18_example_5()).shortest_path_collecting_all_keys(),
            81
        );
    }
    #[test]
    fn collect_all_keys_part_1() {
        assert_eq!(
            UndergroundVault::from(day_18_puzzle_input()).shortest_path_collecting_all_keys(),
            1
        );
    }
}
