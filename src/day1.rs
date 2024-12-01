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

pub fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut v = Vec::with_capacity(1000);
    let mut v2 = Vec::with_capacity(1000);
    //let input = input.trim_start();
    let mut inp = input;
    let mut num;
    loop {
        if inp.is_empty() {
            return (v, v2);
        } else {
            (num, inp) = get_number(inp);
            v.push(num);

            (num, inp) = get_number(inp);

            v2.push(num);
        }
    }
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

#[inline(always)]
pub fn count_values(arr: &[u32], idx: usize) -> u32 {
    let value = arr[idx];

    let mut start = idx;
    while start > 0 && arr[start - 1] == value {
        start -= 1;
    }

    let mut stop = idx;
    while stop < arr.len() - 1 && arr[stop + 1] == value {
        stop += 1;
    }

    (stop - start) as u32 + 1
}

pub fn bin_search_count(arr: &[u32], value: u32) -> u32 {
    let (mut left, mut right) = (0, arr.len() - 1);
    if arr[left] == value {
        return count_values(arr, left);
    }
    if arr[right] == value {
        return count_values(arr, right);
    }
    let mut mid;
    while left <= right {
        mid = left + (right - left) / 2;

        match value.cmp(&arr[mid]) {
            std::cmp::Ordering::Less => {
                right = if mid > 0 {
                    mid - 1
                } else {
                    return 0;
                }
            }
            std::cmp::Ordering::Greater => left = mid + 1,
            std::cmp::Ordering::Equal => return count_values(arr, mid),
        }
    }
    0
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let (left, mut right) = input_generator(input);
    right.sort();
    let mut prev_scores = HashMap::with_capacity(1000);
    let res = left.iter().fold(0, |similarity_score, l| {
        similarity_score
            + *{
                prev_scores
                    .entry(l)
                    .or_insert_with(|| l * bin_search_count(&right, *l))
            }
    });
    res
}
