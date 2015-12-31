extern crate bit_vec;
fn largest_bit(num: &u64) -> Option<usize> {
    if *num == 0 {
        return None;
    } else {
        let bit = match (num & 64, num & 32, num & 16, num & 8, num & 4, num & 2) {
            (_, _, _, _, _, 0) => 0,
            (_, _, _, _, 0, _) => 1,
            (_, _, _, 0, _, _) => 2,
            (_, _, 0, _, _, _) => 3,
            (_, 0, _, _, _, _) => 4,
            (0, _, _, _, _, _) => 5,
            (_, _, _, _, _, _) => 0 - 1,
        };
        if bit > 0 - 1 {
            Some(bit as usize)
        } else {
            println!("Neg");
            None
        }
    }
}
fn option_to_bit(bool_bit: &Option<bool>) -> u64 {
    match *bool_bit {
        Some(b) => {
            if b {
                1
            } else {
                0
            }
        }
        None => 0,
    }
}
fn xsort(vec: &mut Vec<u64>) -> Option<u64> {
    let mut empty: Vec<Option<bool>> = vec![None; 32];
    println!("{}", empty.len());
    for i in 0..vec.len() - 1 {
        let n = vec[i] ^ vec[i + 1];
        let large = largest_bit(&n);
        match large {
            Some(largest) => {
                let bit_flag = vec[i] & (1 << largest) != 0;
                println!("{}, {}, {}, {}, {} ",
                         vec[i],
                         vec[i + 1],
                         n,
                         largest,
                         bit_flag);
                if empty[largest].is_some() && empty[largest].unwrap() != bit_flag {
                    println!("break {:?}\n{},{} ", empty, largest, bit_flag);
                    return None;
                } else {
                    empty[largest] = Some(bit_flag);
                }
            }
            None => println!("skip"),
        }
    }
    println!("{:?}", empty);
    let mut answer: u64 = 0;
    let mut count = 0;
    let answer: u64 = empty.iter().rev().fold(0, |acc, &b| acc * 2 + option_to_bit(&b));
    Some(answer)
}

fn main() {
    let mut t = vec![4, 7, 6, 1, 0, 3];
    let mut t2 = vec![4, 7, 1, 6, 0, 3];
    println!("{:?},", xsort(&mut t));
    println!("next");
    println!("{:?},", xsort(&mut t2));
}
