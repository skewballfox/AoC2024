use std::process::exit;

use aoc_runner_derive::aoc;
const SAFE_RANGE: std::ops::Range<u32> = 1..4;
const INV_RANGE: std::ops::Range<u32> = (u32::MAX - 3)..u32::MAX;
#[repr(usize)]
#[derive(Default, Clone, Copy)]
enum PositionIndex {
    First,
    Second,
    #[default]
    Third,
}

impl PositionIndex {
    fn next(&self) -> Self {
        match self {
            Self::First => Self::Second,
            Self::Second => Self::Third,
            Self::Third => Self::First,
        }
    }
}

#[derive(Default)]
pub struct StateTracker {
    free_level: bool,
    increasing: bool,
    current: PositionIndex,
    data: [u32; 3],
}
fn parse_tens(input: &[u8], start: usize) -> u32 {
    unsafe {
        ((*input.get_unchecked(start) as u32) - 48) * 10
            + ((*input.get_unchecked(start + 1) as u32) - 48)
    }
}

fn parse_single(input: &[u8], start: usize) -> u32 {
    unsafe { (*input.get_unchecked(start) as u32) - 48 }
}

fn parse_digit(input: &[u8], start: usize) -> (u32, usize) {
    if start + 1 == input.len() || input[start + 1].is_ascii_whitespace() {
        (parse_single(input, start), start + 1)
    } else {
        (parse_tens(input, start), start + 2)
    }
}

fn next_line(input: &[u8], mut start: usize) -> usize {
    loop {
        if start >= input.len() || input[start] == b'\n' {
            return start + 1;
        } else {
            start += 1;
        }
    }
}

fn is_valid(first: u32, second: u32, increasing: bool) -> bool {
    let diff = if increasing {
        second - first
    } else {
        first - second
    };
    SAFE_RANGE.contains(&diff)
}

fn grab_two(input: &[u8], mut start: usize) -> (u32, u32, usize) {
    let (first, start) = parse_digit(input, start);
    let (second, start) = parse_digit(input, start + 1);
    (first, second, start)
}

impl StateTracker {
    fn check_next(&mut self, value: u32) -> bool {
        if is_valid(self.data[self.current as usize], value, self.increasing) {
            self.data[self.current.next() as usize] = value;
            self.current = self.current.next();
            true
        } else if self.free_level {
            self.free_level = false;
            true
        } else {
            false
        }
    }
    fn grab_four(&mut self, input: &[u8], mut start: usize) -> (bool, usize) {
        self.free_level = true;
        self.current = PositionIndex::Third;
        //we know the inputs are separated by a space, and are two digits in width
        for i in 0..3 {
            (self.data[i], start) = parse_digit(input, start);
            start += 1;
        }
        let (fourth, start) = parse_digit(input, start);

        let (increasing, replace_level, index) = Self::match_diffs(
            self.data[1] - self.data[0],
            self.data[2] - self.data[1],
            fourth - self.data[2],
            fourth - self.data[1],
            self.data[2] - self.data[0],
        );

        let mut replace_idx = 4;

        if increasing {
            self.increasing = true;
            if replace_level {
                self.free_level = false;
                replace_idx = index;
            }
        } else {
            let (decreasing, replace_level, index) = Self::match_diffs(
                self.data[0] - self.data[1],
                self.data[1] - self.data[2],
                self.data[2] - fourth,
                self.data[1] - fourth,
                self.data[0] - self.data[2],
            );

            if decreasing {
                self.increasing = false;
                if replace_level {
                    self.free_level = false;
                    replace_idx = index;
                }
            } else {
                return (false, start);
            }
        }

        if !self.free_level {
            match replace_idx {
                0 => {
                    self.current = PositionIndex::First;
                    self.data[0] = fourth;
                }
                1 => {
                    self.data[1] = self.data[2];
                    self.data[2] = fourth;
                }
                2 => {
                    self.current = PositionIndex::Second;
                    self.data[2] = fourth;
                }
                _ => {} //do nothing if we don't need to replace
            }
        } else {
            self.data[0] = fourth;
            self.current = PositionIndex::First;
        }

        (true, start)
    }
    fn match_diffs(
        diffs1: u32,
        diffs2: u32,
        diffs3: u32,
        right_tie_breaker: u32,
        left_tie_breaker: u32,
    ) -> (bool, bool, usize) {
        match (
            SAFE_RANGE.contains(&diffs1),
            SAFE_RANGE.contains(&diffs2),
            SAFE_RANGE.contains(&diffs3),
            SAFE_RANGE.contains(&right_tie_breaker),
            SAFE_RANGE.contains(&left_tie_breaker),
        ) {
            (true, true, true, _, _) => (true, false, 0),
            (true, true, false, true, _) => (true, true, 2),
            (true, true, false, false, _) => (true, true, 4),
            (false, true, true, _, true) => (true, true, 1),
            (false, true, true, _, false) => (true, true, 0),
            (true, false, true, _, _) => (true, true, 1),
            _ => (false, false, 0),
        }
    }
}

#[aoc(day2, part1, original)]
pub fn part1original(input: &str) -> u32 {
    input.lines().into_iter().fold(0, |safe_levels, line| {
        let mut spliter = line.split_whitespace();
        let mut old = spliter.next().unwrap().parse::<u32>().unwrap();
        let tmp = spliter.next().unwrap().parse::<u32>().unwrap();
        let increasing = if old < tmp { true } else { false };
        let diff = if increasing { tmp - old } else { old - tmp };
        if !SAFE_RANGE.contains(&diff) {
            return safe_levels;
        }
        old = tmp;
        for num_str in spliter {
            let current = num_str.parse::<u32>().unwrap();
            let diff = if increasing {
                current - old
            } else {
                old - current
            };
            if SAFE_RANGE.contains(&diff) {
                old = current
            } else {
                return safe_levels;
            }
        }
        safe_levels + 1
    })
}

#[aoc(day2, part1)]
pub fn part1(input: &[u8]) -> u32 {
    let mut start = 0;
    let mut old: u32 = 0;
    let mut next = 0;
    let mut increasing = false;
    (0..1000).fold(0, |safe_levels, _| {
        (old, next, start) = grab_two(input, start);
        increasing = if old < next { true } else { false };
        let diff = if increasing { next - old } else { old - next };

        if !SAFE_RANGE.contains(&diff) {
            start = next_line(input, start);
            return safe_levels;
        }
        old = next;

        while start < input.len() && input[start] != b'\n' {
            (next, start) = parse_digit(input, start + 1);

            if is_valid(old, next, increasing) {
                old = next;
            } else {
                start = next_line(input, start);
                return safe_levels;
            }
        }
        if start < input.len() {
            start += 1;
        }
        safe_levels + 1
    })
}
#[aoc(day2, part2)]
pub fn part2(input: &[u8]) -> u32 {
    let mut start = 0;
    let mut state = StateTracker::default();
    let mut next: u32 = 0;

    let mut valid = true;
    (0..1000).fold(0, |safe_lines, i| {
        (valid, start) = state.grab_four(input, start);
        if !valid {
            start = next_line(input, start);
            return safe_lines;
        }

        while start < input.len() && input[start] != b'\n' {
            (next, start) = parse_digit(input, start + 1);
            if !state.check_next(next) {
                start = next_line(input, start);
                return safe_lines;
            }
        }

        if start < input.len() {
            start += 1;
        }

        safe_lines + 1
    })
}
