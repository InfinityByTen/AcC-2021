use std::fs;
use to_binary::BinaryString;

fn parse_literal(cursor: &mut dyn Iterator<Item = &char>) -> (usize, usize) {
    let mut count = 6;
    let res: usize;
    loop {
        let val = (0..5).map(|_| cursor.next().unwrap()).collect::<String>();
        count += 5;
        if &val[0..1] == "0" {
            res = usize::from_str_radix(&val, 2).unwrap();
            break;
        }
    }
    (res, count)
}

fn operate<'a>(operands: &Vec<usize>, id: &str) -> usize {
    let id_val = usize::from_str_radix(id, 2).unwrap();

    match id_val {
        0 => operands.iter().sum(),
        1 => operands.iter().product(),
        2 => operands.iter().cloned().min().unwrap(),
        3 => operands.iter().cloned().max().unwrap(),
        // 4=> ,
        5 => ((operands[0] > operands[1]) as usize),
        6 => ((operands[0] < operands[1]) as usize),
        7 => ((operands[0] == operands[1]) as usize),
        _ => unreachable!(),
    }
}

#[allow(unused_assignments)]
fn parse_packet(
    cursor: &mut dyn Iterator<Item = &char>,
    start_tracking_used: bool,
) -> (usize, usize, usize) {
    let version = (0..3).map(|_| cursor.next().unwrap()).collect::<String>();
    let type_id = (0..3).map(|_| cursor.next().unwrap()).collect::<String>();
    // println!("{:?}", (&version, &type_id));

    let mut ver = usize::from_str_radix(&version, 2).unwrap();

    let mut used = 0;
    let mut out = 0;
    if start_tracking_used {
        used += 6;
    }
    match type_id.as_str() {
        "100" => {
            // println!("Literal");
            let res = parse_literal(cursor);
            // println!("{:?}", res.0);
            return (ver, res.0, res.1);
        }
        op => {
            // println!("Operator {:?}", usize::from_str_radix(op, 2).unwrap());
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
                    let mut operands = Vec::new();
                    while limit > 0 {
                        // println!("Limit: {:?}", limit);
                        // println!("Used before starting {:?}", used);
                        let res = parse_packet(cursor, true);
                        // println!("used up {:?} bits", res.1);
                        limit -= res.2;
                        ver += res.0;
                        used += res.2;
                        // println!("Used before after one Operator {:?}", used);
                        operands.push(res.1);
                    }
                    // println!("{:?}", (&operands, usize::from_str_radix(op, 2).unwrap()));
                    out = operate(&operands, op);
                    // println!("{:?}", out);
                }
                _ => {
                    let count = (0..11).map(|_| cursor.next().unwrap()).collect::<String>();
                    if start_tracking_used {
                        used += 11;
                    }
                    let limit = usize::from_str_radix(&count, 2).unwrap();
                    // println!("Has packet count {:?}", limit);
                    // println!("Used before starting {:?}", used);
                    let operands = (0..limit)
                        .map(|_| {
                            let res = parse_packet(cursor, start_tracking_used);
                            // println!("used up {:?} bits", res.1);
                            ver += res.0;
                            if start_tracking_used {
                                used += res.2;
                            }
                            res.1
                        })
                        .collect::<Vec<usize>>();
                    // println!("Used before after one Operator {:?}", used);
                    // println!("{:?}", (&operands, usize::from_str_radix(op, 2).unwrap()));
                    out = operate(&operands, op);
                }
            }
        }
    }
    // println!("Final {:?}", ver);
    (ver, out, used)
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let stream = BinaryString::from_hex(buf).unwrap().to_string();
    let buffer = stream.chars().collect::<Vec<char>>();
    let mut cursor = buffer.iter();
    println!("{:?}", parse_packet(&mut cursor, false));
}
