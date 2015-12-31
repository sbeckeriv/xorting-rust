extern crate bit_vec;

fn largest_bit(num: &i64) -> Option<usize> {
    if *num == 0 {
        return None;
    } else {
        println!("{:?}",
                 (num & 64, num & 32, num & 16, num & 8, num & 4, num & 2));
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
            // println!("Neg");
            None
        }
    }
}
fn option_to_bit(bool_bit: &Option<bool>) -> i64 {
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
fn xsort(vec: &mut Vec<i64>) -> Option<i64> {
    let mut empty: Vec<Option<bool>> = vec![None; 32];
    // println!("{}", empty.len());
    for i in 0..vec.len() - 1 {
        let n = vec[i] ^ vec[i + 1];
        let large = largest_bit(&n);
        match large {
            Some(largest) => {
                let bit_flag = vec[i] & (1 << largest) != 0;
                // println!("{}, {}, {}, {}, {} ",
                // vec[i],
                // vec[i + 1],
                // n,
                // largest,
                // bit_flag);
                //
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
    // println!("{:?}", empty);
    let answer: i64 = empty.iter().rev().fold(0, |acc, &b| acc * 2 + option_to_bit(&b));
    Some(answer)
}

fn main() {
    let mut t = vec![4, 7, 6, 1, 0, 3];
    let mut t2 = vec![4, 7, 1, 6, 0, 3];
    println!("{:?},", xsort(&mut t));
    println!("next");
    println!("{:?},", xsort(&mut t2));
}

fn load_test(path: std::path::PathBuf) -> (Option<i64>, Vec<i64>) {
    use std::env;
    use std::path::Path;
    use std::io::prelude::*;
    use std::fs::File;
    let mut open_file = File::open(path).unwrap();
    let mut buffer = String::new();
    open_file.read_to_string(&mut buffer).unwrap();
    let mut numbers = buffer.lines()
                            .map(|num| num.trim().parse::<i64>().unwrap())
                            .collect::<Vec<i64>>();

    let file_result = numbers.remove(0);
    let result = if file_result >= 0 {
        Some(file_result)
    } else {
        None
    };
    (result, numbers)

}
#[test]
fn it_works() {
    use std::fs;
    let paths = fs::read_dir("./examples").unwrap();
    for path in paths {
        let real_path = path.unwrap().path();
        println!("File: {}", real_path.display());
        let (result, mut vec) = load_test(real_path);
        println!("{:?}", vec);
        assert_eq!(result, xsort(&mut vec));
    }
    assert!(true);
}
