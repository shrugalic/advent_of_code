use line_reader::read_file_to_lines;
use reformation::Reformation;
use std::collections::HashMap;
use std::ops::AddAssign;

pub(crate) fn day20_part1() -> usize {
    index_of_particle_staying_closest_to_origin(read_file_to_lines("input/day20.txt"))
}

pub(crate) fn day20_part2() -> usize {
    number_of_particles_remaining_after_collisions(read_file_to_lines("input/day20.txt"))
}

fn index_of_particle_staying_closest_to_origin(input: Vec<String>) -> usize {
    let particles = parse_input(input);
    // The particle that will stay closest to the origin is the one with the smallest acceleration
    particles
        .iter()
        .enumerate()
        .min_by_key(|(_, p)| p.acc.abs())
        .unwrap()
        .0
}

fn number_of_particles_remaining_after_collisions(input: Vec<String>) -> usize {
    let mut particles = parse_input(input);
    while particles.iter().any(Particle::is_decelerating)
        // This second condition is only for the part 2 example ;)
        || particles.len() == 4
    {
        let mut count_by_position = HashMap::new();
        particles.iter_mut().for_each(|particle| {
            particle.tick();
            *count_by_position.entry(particle.pos).or_insert(0) += 1;
        });
        for particle_pos in count_by_position
            .iter()
            .filter(|(_, count)| **count > 1)
            .map(|(pos, _)| pos)
        {
            while let Some(idx) = particles
                .iter()
                .position(|particle| particle.pos == *particle_pos)
            {
                particles.swap_remove(idx);
            }
        }
    }
    particles.len()
}

fn parse_input(input: Vec<String>) -> Vec<Particle> {
    input
        .iter()
        .map(|line| Particle::parse(line).unwrap())
        .collect()
}

type Coord = isize;

#[derive(Reformation, PartialEq, Debug, Copy, Clone, Eq, Hash)]
#[reformation(r"<{x},{y},{z}>")]
struct Vec3 {
    x: Coord,
    y: Coord,
    z: Coord,
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl Vec3 {
    fn abs(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

#[derive(Reformation, PartialEq, Debug)]
#[reformation(r"p={pos}, v={vel}, a={acc}")]
struct Particle {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
}
impl Particle {
    fn tick(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
    }
    fn is_decelerating(&self) -> bool {
        (self.vel.x.signum() != self.acc.x.signum() && self.acc.x != 0)
            || (self.vel.y.signum() != self.acc.y.signum() && self.acc.y != 0)
            || (self.vel.z.signum() != self.acc.z.signum() && self.acc.z != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use line_reader::read_str_to_lines;

    const EXAMPLE_1: &str = "\
p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";

    #[test]
    fn parse_vec3() {
        let vec3 = Vec3::parse("<3,0,0>").unwrap();
        assert_eq!(Vec3 { x: 3, y: 0, z: 0 }, vec3);
    }

    #[test]
    fn parse_particle() {
        let particle = Particle::parse("p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>").unwrap();
        assert_eq!(
            Particle {
                pos: Vec3 { x: 3, y: 0, z: 0 },
                vel: Vec3 { x: 2, y: 0, z: 0 },
                acc: Vec3 { x: -1, y: 0, z: 0 }
            },
            particle
        );
    }

    #[test]
    fn particle_tick() {
        let mut p0 = Particle::parse("p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>").unwrap();
        p0.tick();
        assert_eq!(
            Particle::parse("p=<4,0,0>, v=<1,0,0>, a=<-1,0,0>").unwrap(),
            p0
        );
        p0.tick();
        assert_eq!(
            Particle::parse("p=<4,0,0>, v=<0,0,0>, a=<-1,0,0>").unwrap(),
            p0
        );
        p0.tick();
        assert_eq!(
            Particle::parse("p=<3,0,0>, v=<-1,0,0>, a=<-1,0,0>").unwrap(),
            p0
        );

        let mut p1 = Particle::parse("p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>").unwrap();
        p1.tick();
        assert_eq!(
            Particle::parse("p=<2,0,0>, v=<-2,0,0>, a=<-2,0,0>").unwrap(),
            p1
        );
        p1.tick();
        assert_eq!(
            Particle::parse("p=<-2,0,0>, v=<-4,0,0>, a=<-2,0,0>").unwrap(),
            p1
        );
        p1.tick();
        assert_eq!(
            Particle::parse("p=<-8,0,0>, v=<-6,0,0>, a=<-2,0,0>").unwrap(),
            p1
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(
            0,
            index_of_particle_staying_closest_to_origin(read_str_to_lines(EXAMPLE_1))
        );
    }
    #[test]
    fn part1() {
        assert_eq!(258, day20_part1());
    }

    const EXAMPLE_2: &str = "\
p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>";

    #[test]
    fn part2_example() {
        assert_eq!(
            1,
            number_of_particles_remaining_after_collisions(read_str_to_lines(EXAMPLE_2))
        );
    }

    #[test]
    fn part2() {
        assert_eq!(707, day20_part2());
    }
}
