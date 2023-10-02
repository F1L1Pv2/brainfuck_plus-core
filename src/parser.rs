use std::process::exit;

use crate::prelude::*;

fn trim_tokens(i: &mut usize, tokens: &Vec<Token>, token_type: TokenType) -> Operation {
    let mut count = 0;

    let len: usize = tokens.len();

    let mut values: Vec<String> = Vec::new();

    let row = tokens[*i].row;
    let col = tokens[*i].col;
    let filename: String = tokens[*i].filename.clone();

    while tokens[*i].token_type == token_type {
        count += 1;
        *i += 1;
        values.push(tokens[*i].value.clone());
        if *i >= len {
            break;
        }
    }

    Operation {
        token_type,
        count,
        values,
        tape: None,
        row,
        col,
        filename,
    }
}

#[must_use]
pub fn parse_file(tokens: Vec<Token>, tapes: &[Tape]) -> Vec<Operation> {
    let mut operations: Vec<Operation> = Vec::new();

    let mut i: usize = 0;
    let len: usize = tokens.len();

    while i < len {
        let token = tokens[i].clone();

        match token.token_type {
            TokenType::Add => operations.push(trim_tokens(&mut i, &tokens, token.token_type)),

            TokenType::Sub => operations.push(trim_tokens(&mut i, &tokens, token.token_type)),

            TokenType::PointerLeft => {
                operations.push(trim_tokens(&mut i, &tokens, token.token_type))
            }
            TokenType::BitwiseLeft => {
                operations.push(trim_tokens(&mut i, &tokens, token.token_type))
            }
            TokenType::BitwiseRight => {
                operations.push(trim_tokens(&mut i, &tokens, token.token_type))
            }

            TokenType::PointerRight => {
                operations.push(trim_tokens(&mut i, &tokens, token.token_type))
            }

            TokenType::Push => operations.push(trim_tokens(&mut i, &tokens, token.token_type)),

            TokenType::Pop => operations.push(trim_tokens(&mut i, &tokens, token.token_type)),

            TokenType::CurrentTape => {
                i += 1;
                let tape_name = tokens[i].clone();
                if tape_name.token_type != TokenType::TapeName {
                    println!(
                        "{}:{}:{} CurrentTape: Expected tape name after operation got {}",
                        token.filename,
                        token.row,
                        token.col + 1,
                        tape_name.value
                    );
                    exit(1);
                }

                let tape = {
                    let mut tape: Option<Tape> = None;
                    for tapem in tapes {
                        if tapem.name == tape_name.value {
                            tape = Some(tapem.clone());
                            break;
                        }
                    }

                    tape
                };

                if tape.is_none() {
                    println!(
                        "{}:{}:{} Tape {} not defined",
                        token.filename,
                        token.col,
                        token.row + 1,
                        tape_name.value
                    );
                    exit(1);
                }

                operations.push(Operation {
                    token_type: token.token_type,
                    count: 1,
                    values: vec![token.value],
                    tape,
                    row: token.row,
                    col: token.col,
                    filename: token.filename,
                });
                i += 1;
            }

            _ => {
                operations.push(Operation {
                    token_type: token.token_type,
                    count: 1,
                    values: vec![token.value],
                    tape: None,
                    row: token.row,
                    col: token.col,
                    filename: token.filename,
                });
                i += 1;
            }
        }
    }

    operations
}
