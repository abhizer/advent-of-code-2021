use std::collections::BTreeMap;
use std::io::{prelude::*, BufReader};
use std::{fs::File, path::Path};

// This is probably a really bad and unoptimal way to do it, but well, it works
fn main() -> Result<(), std::io::Error> {
    let path = Path::new("data.txt");
    let input = File::open(path)?;

    let data = BufReader::new(&input);
    let counts = counter(BufReader::new(File::open(path)?));
    let (g_r, e_r) = rate_calculator(counts, data.lines().count() as i32);

    // Print the Product
    println!("{}", g_r * e_r);

    Ok(())
}

// Count the number of 1s or 0s in every position and put it in a BTreeMap
fn counter(d: BufReader<File>) -> BTreeMap<usize, i32> {
    let mut counts = BTreeMap::new();
    for i in 0..12 {
        counts.insert(i, 0);
    }

    for line in d.lines() {
        let line = line.unwrap();
        for (i, v) in line.chars().enumerate() {
            counts.insert(i, counts[&i] + v.to_string().parse::<i32>().unwrap());
        }
    }

    counts
}

// Take the data from the BTreeMap and extract Gamma and Epsilon rates
fn rate_calculator(counts: BTreeMap<usize, i32>, n: i32) -> (i32, i32) {
    let mut gamma_rate_bits = String::with_capacity(12);
    let mut epsilon_rate_bits = String::with_capacity(12);

    for (_k, v) in counts.iter() {
        if v > &(n / 2) {
            gamma_rate_bits.push('1');
            epsilon_rate_bits.push('0');
        } else {
            gamma_rate_bits.push('0');
            epsilon_rate_bits.push('1');
        }
    }

    (
        i32::from_str_radix(&gamma_rate_bits, 2).unwrap(),
        i32::from_str_radix(&epsilon_rate_bits, 2).unwrap(),
    )
}
