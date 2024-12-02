use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

#[inline(always)]
pub fn get_number(input: &str) -> (u32, &str) {
    for i in 0..input.len() {
        if !(input.as_bytes()[i] as char).is_numeric() {
            return (input[0..i].parse::<u32>().unwrap(), input[i..].trim_start());
        }
    }
    (
        input[0..input.len()].parse::<u32>().unwrap(),
        input[input.len()..].trim_start(),
    )
}
#[inline(always)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut v = Vec::with_capacity(1000);
    let mut v2 = Vec::with_capacity(1000);
    //let input = input.trim_start();
    let mut inp = input;
    let mut num = 0;
    (0..1000).for_each(|_| {
        (num, inp) = get_number(inp);
        v.push(num);

        (num, inp) = get_number(inp);

        v2.push(num);
    });
    (v, v2)
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let (mut left, mut right) = input_generator(input);
    left.sort();
    right.sort();
    let mut pairs = HashSet::with_capacity(1000);
    left.iter().zip(right.iter()).fold(0, |sum, (l, r)| {
        sum + {
            if pairs.contains(&(l, r)) || l == r {
                0
            } else {
                pairs.insert((l, r));
                if l > r {
                    l - r
                } else {
                    r - l
                }
            }
        }
    })
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut left = Vec::with_capacity(1000);
    let mut right_counter = HashMap::with_capacity(1000);
    //let input = input.trim_start();
    let mut inp = input;
    let mut num = 0;
    (0..1000).for_each(|_i: i32| {
        (num, inp) = get_number(inp);
        left.push(num);

        (num, inp) = get_number(inp);

        right_counter
            .entry(num)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    });

    left.iter().fold(0, |similarity_score, l| {
        similarity_score + { l * right_counter.get(l).unwrap_or_else(|| &0) }
    })
}
