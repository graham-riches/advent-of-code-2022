extern crate utilities;
use std::collections::HashSet;

fn main() {
    let ruck_sacks = utilities::lines_from_file("input.txt").unwrap();    
    let p1 = ruck_sacks.iter()
        .map(|x| calculate_priority_score(&x))
        .fold(0, |sum, y| sum + y);
    println!("Part one: {}", p1);

    let p2 = ruck_sacks
        .chunks(3)
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<_>>>()
        .iter()
        .map(|x| get_group_badge_priority(&x))
        .fold(0, |sum, p| sum + p);
    println!("Part two: {}", p2);
}

// Split string into two, hashset first, return first value in second contained in first
fn calculate_priority_score(line: &str) -> i32 {    
    let (f, s): (&str, &str) = line.split_at(line.chars().count() / 2);
    let set: HashSet<char> = f.chars().collect();
    for c in s.chars() {
        if set.contains(&c) {
            return char_to_int(&c)
        }
    }
    0
}

// Map char values to int "priority" score
fn char_to_int(c: &char) -> i32 {
    match c {
        'a'..='z' => *c as i32 - 'a' as i32 + 1,
        'A'..='Z' => *c as i32 - 'A' as i32 + 27,
        _ => 0,
    }    
}

// Return the single intersection of 3 strings (use hashset intersect)
fn get_group_badge_priority(group: &Vec<String>) -> i32 {
    let sets = group.iter()
        .map(|x| x.chars().collect::<HashSet<char>>())
        .collect::<Vec<_>>();
    let mut iter = sets.into_iter();
    let vec = iter.next()
        .map(|set| iter.fold(set, |s1, s2| &s1 & &s2))
        .unwrap()
        .into_iter()
        .map(|x| char_to_int(&x))
        .collect::<Vec<_>>();
    vec[0]
}

// Unit test to sample values
#[test]
fn test_char_to_int() {
    assert_eq!(char_to_int(&'a'), 1);
    assert_eq!(char_to_int(&'L'), 38);
    assert_eq!(char_to_int(&'P'), 42);
    assert_eq!(char_to_int(&'v'), 22);
    assert_eq!(char_to_int(&'t'), 20);
    assert_eq!(char_to_int(&'s'), 19);
}

#[test]
fn test_calculate_priority_score() {
    assert_eq!(calculate_priority_score("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
    assert_eq!(calculate_priority_score("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 38);
    assert_eq!(calculate_priority_score("PmmdzqPrVvPwwTWBwg"), 42);
    assert_eq!(calculate_priority_score("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 22);
    assert_eq!(calculate_priority_score("ttgJtRGJQctTZtZT"), 20);
    assert_eq!(calculate_priority_score("CrZsJsPPZsGzwwsLwLmpwMDw"), 19);
}

#[test]
fn test_get_group_badge_priority() {
    let v = vec!["vJrwpWtwJgWrhcsFMMfFFhFp".to_string(), "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(), "PmmdzqPrVvPwwTWBwg".to_string()];
    assert_eq!(get_group_badge_priority(&v), 18);
}