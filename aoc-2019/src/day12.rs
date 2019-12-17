// https://adventofcode.com/2019/day/12

use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone, PartialEq)]
struct Position(i32, i32, i32);

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={:3}, y={:3}, z={:3}>", self.0, self.1, self.2)
    }
}

fn parse_position(s: &str) -> Position {
    let s = s.trim_start_matches('<').trim_end_matches('>');
    let parts = s.split(',').map(|s| s.trim());
    let parts: Vec<_> = parts
        .map(|s| s.splitn(2, '=').last())
        .flatten()
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();
    match parts.as_slice() {
        [x, y, z] => Position(*x, *y, *z),
        _ => panic!("invalid position"),
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Moon {
    position: Position,
    velocity: Position,
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos={}, vel={}", self.position, self.velocity)
    }
}

impl Moon {
    fn new(position: Position) -> Self {
        let velocity = Position(0, 0, 0);
        Moon { position, velocity }
    }

    fn energy(&self) -> i32 {
        let e1 = self.position.0.abs() + self.position.1.abs() + self.position.2.abs();
        let e2 = self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs();
        e1 * e2
    }
}

fn parse_input(s: &str) -> Vec<Moon> {
    s.lines()
        .map(|s| s.trim())
        .map(|s| Moon::new(parse_position(s)))
        .collect()
}

fn calc_velocity(p1: i32, p2: i32) -> (i32, i32) {
    if p1 > p2 {
        (-1, 1)
    } else if p2 > p1 {
        (1, -1)
    } else {
        (0, 0)
    }
}

fn calc_velocity_for_pos(p1: &Position, p2: &Position) -> (Position, Position) {
    let dx = calc_velocity(p1.0, p2.0);
    let dy = calc_velocity(p1.1, p2.1);
    let dz = calc_velocity(p1.2, p2.2);
    (Position(dx.0, dy.0, dz.0), Position(dx.1, dy.1, dz.1))
}

fn step_time(moons: &[Moon]) -> Vec<Moon> {
    let positions: Vec<&Position> = moons.iter().map(|moon| &moon.position).collect();

    let delta_v: Vec<_> = positions
        .iter()
        .enumerate()
        .map(|moon| {
            std::iter::repeat(moon)
                .zip(positions.iter().enumerate())
                .filter(|(p1, p2)| p1.0 != p2.0)
                .map(|(p1, p2)| (p1.1, p2.1))
                .map(|(p1, p2)| calc_velocity_for_pos(p1, p2).0)
                .fold(Position(0, 0, 0), |sum, p| sum + p)
        })
        .collect();

    let moons: Vec<_> = moons
        .iter()
        .cloned()
        .zip(delta_v.iter())
        .map(|(moon, v)| Moon {
            position: moon.position + moon.velocity.clone() + v.clone(),
            velocity: moon.velocity.clone() + v.clone(),
        })
        .collect();

    moons
}

fn total_energy(moons: &[Moon]) -> i32 {
    moons.iter().map(|moon| moon.energy()).sum::<i32>()
}

fn simulate(moons: Vec<Moon>) -> impl Iterator<Item = Vec<Moon>> {
    std::iter::successors(Some(moons), |moons| Some(step_time(moons)))
}

fn calc_steps_to_repeat(moons: Vec<Moon>) -> usize {
    let start: Vec<_> = moons.clone();
    let sim = simulate(moons);
    sim.enumerate()
        .skip(1)
        .find(|(_i, moons)| moons == &start)
        .map(|(i, _moons)| i)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

    const INPUT_2: &str = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

    const MY_INPUT: &str = include_str!("day12.input");

    #[test]
    fn test_part_1() {
        let tests = &[((INPUT_1, 10), 179), ((MY_INPUT, 1000), 5937)];
        for ((input, step), expected) in tests {
            let moons = parse_input(input);
            let mut sim = simulate(moons);
            let moons = sim.nth(*step).unwrap();
            let energy = total_energy(&moons);
            assert_eq!(&energy, expected);
        }
    }

    #[test]
    fn test_part_2_ex_1() {
        let moons = parse_input(INPUT_1);
        assert_eq!(calc_steps_to_repeat(moons), 2772);
    }

    #[test]
    #[ignore]
    fn test_part_2_ex_2() {
        let moons = parse_input(INPUT_2);
        assert_eq!(calc_steps_to_repeat(moons), 4686774924);
    }

    #[test]
    #[ignore]
    fn test_part_2_my_input() {
        let moons = parse_input(MY_INPUT);
        assert_eq!(calc_steps_to_repeat(moons), 0);
    }
}
