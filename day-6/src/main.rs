use std::fs;
use std::collections::{VecDeque, HashSet};

// In anticipation of part two not being able to do a quick lookup on a length 4 list, 
// go straight for the hashset solution. This keeps a queue of items in order to be able to
// easily push/pop new and old items from the list, but uses a hashset for fast lookup
// to see if the sequence contains repeated characters
struct FrameScanner {
    seq: VecDeque<char>,
    map: HashSet<char>,
    max: usize,
}

impl FrameScanner {
    // Create a new frame scanner with a fixed size
    pub fn new(size: usize) -> Self {
        Self {
            seq: VecDeque::with_capacity(size),
            map: HashSet::with_capacity(size),
            max: size
        }
    }

    // Flush the buffer until the first character matching "c" is removed
    fn flush_until(&mut self, c: &char) -> () {
        while let Some(x) = self.seq.pop_front() {
            self.map.remove(&x);
            if x == *c { break; }
        }
    }

    // Inserts a new item and returns the current length
    fn insert(&mut self, c: &char) -> usize {
        // If at capacity, rotate one item out
        if self.seq.len() == self.max {
            let last = self.seq.pop_front().unwrap();
            self.map.remove(&last);
        } else {
            // Insert new element
            self.seq.push_back(*c);
            self.map.insert(*c);
        }
        self.seq.len()        
    }

    // Scan until sequence of length max size is found that is all unique
    pub fn scan(&mut self, buffer: &str) -> Option<usize> {        
        let mut count: i32 = 1;
        for c in buffer.chars() {
            if self.map.contains(&c) {
                self.flush_until(&c);
            }
            let current_size = self.insert(&c);
            if current_size == self.max {
                return Some(count as usize);
            }
            count += 1;
        }
        None
    }

    // Reset the scanner to restart the search
    pub fn reset(&mut self) -> () {
        self.seq.clear();
        self.map.clear();
    }
}


fn main() {
    let buffer = fs::read_to_string("input.txt").unwrap();
    let p1 = find_start_of_frame(&buffer);
    println!("Part one: {}", p1);

    let p2 = find_start_of_message(&buffer);
    println!("Part two: {}", p2);
}

fn find_start_of_frame(buffer: &str) -> usize {
    let mut scanner = FrameScanner::new(4);
    scanner.scan(buffer).unwrap()    
}

fn find_start_of_message(buffer: &str) -> usize {
    let mut scanner = FrameScanner::new(14);
    scanner.scan(buffer).unwrap()    
}


#[test]
fn test_find_start_of_frame() {
    assert_eq!(find_start_of_frame("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(find_start_of_frame("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(find_start_of_frame("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(find_start_of_frame("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(find_start_of_frame("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}

#[test]
fn test_find_start_of_message() {
    assert_eq!(find_start_of_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(find_start_of_message("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(find_start_of_message("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(find_start_of_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(find_start_of_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}