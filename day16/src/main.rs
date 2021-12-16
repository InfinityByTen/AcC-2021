use std::fs;
use to_binary::BinaryString;

fn parse_literal(cursor: &mut dyn Iterator<Item = &char>) -> usize {
    let mut count = 6;
    loop {
        let val = (0..5).map(|_| cursor.next().unwrap()).collect::<String>();
        // println!("{:?}", val);
        count += 5;
        if &val[0..1] == "0" {
            break;
        }
    }
    count
}

fn parse_packet(
    cursor: &mut dyn Iterator<Item = &char>,
    start_tracking_used: bool,
) -> (usize, usize) {
    let version = (0..3).map(|_| cursor.next().unwrap()).collect::<String>();
    let type_id = (0..3).map(|_| cursor.next().unwrap()).collect::<String>();
    // println!("{:?}", (&version, &type_id));

    let mut ver = usize::from_str_radix(&version, 2).unwrap();

    let mut used = 0;
    if start_tracking_used {
        used += 6;
    }
    match type_id.as_str() {
        "100" => {
            // println!("Literal");
            return (ver, parse_literal(cursor));
        }
        _ => {
            // println!("Operator");
            let length_id = cursor.next().unwrap();
            if start_tracking_used {
                used += 1;
            }

            match length_id {
                '0' => {
                    let length = (0..15).map(|_| cursor.next().unwrap()).collect::<String>();
                    if start_tracking_used {
                        used += 15;
                    }
                    let mut limit = usize::from_str_radix(&length, 2).unwrap();
                    // println!("Has total length {:?}", limit);
                    while limit > 0 {
                        // println!("Limit: {:?}", limit);
                        // println!("Used before starting {:?}", used);
                        let res = parse_packet(cursor, true);
                        // println!("used up {:?} bits", res.1);
                        limit -= res.1;
                        ver += res.0;
                        used += res.1;
                        // println!("Used before after one Operator {:?}", used);
                    }
                }
                _ => {
                    let count = (0..11).map(|_| cursor.next().unwrap()).collect::<String>();
                    if start_tracking_used {
                        used += 11;
                    }
                    let limit = usize::from_str_radix(&count, 2).unwrap();
                    // println!("Has packet count {:?}", limit);
                    // println!("Used before starting {:?}", used);
                    (0..limit).for_each(|_| {
                        let res = parse_packet(cursor, start_tracking_used);
                        // println!("used up {:?} bits", res.1);
                        ver += res.0;
                        if start_tracking_used {
                            used += res.1;
                        }
                    });
                    // println!("Used before after one Operator {:?}", used);
                }
            }
        }
    }
    // println!("Final {:?}", ver);
    (ver, used)
}

fn main() {
    let buf = fs::read_to_string("./test.txt").unwrap();
    let stream = BinaryString::from_hex(buf).unwrap().to_string();
    let buffer = stream.chars().collect::<Vec<char>>();
    let mut cursor = buffer.iter();
    println!("{:?}", parse_packet(&mut cursor, false));
}
