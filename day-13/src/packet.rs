use std::fmt;

// Packet is either an int, or a list of packets
#[derive(Clone)]
pub enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

// Custom debug print trait implementation for debugging
impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Packet::Int(x) => write!(f, "{}", x)?,
            Packet::List(l) => {
                write!(f, "[")?;
                for i in 0..l.len() {
                    if i < l.len() - 1 {
                        write!(f, "{:?},", l[i])?;
                    } else {
                        write!(f, "{:?}", l[i])?;
                    }
                }
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}

// Custom implmentation of PartialEq trait for the main puzzle logic
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match &self {
            Packet::Int(x) => match other {
                Packet::Int(y) => x == y,
                Packet::List(_) => false,
            },
            Packet::List(x) => match other {
                Packet::Int(_) => false,
                Packet::List(y) => {
                    if x.len() != y.len() {
                        return false;
                    }
                    x.iter().zip(y.iter()).map(|(x, y)| x == y).all(|x| x)
                }
            },
        }
    }
}

// Implement eq (uses PartialEq by defaul)
impl Eq for Packet {}

// Implement PartialOrd, which uses Ord
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Implement Ord trait to compare packets based on challenge rules
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match &self {
            Packet::Int(x) => match other {
                Packet::Int(y) => x.cmp(&y),
                Packet::List(_) => Packet::List(vec![Packet::Int(*x)]).cmp(other),
            },
            Packet::List(x) => match other {
                Packet::Int(y) => self.cmp(&Packet::List(vec![Packet::Int(*y)])),
                Packet::List(y) => {
                    let mut ix = x.iter();
                    let mut iy = y.iter();
                    loop {
                        match (ix.next(), iy.next()) {
                            (Some(x_val), Some(y_val)) => match x_val.cmp(y_val) {
                                std::cmp::Ordering::Less    => return std::cmp::Ordering::Less,
                                std::cmp::Ordering::Equal   => continue,
                                std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                            },
                            (Some(_), None) => return std::cmp::Ordering::Greater,
                            (None, Some(_)) => return std::cmp::Ordering::Less,
                            (None, None)    => return std::cmp::Ordering::Equal,
                        }
                    }
                }
            },
        }
    }
}

#[test]
fn test_equality_comparison() {
    assert_eq!(Packet::Int(1), Packet::Int(1));
    assert_ne!(Packet::Int(2), Packet::Int(1));
    assert_eq!(
        Packet::List(vec![Packet::Int(7), Packet::Int(7)]),
        Packet::List(vec![Packet::Int(7), Packet::Int(7)])
    );
    assert_ne!(Packet::List(vec![Packet::Int(2)]), Packet::Int(1));
    assert_ne!(Packet::Int(1), Packet::List(vec![Packet::Int(2)]));
}

#[test]
fn test_less_comparison() {
    assert_ne!(
        Packet::List(vec![Packet::Int(7), Packet::Int(7)]) < Packet::List(vec![Packet::Int(7)]),
        true
    );
    let l1 = Packet::List(vec![
        Packet::List(vec![Packet::Int(1)]),
        Packet::List(vec![Packet::Int(2), Packet::Int(3), Packet::Int(4)]),
    ]);
    let l2 = Packet::List(vec![Packet::List(vec![Packet::Int(1)]), Packet::Int(4)]);
    assert_eq!(l1 < l2, true);
}
