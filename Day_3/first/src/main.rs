use regex::Regex;
use std::fs::read_to_string;

const EMPTY: char = '.';

#[derive(Debug)]
struct Token {
    pos: usize,
    end_pos: usize,
}

#[derive(Debug)]
enum Tokens {
    Empty(Token),
    Number(Token, u32),
    Symbol(Token, char),
}

//todo!("Write my own lexer in the future");
fn tokenize(text: &str, tokenizer_exp: &regex::Regex) -> Vec<Tokens> {
    tokenizer_exp
        .find_iter(text)
        .map(|mat_str| {
            let first_char = mat_str.as_str().chars().next().unwrap();
            match first_char {
                '.' => Tokens::Empty(Token {
                    pos: mat_str.start(),
                    end_pos: mat_str.end(),
                }),
                '0'..='9' => Tokens::Number(
                    Token {
                        pos: mat_str.start(),
                        end_pos: mat_str.end(),
                    },
                    mat_str.as_str().parse::<u32>().unwrap(),
                ),
                _ => Tokens::Symbol(
                    Token {
                        pos: mat_str.start(),
                        end_pos: mat_str.end(),
                    },
                    first_char,
                ),
            }
        })
        .collect::<Vec<Tokens>>()
}

fn has_adjacent_symbol(list: &Vec<Tokens>, number: &Token) -> bool {
    for token in list {
        match token {
            // Tokens::Empty(t) => {
            //     if t.pos > number.end_pos + 1 {
            //         return false;
            //     }
            // }
            // Tokens::Number(t, _) => {
            //     if t.pos > number.end_pos + 1 {
            //         return false;
            //     }
            // }
            Tokens::Symbol(t, c) => {
                println!("Checking symbol: {}", c);
                if (t.pos + 1 >= number.pos) && (t.pos <= number.end_pos) {
                    return true;
                } else if t.pos > number.end_pos {
                    println!("{} > {}; return", t.pos, number.end_pos);
                    return false;
                } else {
                    println!("pos: {} not within range of {}-{}", t.pos, number.pos, number.end_pos)
                }
            },
            _ => {}
        }
    }
    false
}

fn main() {
    let data = read_to_string("data/input.txt").unwrap();
    let test_x = 6;

    assert!(test_x >= 6 && test_x <= 9);

    let re = Regex::new(r"(\.+|[0-9]+|.)").unwrap();

    let mut list_of_tokens: Vec<Vec<Tokens>> = data.lines().map(|line| tokenize(line, &re)).collect();
    // Insert empty element at start and end of the vector so we can simplify running a window over the list
    list_of_tokens.insert(0, vec![]);
    list_of_tokens.push(vec![]);


    let mut accumulator = 0;

    list_of_tokens.windows(3)
                  .inspect(|w| println!("prev: {:?}\ncur: {:?}\nnext: {:?}", w[0], w[1], w[2]))
                  .map(|w| {
                    let prev = &w[0];
                    let cur = &w[1];
                    let next = &w[2];

                    for token in cur {
                        match token {
                            Tokens::Number(t, val) => { 
                                print!("Number: {val} ");
                                if has_adjacent_symbol(prev, t) || has_adjacent_symbol(cur, t) || has_adjacent_symbol(next, t) {
                                    accumulator += val;
                                    println!(" has adjacent symbol")
                                } else {
                                    println!("doesn't have adjacent symbol")
                                }
                            },
                            _ => {}
                        }
                    }
                    println!("");
                  })
                  .for_each(drop); //   .collect::<Vec<_>>();

    println!("Total accumulated value: {accumulator}");
}
