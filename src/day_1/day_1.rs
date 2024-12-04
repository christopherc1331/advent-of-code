use std::{collections::HashMap, fs};

pub fn solution_a() -> i32 {
    let contents = fs::read_to_string("src/day_1/input.txt").expect("to read file");
    let lines: Vec<Vec<&str>> = contents
        .split('\n')
        .map(|x| x.split("   ").collect())
        .filter(|x: &Vec<&str>| x.len() == 2)
        .collect();

    let mut left: Vec<&str> = Vec::default();
    let mut right: Vec<&str> = Vec::default();
    for line in lines {
        left.push(line.first().unwrap());
        right.push(line.get(1).unwrap());
    }

    left.sort();
    right.sort();
    let mut res: i32 = 0;
    for i in 0..left.len() {
        res += (left.get(i).unwrap().parse::<i32>().unwrap()
            - right.get(i).unwrap().parse::<i32>().unwrap())
        .abs();
    }
    res
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
