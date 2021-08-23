type Coord = isize;
struct Point {
    x: Coord,
    y: Coord,
    z: Coord,
    t: Coord,
}

impl From<&String> for Point {
    fn from(s: &String) -> Self {
        let n: Vec<isize> = s.split(',').map(|c| c.parse().unwrap()).collect();
        assert_eq!(n.len(), 4);
        Point {
            x: n[0],
            y: n[1],
            z: n[2],
            t: n[3],
        }
    }
}

impl Point {
    fn is_close_to(&self, other: &Point) -> bool {
        3 >= ((self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.t - other.t).abs())
    }
    fn remove_close_constellations(&self, cons: &mut Vec<Constellation>) -> Vec<Constellation> {
        let mut close_cons = vec![];
        while let Some(idx) = cons.iter().position(|con| con.is_close_to(self)) {
            close_cons.push(cons.remove(idx));
        }
        close_cons
    }
}

struct Constellation {
    points: Vec<Point>,
}

impl From<Point> for Constellation {
    fn from(point: Point) -> Self {
        Constellation {
            points: vec![point],
        }
    }
}

impl Constellation {
    fn is_close_to(&self, point: &Point) -> bool {
        self.points.iter().any(|own| point.is_close_to(own))
    }
    fn add_cons(&mut self, mut others: Vec<Constellation>) {
        others
            .drain(..)
            .for_each(|other| self.points.extend(other.points));
    }
}

pub(crate) fn number_of_constellations(input: &[String]) -> usize {
    let mut points: Vec<Point> = input.iter().map(Point::from).collect();

    let mut cons: Vec<Constellation> = vec![];
    while let Some(point) = points.pop() {
        let close_cons = point.remove_close_constellations(&mut cons);
        let mut new_con = Constellation::from(point);
        new_con.add_cons(close_cons);
        cons.push(new_con);
    }
    cons.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::{read_file_to_lines, read_str_to_lines};

    #[test]
    fn part1_example1() {
        assert_eq!(
            2,
            number_of_constellations(&read_str_to_lines(
                "\
0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0"
            ))
        );
    }

    #[test]
    fn part1_example2() {
        assert_eq!(
            4,
            number_of_constellations(&read_str_to_lines(
                "\
-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0"
            ))
        );
    }

    #[test]
    fn part1_example3() {
        assert_eq!(
            3,
            number_of_constellations(&read_str_to_lines(
                "\
1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2"
            ))
        );
    }

    #[test]
    fn part1_example4() {
        assert_eq!(
            8,
            number_of_constellations(&read_str_to_lines(
                "\
1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2"
            ))
        );
    }

    #[test]
    fn part1() {
        assert_eq!(
            399,
            number_of_constellations(&read_file_to_lines("input/day25.txt"))
        );
    }
}
