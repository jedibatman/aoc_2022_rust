use std::fs;
use itertools::Itertools;

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("Input file to be read");

    let elf_loads: Vec<i32> =
        contents
            .split("\n\n")
            .map(|s: &str| s.split_whitespace().map(|s: &str| s.parse::<i32>().unwrap()).sum())
            .sorted()
            .collect();

    for (i, elf_load) in elf_loads.iter().enumerate() {
        println!("elf {} caloric load: {}", i + 1, elf_load);
    }
}
