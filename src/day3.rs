use aoc_runner_derive::aoc;
use std::ops::Range;

const MUL_BYTES: [u8; 3] = [b'u', b'l', b'('];
const DONT_BYTES: [u8; 6] = [b'o', b'n', b'\'', b't', b'(', b')'];
const DO_BYTES: [u8; 3] = [b'o', b'(', b')'];
const ASCII_DIGITS: Range<u8> = 48..58;

enum ParserState {
    Seeking,
    FirstInput,
    SecondInput,
}

enum TogglableParserState {
    Seeking,
    FirstInput,
    SecondInput,
    Disabled,
}

fn get_muls(input: &[u8]) -> Vec<(u32, u32)> {
    let mut muls = Vec::with_capacity(1000);
    let mut start = 0;
    let mut state = ParserState::Seeking;
    let mut first = 0;
    let mut second = 0;
    'outer: while start < input.len() {
        match state {
            ParserState::Seeking => {
                if input[start] == b'm' {
                    if start + 7 < input.len() && &input[start + 1..start + 4] == &MUL_BYTES {
                        state = ParserState::FirstInput;
                        start += 4;
                    } else {
                        start += 1;
                    }
                } else {
                    start += 1;
                }
            }
            ParserState::FirstInput => {
                let mut offset = start;

                while offset < input.len() {
                    match input[offset] {
                        b',' => {
                            first = get_digit(&input[start..offset]);
                            start = offset + 1;
                            state = ParserState::SecondInput;
                            continue 'outer;
                        }
                        c if ASCII_DIGITS.contains(&c) => {
                            offset += 1;
                        }
                        _ => {
                            start = offset;
                            state = ParserState::Seeking;
                            continue 'outer;
                        }
                    }
                }
            }
            ParserState::SecondInput => {
                let mut offset = start;

                while offset < input.len() {
                    match input[offset] {
                        b')' => {
                            second = get_digit(&input[start..offset]);
                            muls.push((first, second));
                            start = offset + 1;
                            state = ParserState::Seeking;
                            continue 'outer;
                        }
                        c if ASCII_DIGITS.contains(&c) => {
                            offset += 1;
                        }
                        _ => {
                            start = offset;
                            state = ParserState::Seeking;
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }
    muls
}

fn get_muls_toggle(input: &[u8]) -> Vec<(u32, u32)> {
    let mut muls = Vec::with_capacity(1000);
    let mut start = 0;
    let mut state = TogglableParserState::Seeking;
    let mut first = 0;
    let mut second = 0;
    'outer: while start < input.len() {
        match state {
            TogglableParserState::Seeking => {
                if input[start] == b'm' {
                    if start + 7 < input.len() && &input[start + 1..start + 4] == &MUL_BYTES {
                        state = TogglableParserState::FirstInput;
                        start += 4;
                    } else {
                        start += 1;
                    }
                } else if input[start] == b'd' {
                    if start + 6 < input.len() && &input[start + 1..start + 7] == &DONT_BYTES {
                        state = TogglableParserState::Disabled;
                        start += 6;
                    } else {
                        start += 1;
                    }
                } else {
                    start += 1;
                }
            }
            TogglableParserState::FirstInput => {
                let mut offset = start;

                while offset < input.len() {
                    match input[offset] {
                        b',' => {
                            first = get_digit(&input[start..offset]);
                            start = offset + 1;
                            state = TogglableParserState::SecondInput;
                            continue 'outer;
                        }
                        c if ASCII_DIGITS.contains(&c) => {
                            offset += 1;
                        }
                        _ => {
                            start = offset;
                            state = TogglableParserState::Seeking;
                            continue 'outer;
                        }
                    }
                }
            }
            TogglableParserState::SecondInput => {
                let mut offset = start;

                while offset < input.len() {
                    match input[offset] {
                        b')' => {
                            second = get_digit(&input[start..offset]);
                            muls.push((first, second));
                            start = offset + 1;
                            state = TogglableParserState::Seeking;
                            continue 'outer;
                        }
                        c if ASCII_DIGITS.contains(&c) => {
                            offset += 1;
                        }
                        _ => {
                            start = offset;
                            state = TogglableParserState::Seeking;
                            continue 'outer;
                        }
                    }
                }
            }
            TogglableParserState::Disabled => {
                if input[start] == b'd' {
                    if start + 3 < input.len() && &input[start + 1..start + 4] == &DO_BYTES {
                        state = TogglableParserState::Seeking;
                        start += 3;
                        continue 'outer;
                    }
                }
                start += 1;
            }
        }
    }
    muls
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let muls = get_muls(input);
    let mut sum = 0;
    for (a, b) in muls {
        sum += a * b;
    }
    sum
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    let muls = get_muls_toggle(input);
    let mut sum = 0;
    for (a, b) in muls {
        sum += a * b;
    }
    sum
}

fn get_digit(offset: &[u8]) -> u32 {
    unsafe {
        match offset.len() {
            1 => *offset.get_unchecked(0) as u32 - 48,
            2 => (*offset.get_unchecked(0) as u32) * 10 + (*offset.get_unchecked(1) as u32) - 528,
            3 => {
                (*offset.get_unchecked(0) as u32 * 100)
                    + (*offset.get_unchecked(1) as u32 * 10)
                    + *offset.get_unchecked(2) as u32
                    - 5328
            }
            _ => panic!("Unexpected number"),
        }
    }
}
