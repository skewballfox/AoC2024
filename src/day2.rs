use aoc_runner_derive::aoc;
const SAFE_RANGE: std::ops::Range<u32> = 1..4;
const INPUT_LEN: usize = 1000;
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
        (*input.get_unchecked(start) as u32) * 10 + (*input.get_unchecked(start + 1) as u32) - 528
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

#[inline(always)]
fn grab_two(input: &[u8], start: usize) -> (u32, u32, usize) {
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
        );

        let mut replace_idx = 4;

        if increasing {
            self.increasing = true;
            if replace_level {
                self.free_level = false;
                if index == 0 {
                    if SAFE_RANGE.contains(&(self.data[2] - self.data[0])) {
                        replace_idx = 1;
                    } else {
                        replace_idx = 0;
                    }
                } else {
                    replace_idx = index;
                }
            }
        } else {
            let (decreasing, replace_level, index) = Self::match_diffs(
                self.data[0] - self.data[1],
                self.data[1] - self.data[2],
                self.data[2] - fourth,
                self.data[1] - fourth,
            );

            if decreasing {
                self.increasing = false;
                if replace_level {
                    self.free_level = false;
                    if index == 0 {
                        if SAFE_RANGE.contains(&(self.data[0] - self.data[2])) {
                            replace_idx = 1;
                        } else {
                            replace_idx = 0;
                        }
                    } else {
                        replace_idx = index;
                    }
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
    ) -> (bool, bool, usize) {
        match (
            SAFE_RANGE.contains(&diffs1),
            SAFE_RANGE.contains(&diffs2),
            SAFE_RANGE.contains(&diffs3),
            SAFE_RANGE.contains(&right_tie_breaker),
        ) {
            (true, true, true, _) => (true, false, 0),
            (true, true, false, _) => (true, true, 3),
            (false, true, true, _) => (true, true, 0),
            (true, false, true, _) => (true, true, 1),

            (true, false, false, true) => (true, true, 2),
            _ => (false, false, 0),
        }
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut start = 0;
    let mut old: u32;
    let mut next: u32;
    let mut increasing;
    let mut safe_levels = 0;
    'outer: for _ in 0..INPUT_LEN {
        (old, next, start) = grab_two(input, start);
        increasing = old < next;
        let diff = if increasing { next - old } else { old - next };

        if !SAFE_RANGE.contains(&diff) {
            start = next_line(input, start);
            continue;
        }
        old = next;

        while start < input.len() && input[start] != b'\n' {
            (next, start) = parse_digit(input, start + 1);

            if is_valid(old, next, increasing) {
                old = next;
            } else {
                start = next_line(input, start);
                continue 'outer;
            }
        }
        if start < input.len() {
            start += 1;
        }
        safe_levels += 1
    }
    safe_levels
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut start = 0;
    let mut state = StateTracker::default();
    let mut next: u32;

    let mut valid: bool;
    let mut safe_levels = 0;
    'outer: for _ in 0..INPUT_LEN {
        (valid, start) = state.grab_four(input, start);
        if !valid {
            start = next_line(input, start);
            continue;
        }

        while start < input.len() && input[start] != b'\n' {
            (next, start) = parse_digit(input, start + 1);
            if !state.check_next(next) {
                start = next_line(input, start);
                continue 'outer;
            }
        }

        if start < input.len() {
            start += 1;
        }

        safe_levels += 1
    }
    safe_levels
}
