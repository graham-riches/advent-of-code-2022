pub mod packet;
use packet::Packet;

fn main() {
    let packet_pairs = std::fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .map(|y| parse_packet(y))
                .collect::<Vec<Packet>>()
        })
        .map(|packets| (packets[0].clone(), packets[1].clone()))
        .collect::<Vec<(Packet, Packet)>>();

    // Part one
    let p1: usize = packet_pairs
        .iter()
        .zip(1..)
        .flat_map(|(packets, index)| if packets.0 < packets.1 { Some(index) } else { None })
        .sum();
    println!("Part one: {:?}", p1);

    // Part two
    let mut packets = Vec::new();
    for pair in packet_pairs {
        packets.push(pair.0);
        packets.push(pair.1);
    }
    let div_a = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let div_b = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
    packets.push(div_a.clone());
    packets.push(div_b.clone());
    packets.sort();
    let i1 = packets.iter().zip(1..).find(|(x, _)| *x == &div_a).unwrap();
    let i2 = packets.iter().zip(1..).find(|(x, _)| *x == &div_b).unwrap();
    println!("Part two: {}", i1.1 * i2.1);
}

// Gets the next valid packet sequence out of the current list
fn get_next_packet_string(s: &str) -> String {
    let mut n = 1;
    let mut str = Vec::new();
    for c in s.chars() {
        str.push(c);
        match c {
            '[' => {
                n += 1;
            }
            ']' => {
                n -= 1;
                if n == 0 {
                    return str.into_iter().collect();
                }
            }
            _ => (),
        }
    }
    str.into_iter().collect()
}


// Parses a string into a packet 
fn parse_packet(s: &str) -> Packet {
    let (_, p) = parse_inner(s);
    if p.len() > 1 {
        Packet::List(p)
    } else {
        p[0].clone()
    }
}

// Parses an inner packet, returns tuple of: num characters parsed, and the Packet parsed
fn parse_inner(s: &str) -> (usize, Vec<Packet>) {
    let mut p = Vec::new();
    let mut str = Vec::new();
    let mut count = 0;
    let mut iter = s.chars();
    while let Some(c) = iter.next() {
        count += 1;
        match c {
            '[' => {
                let from = s.chars().skip(count).collect::<String>();
                let new = get_next_packet_string(&from);
                let (used, packets) = parse_inner(new.as_str());
                p.push(Packet::List(packets));
                for _ in 0..used + 1 {
                    iter.next();
                    count += 1;
                }
                str = Vec::new();
            }
            ']' => (),
            ',' => {
                if str.len() > 0 {
                    let new: String = str.iter().collect();
                    let (_, packets) = parse_inner(new.as_str());
                    for packet in packets {
                        p.push(packet);
                    }
                }
                str = Vec::new();
            }
            _ => str.push(c),
        }
    }

    if str.len() > 0 {
        let items = str.iter().collect::<String>();
        let packets = items
            .split(",")
            .flat_map(|x| x.parse::<i32>().ok())
            .map(|x| Packet::Int(x))
            .collect::<Vec<Packet>>();
        for packet in packets {
            p.push(packet);
        }
    }
    return (count, p);
}
