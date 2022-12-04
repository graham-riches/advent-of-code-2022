use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Range {
    lower: i32,
    upper: i32
}

// Implement from trait to convert raw tuple to Range
impl From<(i32, i32)> for Range {
    fn from((a, b): (i32, i32)) -> Self {
        Self{ lower: a, upper: b }
    }
}

impl Range {
    // Create a new range
    pub fn new(l: i32, u: i32) -> Range {
        Self{ lower: l, upper: u }
    }    

    // Create from string
    pub fn from_str(s: &str) -> Option<Self> {
        let t: (i32, i32) = s.split("-")
         .map(|x| x.parse::<i32>().unwrap())
         .next_tuple()
         .unwrap();
        Some(Range::from(t))
    }

    // Check if one range contains another
    pub fn contains(&self, other: &Range) -> bool {
        other.lower >= self.lower && other.upper <= self.upper
    }

    // Check if two ranges overlap at all
    pub fn overlaps(&self, other: &Range) -> bool {
        (self.lower >= other.lower && self.lower <= other.upper) ||
        (self.upper >= other.lower && self.lower <= other.lower)

    }
}

#[test]
fn test_range_contains() {
    let f = Range::new(1, 10);
    let s = Range::new(2, 9);
    assert_eq!(f.contains(&s), true);
}

#[test]
fn test_range_from_string() {
    assert_eq!(Range::from_str("7-8").unwrap(), Range::new(7, 8));
    assert_eq!(Range::from_str("1-5").unwrap(), Range::new(1, 5));
    assert_eq!(Range::from_str("14-33").unwrap(), Range::new(14, 33));
}

#[test]
fn test_ranges_overlap() {    
    assert_eq!(Range::new(1, 10).overlaps(&Range::new(2, 9)), true);
    assert_eq!(Range::new(7, 9).overlaps(&Range::new(8, 10)), true);
    assert_eq!(Range::new(1, 5).overlaps(&Range::new(0, 2)), true);
}