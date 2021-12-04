use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
    process::exit,
};

#[derive(Debug, Copy, Clone)]
enum Bingo {
    Marked(u32),
    Unmarked(u32),
}

impl Bingo {
    fn mark(mut self) -> Self {
        println!("{:?}", self);
        if let Bingo::Unmarked(v) = self {
            self = Bingo::Marked(v);
        };
        println!("{:?}", self);
        self
    }
}

#[derive(Debug, Clone)]
struct BingoBoard {
    board: [[Bingo; 5]; 5],
    winner: bool,
    map: std::collections::HashMap<u32, (u32, u32)>,
    record: std::collections::HashMap<String, u32>,
}

impl BingoBoard {
    fn new() -> Self {
        BingoBoard {
            board: [[Bingo::Unmarked(0); 5]; 5],
            winner: false,
            map: HashMap::new(),
            record: HashMap::new(),
        }
    }

    fn fill(&mut self, data: &[Vec<u32>]) {
        // data.iter().enumerate().for_each(|(i, row)| {
        for (i, row) in data.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                self.board[i][j] = Bingo::Unmarked(*val);
                self.map.insert(*val, (i as u32, j as u32));
            }
        }

        for i in 0..5 {
            self.record.insert(format!("{}i", i), 0);
            self.record.insert(format!("{}j", i), 0);
        }
    }

    fn wins(&mut self, num: u32) -> u32 {
        self.winner = true;
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if let Bingo::Unmarked(v) = self.board[i][j] {
                    sum += v
                }
            }
        }
        sum * num
    }

    fn update_record(&mut self, k: &str) {
        if self.record.contains_key(k) {
            self.record.insert(k.to_string(), self.record[k] + 1);
        }
        // println!("{:?}", self.record);
    }

    fn check_record(&self, k: &str, k2: &str) -> bool {
        self.record[k] == 5 || self.record[k2] == 5
    }

    fn mark_it(&mut self, num: &u32) {
        // println!("{:?} {}", self.board[0], num);
        // let (i, j) = match self.map.get(num) {
        //     Some(x) => x.to_owned(),
        //     None => {
        //         eprintln!("None");
        //         return;
        //     }
        // };

        if self.map.contains_key(num) {
            let (i, j) = self.map[num];
            println!("{:?} ({},{})", self.board[0], i, j);
        }

        // if !self.map.contains_key(num) {
        //     println!("Does not exist");
        //     return;
        // }
        // let (mut i, mut j) = self.map[num];

        // self.update_record(&format!("{}i", i));
        // self.update_record(&format!("{}j", j));

        // self.board[i as usize][j as usize] = self.board[i as usize][j as usize].mark();

        // if self.check_record(&format!("{}i", i), &format!("{}j", j)) {
        //     println!("Winner Points: {}", self.wins(*num));
        //     exit(0);
        // }
        // i = 10;
        // j = 10;
        // drop(i);
        // drop(j);
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut data = BufReader::new(File::open(Path::new("data.txt"))?);
    let mut buffer = String::from("");
    data.read_line(&mut buffer)?;

    let draw_sequence: Vec<u32> = buffer
        .trim()
        .split(',')
        .map(|v| v.parse::<u32>().unwrap())
        .collect();

    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut board: BingoBoard = BingoBoard::new();
    let mut temp_board: Vec<Vec<u32>> = Vec::new();

    for line in data.lines().skip(1) {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() & !temp_board.is_empty() {
            board.fill(&temp_board);
            boards.push(board.clone());
            temp_board = Vec::new();
        }
        if line.is_empty() {
            continue;
        }
        temp_board.push(
            line.split(' ')
                .filter(|v| !v.trim().is_empty())
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        );
    }

    // for board in boards.iter() {
    //     for i in 0..5 {
    //         for j in 0..5 {
    //             match board.board[i][j] {
    //                 Bingo::Marked(v) => print!("{} ", v),
    //                 Bingo::Unmarked(v) => print!("{} ", v),
    //             }
    //         }
    //         println!();
    //     }
    //     println!();
    // }

    for draw in draw_sequence.iter() {
        println!("{}", draw);
        boards.iter_mut().for_each(|b| b.mark_it(draw));
    }

    Ok(())
}
