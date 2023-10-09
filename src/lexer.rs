use std::process::exit;

use crate::prelude::*;

#[must_use]
pub fn lex_file(contents: String, filename: String) -> Vec<Token> {
    let mut comment_single = false;
    let mut comment_mul = false;
    let mut i: usize = 0;
    let len = contents.len();
    let mut tokens: Vec<Token> = Vec::new();
    let mut row: usize = 1;
    let mut col: usize = 1;
    while i < len {
        let ch = contents.chars().nth(i).unwrap();
        let next_char = if i + 1 < len {
            contents.chars().nth(i + 1).unwrap()
        } else {
            '\0'
        };
        let checker = ch.to_string() + next_char.to_string().as_str();
        col = i / row;

        if !comment_mul && !comment_single {
            match ch {
                '<' => {
                    tokens.push(Token {
                        token_type: TokenType::PointerLeft,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }
                '>' => {
                    tokens.push(Token {
                        token_type: TokenType::PointerRight,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }
                '&' => {
                    tokens.push(Token {
                        token_type: TokenType::PointerReset,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }
                '+' => {
                    tokens.push(Token {
                        token_type: TokenType::Add,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }
                '-' => {
                    tokens.push(Token {
                        token_type: TokenType::Sub,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }
                ',' => {
                    tokens.push(Token {
                        token_type: TokenType::ReadByte,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }
                '.' => {
                    tokens.push(Token {
                        token_type: TokenType::WriteByte,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }
                '\'' => {
                    tokens.push(Token {
                        token_type: TokenType::Clear,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }
                '%' => {
                    tokens.push(Token {
                        token_type: TokenType::BaseMemAddr,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }
                '$' => {
                    tokens.push(Token {
                        token_type: TokenType::MemAddr,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }
                '[' => {
                    tokens.push(Token {
                        token_type: TokenType::ZeroJump,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }
                ']' => {
                    tokens.push(Token {
                        token_type: TokenType::NonZeroJump,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }
                '?' => {
                    tokens.push(Token {
                        token_type: TokenType::Syscall,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }
                '\n' => {
                    tokens.push(Token {
                        token_type: TokenType::NewLine,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                    row += 1;
                }

                '\0' => {
                    tokens.push(Token {
                        token_type: TokenType::NewLine,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }

                '^' => {
                    tokens.push(Token {
                        token_type: TokenType::Push,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }

                '_' => {
                    tokens.push(Token {
                        token_type: TokenType::Pop,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }

                '!' => {
                    tokens.push(Token {
                        token_type: TokenType::StackDel,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }

                '@' => {
                    tokens.push(Token {
                        token_type: TokenType::CurrentTape,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }
                ';' => {
                    tokens.push(Token {
                        token_type: TokenType::BitwiseLeft,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }
                ':' => {
                    tokens.push(Token {
                        token_type: TokenType::BitwiseRight,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }
                '|' => {
                    tokens.push(Token {
                        token_type: TokenType::BitwiseOr,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }
                '\\' => {
                    tokens.push(Token {
                        token_type: TokenType::BitwiseAnd,
                        value: ch.to_string(),
                        row,
                        col,
                        filename: filename.clone(),
                    });
                }

                _ => match checker.as_str() {
                    "//" => {
                        // println!("[Single Line Comment: ");
                        comment_single = true;
                        i += 2;
                        continue;
                    }
                    "/*" => {
                        // println!("[Multi Line Comment: ");
                        comment_mul = true;
                        i += 2;
                        continue;
                    }
                    _ => {
                        // println!("unexpected token: {}", ch);
                        if !ch.is_whitespace() {
                            match ch {
                                '\"' => {
                                    let mut word: String = String::new();

                                    i += 1;
                                    // let nexty_ch = contents.chars().nth(i);
                                    while contents.chars().nth(i).unwrap() != '\"' {
                                        // print!("{}", contents.chars().nth(i).unwrap());
                                        // println!("{}",contents.chars().nth(i).unwrap());
                                        word +=
                                            contents.chars().nth(i).unwrap().to_string().as_str();
                                        i += 1;
                                        if contents.chars().nth(i).is_none() {
                                            break;
                                        }
                                    }

                                    let mut new_str: String = String::new();
                                    // let len: usize = word.len() - 1;
                                    for n in 0..word.len() {
                                        let ch = word.chars().nth(n).unwrap().to_string();
                                        new_str += ch.as_str();
                                    }
                                    tokens.push(Token {
                                        token_type: TokenType::StringLit,
                                        value: new_str.replace("\\n", "\n").replace("\\0", "\0"),
                                        row,
                                        col,
                                        filename: filename.clone(),
                                    });
                                }

                                '(' => {
                                    let mut word: String = String::new();

                                    i += 1;
                                    // let nexty_ch = contents.chars().nth(i);
                                    while contents.chars().nth(i).unwrap() != ')' {
                                        // print!("{}", contents.chars().nth(i).unwrap());
                                        // println!("{}",contents.chars().nth(i).unwrap());
                                        word +=
                                            contents.chars().nth(i).unwrap().to_string().as_str();
                                        i += 1;
                                        if contents.chars().nth(i).is_none() {
                                            break;
                                        }
                                    }

                                    let mut new_str: String = String::new();
                                    // let len: usize = word.len() - 1;
                                    for n in 0..word.len() {
                                        let ch = word.chars().nth(n).unwrap().to_string();
                                        new_str += ch.as_str();
                                    }
                                    tokens.push(Token {
                                        token_type: TokenType::IncludePath,
                                        value: new_str,
                                        row,
                                        col,
                                        filename: filename.clone(),
                                    });
                                }

                                '{' => {
                                    let mut word: String = String::new();

                                    i += 1;
                                    // let nexty_ch = contents.chars().nth(i);
                                    while contents.chars().nth(i).unwrap() != '}' {
                                        // print!("{}", contents.chars().nth(i).unwrap());
                                        // println!("{}",contents.chars().nth(i).unwrap());
                                        word +=
                                            contents.chars().nth(i).unwrap().to_string().as_str();
                                        i += 1;
                                        if contents.chars().nth(i).is_none() {
                                            break;
                                        }
                                    }

                                    let mut new_str: String = String::new();
                                    // let len: usize = word.len() - 1;
                                    for n in 0..word.len() {
                                        let ch = word.chars().nth(n).unwrap().to_string();
                                        new_str += ch.as_str();
                                    }
                                    tokens.push(Token {
                                        token_type: TokenType::TapeName,
                                        value: new_str,
                                        row,
                                        col,
                                        filename: filename.clone(),
                                    });
                                }

                                // '`' => {
                                //     let mut word: String = String::new();

                                //     i += 1;
                                //     // let nexty_ch = contents.chars().nth(i);
                                //     while contents.chars().nth(i).unwrap() != '`' {
                                //         // print!("{}", contents.chars().nth(i).unwrap());
                                //         // println!("{}",contents.chars().nth(i).unwrap());
                                //         word +=
                                //             contents.chars().nth(i).unwrap().to_string().as_str();
                                //         i += 1;
                                //         if contents.chars().nth(i).is_none() {
                                //             break;
                                //         }
                                //     }

                                //     let mut is_number = true;
                                //     for ch in word.chars() {
                                //         if !ch.is_ascii_digit() {
                                //             is_number = false;
                                //             break;
                                //         }
                                //     }

                                //     if is_number {
                                //         tokens.push(Token {
                                //             token_type: TokenType::IntLit,
                                //             value: word,
                                //             row,
                                //             col,
                                //             filename: filename.clone(),
                                //         });
                                //     } else {
                                //         println!(
                                //             "{}:{}:{} Expected int literal",
                                //             filename, row, col
                                //         );
                                //         exit(1);
                                //     }
                                // }

                                _ => {
                                    // println!("Unexpected token: {}", ch);
                                    // println!("Idents and Macros not implemented yet");
                                    let mut word: String = String::new();
                                    // let nexty_ch = contents.chars().nth(i);

                                    while 
                                    // !contents.chars().nth(i).unwrap().is_whitespace() 
                                    contents.chars().nth(i).unwrap().is_ascii_graphic() 
                                    // && contents.chars().nth(i).unwrap() != '`' &&
                                    // contents.chars().nth(i).unwrap() != '\n'
                                    {
                                        // print!("{}", contents.chars().nth(i).unwrap());
                                        // println!("{}",contents.chars().nth(i).unwrap());
                                        word +=
                                            contents.chars().nth(i).unwrap().to_string().as_str();
                                        i += 1;
                                        if contents.chars().nth(i).is_none() {
                                            break;
                                        }
                                    }
                                    // println!("");

                                    if word.is_empty() {
                                        println!(
                                            "{}:{}:{} Something fucked up in lexer",
                                            filename,
                                            row,
                                            col + 1
                                        );
                                        exit(1);
                                    }

                                    // println!("Word: \"{}\"", word);
                                    match word.to_lowercase().as_str() {
                                        "#define" => {
                                            tokens.push(Token {
                                                token_type: TokenType::MacroDecl,
                                                value: word,
                                                row,
                                                col,
                                                filename: filename.clone(),
                                            });
                                            continue;
                                        }

                                        "#ifdef" => {
                                            tokens.push(Token {
                                                token_type: TokenType::IfdefMacro,
                                                value: word,
                                                row,
                                                col,
                                                filename: filename.clone(),
                                            });
                                            continue;
                                        }

                                        "#ifndef" => {
                                            tokens.push(Token {
                                                token_type: TokenType::IfNdefMacro,
                                                value: word,
                                                row,
                                                col,
                                                filename: filename.clone(),
                                            });
                                            continue;
                                        }

                                        "#else" => {
                                            tokens.push(Token {
                                                token_type: TokenType::ElseMacro,
                                                value: word,
                                                row,
                                                col,
                                                filename: filename.clone(),
                                            });
                                            continue;
                                        }

                                        "#endif" => {
                                            tokens.push(Token {
                                                token_type: TokenType::EndifMacro,
                                                value: word,
                                                row,
                                                col,
                                                filename: filename.clone(),
                                            });
                                            continue;
                                        }

                                        "#include" => {
                                            tokens.push(Token {
                                                token_type: TokenType::IncludeMacro,
                                                value: word,
                                                row,
                                                col,
                                                filename: filename.clone(),
                                            });
                                            continue;
                                        }

                                        "#tape" => {
                                            tokens.push(Token {
                                                token_type: TokenType::TapeDecl,
                                                value: word,
                                                row,
                                                col,
                                                filename: filename.clone(),
                                            });
                                            continue;
                                        }

                                        "byte" => {
                                            tokens.push(Token {
                                                token_type: TokenType::CellSize,
                                                value: word,
                                                row,
                                                col,
                                                filename: filename.clone(),
                                            });
                                            continue;
                                        }

                                        "word" => {
                                            tokens.push(Token {
                                                token_type: TokenType::CellSize,
                                                value: word,
                                                row,
                                                col,
                                                filename: filename.clone(),
                                            });
                                            continue;
                                        }

                                        "dword" => {
                                            tokens.push(Token {
                                                token_type: TokenType::CellSize,
                                                value: word,
                                                row,
                                                col,
                                                filename: filename.clone(),
                                            });
                                            continue;
                                        }

                                        "qword" => {
                                            tokens.push(Token {
                                                token_type: TokenType::CellSize,
                                                value: word,
                                                row,
                                                col,
                                                filename: filename.clone(),
                                            });
                                            continue;
                                        }

                                        _ => {
                                            // dbg!(&word);
                                            // exit(1);

                                            let mut is_number = true;
                                            for ch in word.chars() {
                                                if !ch.is_ascii_digit() {
                                                    is_number = false;
                                                    break;
                                                }
                                            }

                                            if is_number {
                                                tokens.push(Token {
                                                    token_type: TokenType::IntLit,
                                                    value: word,
                                                    row,
                                                    col,
                                                    filename: filename.clone(),
                                                });
                                            } else {
                                                tokens.push(Token {
                                                    token_type: TokenType::Ident,
                                                    value: word,
                                                    row,
                                                    col,
                                                    filename: filename.clone(),
                                                });
                                            }
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
            }
            i += 1;
        } else {
            // print!("{}", ch);

            if comment_mul && (checker.as_str() == "\0" || checker.as_str() == "\0") {
                row += 1;
            }

            if comment_mul && checker.as_str() == "*/" {
                comment_mul = false;
                comment_single = false;
                i += 2;
                tokens.push(Token {
                    token_type: TokenType::NewLine,
                    value: "\n".to_string(),
                    row,
                    col,
                    filename: filename.clone(),
                });
                // println!("\nEnd of multi line comment]");
                continue;
            }

            if comment_single && (ch == '\n' || ch == '\0') {
                comment_mul = false;
                comment_single = false;
                tokens.push(Token {
                    token_type: TokenType::NewLine,
                    value: "\n".to_string(),
                    row,
                    col,
                    filename: filename.clone(),
                });
                row += 1;
                // println!("\nEnd of single line comment]");
            }
            i += 1;
        }
    }
    tokens.push(Token {
        token_type: TokenType::NewLine,
        value: "\n".to_string(),
        row,
        col,
        filename: filename.clone(),
    });
    tokens
}
