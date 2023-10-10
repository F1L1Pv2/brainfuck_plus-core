use std::process::exit;

use crate::prelude::*;

pub fn lex_token(
    i: &mut usize,
    contents: String,
    row: &mut usize,
    col: usize,
    filename: String,
    comment_single: &mut bool,
    comment_mul: &mut bool
) -> Vec<Token> {
    let len = contents.len();
    let mut tokens: Vec<Token> = Vec::new();
    let ch = contents.chars().nth(*i).unwrap();
        let next_char = if *i + 1 < len {
            contents.chars().nth(*i + 1).unwrap()
        } else {
            '\0'
        };
        let checker = ch.to_string() + next_char.to_string().as_str();
        
    match ch {
        '<' => {
            tokens.push(Token {
                token_type: TokenType::PointerLeft,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }
        '>' => {
            tokens.push(Token {
                token_type: TokenType::PointerRight,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }
        '&' => {
            tokens.push(Token {
                token_type: TokenType::PointerReset,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }
        '+' => {
            tokens.push(Token {
                token_type: TokenType::Add,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }
        '-' => {
            tokens.push(Token {
                token_type: TokenType::Sub,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }
        ',' => {
            tokens.push(Token {
                token_type: TokenType::ReadByte,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }
        '.' => {
            tokens.push(Token {
                token_type: TokenType::WriteByte,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }
        '\'' => {
            tokens.push(Token {
                token_type: TokenType::Clear,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }
        '%' => {
            tokens.push(Token {
                token_type: TokenType::BaseMemAddr,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }
        '$' => {
            tokens.push(Token {
                token_type: TokenType::MemAddr,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }
        '[' => {
            tokens.push(Token {
                token_type: TokenType::ZeroJump,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }
        ']' => {
            tokens.push(Token {
                token_type: TokenType::NonZeroJump,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }
        '?' => {
            tokens.push(Token {
                token_type: TokenType::Syscall,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }
        '\n' => {
            tokens.push(Token {
                token_type: TokenType::NewLine,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
            *row += 1;
        }

        '\0' => {
            tokens.push(Token {
                token_type: TokenType::NewLine,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }

        '^' => {
            tokens.push(Token {
                token_type: TokenType::Push,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }

        '_' => {
            tokens.push(Token {
                token_type: TokenType::Pop,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }

        '!' => {
            tokens.push(Token {
                token_type: TokenType::StackDel,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }

        '@' => {
            tokens.push(Token {
                token_type: TokenType::CurrentTape,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }
        ';' => {
            tokens.push(Token {
                token_type: TokenType::BitwiseLeft,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }
        ':' => {
            tokens.push(Token {
                token_type: TokenType::BitwiseRight,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }
        '|' => {
            tokens.push(Token {
                token_type: TokenType::BitwiseOr,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }
        '\\' => {
            tokens.push(Token {
                token_type: TokenType::BitwiseAnd,
                value: ch.to_string(),
                row: *row,
                col,
                filename: filename.clone(),
            });
        }

        _ => match checker.as_str() {
            "//" => {
                // println!("[Single Line Comment: ");
                *comment_single = true;
                *i += 2;
                return Vec::new();
            }
            "/*" => {
                // println!("[Multi Line Comment: ");
                *comment_mul = true;
                *i += 2;
                return Vec::new();
            }
            _ => {
                // println!("unexpected token: {}", ch);
                if !ch.is_whitespace() {
                    match ch {
                        '\"' => {
                            let mut word: String = String::new();

                            *i += 1;
                            // let nexty_ch = contents.chars().nth(i);
                            while contents.chars().nth(*i).unwrap() != '\"' {
                                // print!("{}", contents.chars().nth(i).unwrap());
                                // println!("{}",contents.chars().nth(i).unwrap());
                                word += contents.chars().nth(*i).unwrap().to_string().as_str();
                                *i += 1;
                                if contents.chars().nth(*i).is_none() {
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
                                row: *row,
                                col,
                                filename: filename.clone(),
                            });
                        }

                        '(' => {
                            let mut word: String = String::new();

                            *i += 1;
                            // let nexty_ch = contents.chars().nth(i);
                            while contents.chars().nth(*i).unwrap() != ')' {
                                // print!("{}", contents.chars().nth(i).unwrap());
                                // println!("{}",contents.chars().nth(i).unwrap());
                                word += contents.chars().nth(*i).unwrap().to_string().as_str();
                                *i += 1;
                                if contents.chars().nth(*i).is_none() {
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
                                row: *row,
                                col,
                                filename: filename.clone(),
                            });
                        }

                        '{' => {
                            let mut word: String = String::new();

                            *i += 1;
                            // let nexty_ch = contents.chars().nth(i);
                            while contents.chars().nth(*i).unwrap() != '}' {
                                // print!("{}", contents.chars().nth(i).unwrap());
                                // println!("{}",contents.chars().nth(i).unwrap());
                                word += contents.chars().nth(*i).unwrap().to_string().as_str();
                                *i += 1;
                                if contents.chars().nth(*i).is_none() {
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
                                row: *row,
                                col,
                                filename: filename.clone(),
                            });
                        }
                        _ => {
                            // println!("Unexpected token: {}", ch);
                            // println!("Idents and Macros not implemented yet");
                            let mut word: String = String::new();
                            // let nexty_ch = contents.chars().nth(i);

                            while
                            // !contents.chars().nth(i).unwrap().is_whitespace()
                            contents.chars().nth(*i).unwrap().is_ascii_graphic()
                            && contents.chars().nth(*i).unwrap() != '`' 
                            // && contents.chars().nth(*i).unwrap() != '\n'
                            {
                                // print!("{}", contents.chars().nth(i).unwrap());
                                // println!("{}",contents.chars().nth(i).unwrap());
                                word += contents.chars().nth(*i).unwrap().to_string().as_str();
                                *i += 1;
                                if contents.chars().nth(*i).is_none() {
                                    break;
                                }
                            }
                            // println!("");

                            if word.is_empty() {
                                println!(
                                    "{}:{}:{} Something fucked up in lexer",
                                    filename,
                                    *row,
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
                                        row: *row,
                                        col,
                                        filename: filename.clone(),
                                    });
                                    return tokens;
                                }

                                "#ifdef" => {
                                    tokens.push(Token {
                                        token_type: TokenType::IfdefMacro,
                                        value: word,
                                        row: *row,
                                        col,
                                        filename: filename.clone(),
                                    });
                                    return tokens;
                                }

                                "#ifndef" => {
                                    tokens.push(Token {
                                        token_type: TokenType::IfNdefMacro,
                                        value: word,
                                        row: *row,
                                        col,
                                        filename: filename.clone(),
                                    });
                                    return tokens;
                                }

                                "#else" => {
                                    tokens.push(Token {
                                        token_type: TokenType::ElseMacro,
                                        value: word,
                                        row: *row,
                                        col,
                                        filename: filename.clone(),
                                    });
                                    return tokens;
                                }

                                "#endif" => {
                                    tokens.push(Token {
                                        token_type: TokenType::EndifMacro,
                                        value: word,
                                        row: *row,
                                        col,
                                        filename: filename.clone(),
                                    });
                                    return tokens;
                                }

                                "#include" => {
                                    tokens.push(Token {
                                        token_type: TokenType::IncludeMacro,
                                        value: word,
                                        row: *row,
                                        col,
                                        filename: filename.clone(),
                                    });
                                    return tokens;
                                }

                                "#tape" => {
                                    tokens.push(Token {
                                        token_type: TokenType::TapeDecl,
                                        value: word,
                                        row: *row,
                                        col,
                                        filename: filename.clone(),
                                    });
                                    return tokens;
                                }

                                "byte" => {
                                    tokens.push(Token {
                                        token_type: TokenType::CellSize,
                                        value: word,
                                        row: *row,
                                        col,
                                        filename: filename.clone(),
                                    });
                                    return tokens;
                                }

                                "word" => {
                                    tokens.push(Token {
                                        token_type: TokenType::CellSize,
                                        value: word,
                                        row: *row,
                                        col,
                                        filename: filename.clone(),
                                    });
                                    return tokens;
                                }

                                "dword" => {
                                    tokens.push(Token {
                                        token_type: TokenType::CellSize,
                                        value: word,
                                        row: *row,
                                        col,
                                        filename: filename.clone(),
                                    });
                                    return tokens;
                                }

                                "qword" => {
                                    tokens.push(Token {
                                        token_type: TokenType::CellSize,
                                        value: word,
                                        row: *row,
                                        col,
                                        filename: filename.clone(),
                                    });
                                    return tokens;
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
                                        // dbg!(&word);
                                        if contents.chars().nth(*i).unwrap() == '`'{
                                            *i+=1;
                                            let iter_tokens = lex_token(i, contents.clone(), row, col, filename, comment_single, comment_mul);
                                            // dbg!(contents.chars().nth(*i));
                                            // exit(1);

                                            // println!("{}",word.parse::<usize>().unwrap());

                                            for _ in 0..word.parse::<usize>().unwrap(){
                                                tokens.append(&mut iter_tokens.clone());
                                            }
                                        }else{
                                            tokens.push(Token {
                                                token_type: TokenType::IntLit,
                                                value: word.clone(),
                                                row: *row,
                                                col,
                                                filename: filename.clone(),
                                            });
                                        }

                                    } else {
                                        tokens.push(Token {
                                            token_type: TokenType::Ident,
                                            value: word,
                                            row: *row,
                                            col,
                                            filename: filename.clone(),
                                        });
                                    }
                                    return tokens;
                                }
                            }
                        }
                    }
                }
            }
        },
    }

    *i += 1;
    tokens
}

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
            tokens.append(&mut lex_token(&mut i, contents.clone(), &mut row, col, filename.clone(), &mut comment_single, &mut comment_mul));
            // i += 1;
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
