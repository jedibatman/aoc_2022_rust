use std::{fs, collections::{HashMap, HashSet}};
use itertools::Itertools;

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("input file to be read");

    let alphabet = String::from_utf8(
        (b'a'..=b'z').chain(b'A'..=b'Z').collect()
    ).unwrap();

    let priorities = 1..((alphabet.len() + 1) as i32);

    let priority_map: HashMap<char, i32> = alphabet.chars().into_iter().zip(priorities).collect();

    let priority_sum =
        contents.lines()
        .map(|s | {
            let center = s.len() / 2;
            let (compartment_1, compartment_2) = (&s[..center], &s[center..]);

            return (compartment_1, compartment_2);
        })
        .fold(0,|acc, compartment_iter| {
            let (compartment_1, compartment_2) = compartment_iter;

            let matching_char: char = find_match(compartment_1, compartment_2) as char;

            let char_priority = priority_map.get(&matching_char).unwrap();

            return acc + char_priority;
        });

    let badge_priority_sum =
        contents.lines()
        .chunks(3)
        .into_iter()
        .map(|mut rucksacks| {
            let (rucksack_1, rucksack_2, rucksack_3) =
                rucksacks.next_tuple()
                .unwrap();

            return (rucksack_1, rucksack_2, rucksack_3);
        })
        .fold(0, |acc, rucksack_iter| {
            let(rucksack_1, rucksack_2, rucksack_3) = rucksack_iter;

            let matching_char: char = find_group_match(rucksack_1, rucksack_2, rucksack_3) as char;

            let char_priority = priority_map.get(&matching_char).unwrap();

            return acc + char_priority;
        });

    println!("The sum of all priorities is {priority_sum} (part 1)");
    println!("The sum of all badge priorities is {badge_priority_sum} (part 2)");
}

fn find_match(this_compartment: &str, that_compartment: &str) -> u8 {
    let this_compartment_chars: HashSet<u8> = HashSet::from_iter(this_compartment.as_bytes().iter().cloned());
    let that_compartment_chars: HashSet<u8> = HashSet::from_iter(that_compartment.as_bytes().iter().cloned());

    let intersection = this_compartment_chars.intersection(&that_compartment_chars)
    .cloned()
    .at_most_one()
    .unwrap();

    return intersection.unwrap();
}

fn find_group_match(this_sack: &str, that_sack: &str, other_sack: &str) -> u8 {
    let this_sack_chars: HashSet<u8> = HashSet::from_iter(this_sack.as_bytes().iter().cloned());
    let that_sack_chars: HashSet<u8> = HashSet::from_iter(that_sack.as_bytes().iter().cloned());
    let other_sack_chars: HashSet<u8> = HashSet::from_iter(other_sack.as_bytes().iter().cloned());

    let left_intersection = this_sack_chars.intersection(&that_sack_chars);
    let right_intersection = this_sack_chars.intersection(&other_sack_chars);

    let left_intersect_chars: HashSet<u8> = HashSet::from_iter(left_intersection.into_iter().cloned());
    let right_intersect_chars: HashSet<u8> = HashSet::from_iter(right_intersection.into_iter().cloned());

    let intersection = left_intersect_chars.intersection(&right_intersect_chars)
    .cloned()
    .at_most_one()
    .unwrap();

    return intersection.unwrap();
}
