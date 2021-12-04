use std::collections::{BTreeMap, HashMap};
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

    part_2(BufReader::new(File::open(path)?));

    // life_support_rating_calculator(g_r, e_r, BufReader::new(File::open(path)?));

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

// Part 2
enum Gas {
    Oxygen,
    CO2,
}

// Horrible horrible, probably way too over engineered code this
fn part_2(d: BufReader<File>) {
    let arr: Vec<String> = d.lines().map(|line| line.unwrap()).collect();
    let o2 = calculate_gas_metrics(&arr, Gas::Oxygen);
    let co2 = calculate_gas_metrics(&arr, Gas::CO2);

    println!("{}", o2 * co2);
}

fn calculate_gas_metrics(arr: &[String], g: Gas) -> u32 {
    let mut arr = arr.to_owned();
    for i in 0..12 {
        let popular_char = get_popular_char_at(&arr, i, &g);
        let retained = retain(&arr, i as u32, popular_char);
        let mut iter = retained.iter();
        arr.retain(|_| *iter.next().unwrap());
        if arr.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(arr.first().unwrap(), 2).unwrap()
}

fn get_popular_char_at(arr: &[String], index: i32, g: &Gas) -> char {
    let mut counter = 0;

    for val in arr.iter().map(|v| v.chars().nth(index as usize).unwrap()) {
        counter += val.to_digit(10).unwrap();
    }

    match g {
        Gas::Oxygen => {
            if counter as f32 >= (arr.len() as f32 / 2_f32) {
                '1'
            } else {
                '0'
            }
        }
        Gas::CO2 => {
            if counter as f32 >= (arr.len() as f32 / 2_f32) {
                '0'
            } else {
                '1'
            }
        }
    }
}

fn retain(arr: &[String], index: u32, ch: char) -> Vec<bool> {
    let mut retained: Vec<bool> = Vec::with_capacity(arr.len());
    for val in arr.iter().map(|v| v.chars().nth(index as usize).unwrap()) {
        if val == ch {
            retained.push(true);
        } else {
            retained.push(false);
        }
    }

    retained
}
