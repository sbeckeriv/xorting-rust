extern crate time;

fn largest_bit(num: &i64) -> Option<usize> {
    if *num == 0 {
        return None;
    } else {
        let bit = format!("{:b}", num).len() - 1;
        if bit >= 0 {
            Some(bit as usize)
        } else {
            None
        }
    }
}

fn option_to_bit(bool_bit: &Option<bool>) -> i64 {
    match *bool_bit {
        Some(b) => {
            match b {
                true => 1,
                false => 0,
            }
        }
        None => 0,
    }
}
fn xsort(vec: &mut Vec<i64>) -> Option<i64> {
    // numbers never larger then 32bits
    let mut empty: Vec<Option<bool>> = vec![None; 32];
    for i in 0..vec.len() - 1 {
        let n = vec[i] ^ vec[i + 1];
        match largest_bit(&n) {
            Some(largest) => {
                let bit_flag = vec[i] & (1 << largest) != 0;
                if empty[largest].is_some() && empty[largest].unwrap() != bit_flag {
                    return None;
                } else {
                    empty[largest] = Some(bit_flag);
                }
            }
            None => {}
        }
    }
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

fn load_test(path: &std::path::PathBuf) -> (Option<i64>, Vec<i64>) {
    use std::env;
    use std::path::Path;
    use std::io::prelude::*;
    use std::fs::File;
    let mut open_file = File::open((*path).clone()).unwrap();
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
        let (result, mut vec) = load_test(&real_path);
        let t = time::now();
        let x_result = xsort(&mut vec);
        let t2 = time::now();
        assert_eq!(result, x_result);
        println!("File {} with {} numbers took {:?} result {:?}",
                 real_path.display(),
                 vec.len(),
                 t2 - t,
                 x_result);
    }
    assert!(true);
}
