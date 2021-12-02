use std::fs::File;
use std::io::{prelude::*, BufReader, Error};
use std::path::Path;

enum Direction {
    Up,
    Down,
    Forward,
    Backward,
}

enum Puzzle {
    First,
    Second,
}

struct Position {
    depth: i32,
    horizontal_position: i32,
    aim: i32,
}

impl Position {
    fn new() -> Self {
        Position {
            depth: 0,
            horizontal_position: 0,
            aim: 0,
        }
    }

    // Part 1
    fn change_position(&mut self, dir: &Direction, mag: &i32) {
        match dir {
            Direction::Up => self.depth -= mag,
            Direction::Down => self.depth += mag,
            Direction::Forward => self.horizontal_position += mag,
            Direction::Backward => self.horizontal_position -= mag,
        }
    }

    fn change(&mut self, dir: &Direction, mag: &i32, puzzle: &Puzzle) {
        match puzzle {
            Puzzle::First => &self.change_position(dir, mag),
            Puzzle::Second => &self.adjust(dir, mag),
        };
    }

    fn multiply(&self) -> i32 {
        self.depth * self.horizontal_position
    }

    // Part 2
    fn adjust(&mut self, dir: &Direction, mag: &i32) {
        match dir {
            Direction::Up => self.aim -= mag,
            Direction::Down => self.aim += mag,
            Direction::Forward => {
                self.horizontal_position += mag;
                self.depth += self.aim * mag;
            }
            _ => {
                unreachable!()
            }
        }
    }
}

fn main() -> Result<(), Error> {
    solve("data.txt", &Puzzle::First)?;
    solve("data2.txt", &Puzzle::Second)?;

    Ok(())
}

fn solve(path: &str, puzzle: &Puzzle) -> Result<(), Error> {
    let mut position = Position::new();

    let path = Path::new(path);
    let input = File::open(path)?;

    let data = BufReader::new(input);
    let data_vec = parse(data)?;

    for (dir, mag) in data_vec.iter() {
        position.change(dir, mag, puzzle);
    }

    println!("Final Product: {}", position.multiply());

    Ok(())
}

fn parse(d: BufReader<File>) -> Result<Vec<(Direction, i32)>, Error> {
    let mut parsed: Vec<(Direction, i32)> = Vec::with_capacity(d.capacity());

    for line in d.lines() {
        let line = line.unwrap();
        let data = line.split(' ').collect::<Vec<&str>>();
        let dir = match data.get(0).unwrap().to_owned() {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            "backward" => Direction::Backward,
            _ => unreachable!(),
        };

        parsed.push((dir, data.get(1).unwrap().parse::<i32>().unwrap()));
    }

    Ok(parsed)
}
