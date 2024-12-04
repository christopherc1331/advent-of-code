use std::{collections::HashMap, fs};

pub fn solution_a() -> i32 {
    let contents = fs::read_to_string("src/day_2/input.txt").expect("to read file");
    let lines: Vec<Vec<i32>> = contents
        .split('\n')
        .map(|x| {
            x.split(' ')
                .filter(|y| !y.is_empty())
                .map(|f| f.parse::<i32>().unwrap())
                .collect()
        })
        .filter(|y: &Vec<i32>| !y.is_empty())
        .collect();

    let mut ct = 0;

    for row in lines {
        let mut is_safe = true;
        let is_ascending = row.first() < row.get(1);
        match is_ascending {
            true => {
                for col_idx in 1..row.len() {
                    if !is_safe {
                        break;
                    }
                    let diff = row.get(col_idx).unwrap() - row.get(col_idx - 1).unwrap();
                    is_safe = matches!(diff, 1..=3);
                }
            }
            false => {
                for col_idx in 0..row.len() - 1 {
                    if !is_safe {
                        break;
                    }
                    let diff = row.get(col_idx).unwrap() - row.get(col_idx + 1).unwrap();
                    is_safe = matches!(diff, 1..=3);
                }
            }
        }
        if is_safe {
            ct += 1;
        }
    }

    ct
}

pub fn solution_b() -> i32 {
    let contents = fs::read_to_string("src/day_1/input.txt").expect("to read file");
    let lines: Vec<Vec<&str>> = contents
        .split('\n')
        .map(|x| x.split("   ").collect())
        .filter(|x: &Vec<&str>| x.len() == 2)
        .collect();

    let mut left: Vec<i32> = Vec::default();
    let mut map: HashMap<i32, i32> = HashMap::new();
    for line in lines {
        let left_num = line.first().unwrap().parse::<i32>().unwrap();
        left.push(left_num);
        let right_num = line.get(1).unwrap().parse::<i32>().unwrap();
        map.entry(right_num)
            .and_modify(|num| *num += 1)
            .or_insert(1);
    }

    left.into_iter()
        .fold(0, |acc, num| acc + num * map.get(&num).unwrap_or(&0))
}
