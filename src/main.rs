use std::io::{BufReader, BufRead};
use std::fs::File;

fn part_one() -> usize {
    let mut reader = BufReader::new(File::open("input.txt").unwrap());

    let length_signal = reader.lines().nth(0).unwrap().unwrap().len();

    reader = BufReader::new(File::open("input.txt").unwrap()); // find out how to read first line properly

    let (ones, count) = reader.lines().map(|l| l.unwrap()).fold(
        (vec![0; length_signal], 0), |(mut buckets, count), line| {
            line.chars().enumerate().for_each(|(i, c)| if c == '1' {buckets[i] += 1;} );
            return (buckets, count + 1);
        }
    );

    let (gamma, epsilon) = ones.into_iter().rev().enumerate().fold((0, 0), |(gamma, epsilon), (base, freq)| {
        if freq > count / 2 {
            return (gamma + (1 << base), epsilon);
        }
        return (gamma, epsilon + (1 << base));
    });

    gamma * epsilon
}

fn part_two() -> usize {
    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap())
    .lines().map(|l| l.unwrap()).collect();

    let signal_length = lines[0].len();

    let mut buf = lines.clone();

    for index in 0..signal_length
    {
        if 2 * calculate_one_frequency(&mut buf, index) >= buf.len() {
            buf.retain(|line| line.chars().nth(index).unwrap() == '1');
        } else {
            buf.retain(|line| line.chars().nth(index).unwrap() == '0');
        }
        if buf.len() == 1 {break;}
    };

    let oxygen = usize::from_str_radix(&buf[0], 2).unwrap();

    buf = lines.clone();

    for index in 0..signal_length
    {
        if 2 * calculate_one_frequency(&mut buf, index) >= buf.len() {
            buf.retain(|line| line.chars().nth(index).unwrap() == '0');
        } else {
            buf.retain(|line| line.chars().nth(index).unwrap() == '1');
        }
        if buf.len() == 1 {break;}
    };

    let co2 = usize::from_str_radix(&buf[0], 2).unwrap();

    oxygen * co2
}

fn calculate_one_frequency(buf: &Vec<String>, index: usize) -> usize {

    return buf.iter().fold(
        0, |count, line| 
        if line.chars().nth(index).unwrap() == '1' {
            return count + 1;
        } else {return count;});
}

fn main() {
    println!("part one {}", part_one());
    println!("part two {}", part_two());

}
