use crate::*;
use crate::tokenizer::Token;
pub struct Parser {
    lexer: Lexer
}

impl Parser {
    pub fn new(lex: Lexer) -> Self {
        Parser {
            lexer: lex
        }
    }

    pub fn parse(&self) -> String {
        let mut ret = String::new();
        let mut counts: Vec<&Token> = Vec::new();
        let tokens = self.lexer.get_output();
        let mut match_has_count: bool = false;

        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::OPEN_MATCH => {
                    ret += "(?:";
                    if let Token::COUNT(_) = tokens[i-1] {
                        match_has_count = true;
                    }
                }
                Token::CLOSE_MATCH => {
                    ret += ")";
                    if match_has_count {
                        if let Token::COUNT(a) = counts[counts.len()-1] {
                            ret += &String::from(format!("{{{a}}}", a=(*a)));
                            counts.remove(counts.len()-1);
                        }
                        match_has_count = false;
                    }
                }
                Token::RANGE(a, b) => {
                    ret += &String::from(format!("[{a}-{b}]", a=(*a), b=(*b)));
                }
                Token::COUNT(_) => {
                    counts.push(token);
                }
                Token::IDENTIFIER(s) => {
                    let mut str = s.replace("^\"|\"$", "");
                    str = str.trim().to_string();
                    ret += &String::from(format!("({s})", s=str.replace("\"", "")));
                    if counts.len() != 0 {
                        if let Token::COUNT(a) = tokens[i-1] {
                            ret += &String::from(format!("{{{a}}}", a=a));
                            counts.remove(counts.len()-1);
                        }
                    }
                }
                _ => {

                }
            }
        }

        ret
    }
}
